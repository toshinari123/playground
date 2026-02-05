use crate::{
    component::prelude::*,
    frame::{Token, TokensExt},
    message::{handle_messages, send},
    prelude::{DisplayList, Element, Frame, FrameExt, Size},
};
use std::{
    io::{self, Write},
    thread,
    time::{Duration, Instant},
};

use crossterm::{
    ExecutableCommand, QueueableCommand,
    cursor::{Hide, MoveTo, Show},
    event::{self, Event, KeyCode, KeyEvent, KeyModifiers},
    terminal::{
        Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode,
        enable_raw_mode,
    },
};
use tokio::sync::mpsc::{UnboundedSender, unbounded_channel};

pub mod prelude {
    pub use super::{Tick, render};
}

pub struct Tick(pub Duration);

fn print_frame(frame: Frame) -> std::io::Result<()> {
    let mut stdout = io::stdout();
    stdout.queue(Clear(ClearType::All))?;
    for row_index in 0..frame.height() {
        if row_index >= u16::MAX as usize {
            break;
        }
        stdout.queue(MoveTo(0, row_index as u16))?;
        print!("{}", frame[row_index].to_string());
    }
    stdout.flush()?;
    Ok(())
}

fn setup() -> (
    UnboundedSender<Box<dyn Element>>,
    std::thread::JoinHandle<std::io::Result<()>>,
) {
    let (sender, mut receiver) = unbounded_channel::<Box<dyn Element>>();
    let rendering_task = thread::spawn(move || -> std::io::Result<()> {
        let mut stdout = io::stdout();
        enable_raw_mode()?;
        stdout.execute(EnterAlternateScreen)?;
        stdout.execute(Hide)?;
        while let Some(element) = receiver.blocking_recv() {
            let (cols, rows) = crossterm::terminal::size()?;
            let mut display_list = DisplayList::default();
            element.draw(
                Size {
                    x: cols as isize,
                    y: rows as isize,
                },
                &mut display_list,
            );
            let mut frame = vec![vec![Token::Char(' '); cols as usize]; rows as usize];
            display_list.draw_on(&mut frame);
            print_frame(frame)?;
        }
        stdout.execute(Show)?;
        stdout.execute(LeaveAlternateScreen)?;
        disable_raw_mode()?;
        Ok(())
    });
    (sender, rendering_task)
}

pub fn render(widget: Component) -> std::io::Result<()> {
    let (frame_sender, rendering_task) = setup();
    let start = Instant::now();
    let (_, element) = widget.borrow_mut().create_element();
    _ = frame_sender.send(element);
    loop {
        let tick_start = Instant::now();
        if event::poll(Duration::default())? {
            let event = event::read()?;
            if let Event::Key(
                event @ KeyEvent {
                    code, modifiers, ..
                },
            ) = event
            {
                match (modifiers, code) {
                    (KeyModifiers::CONTROL, KeyCode::Char('c')) => {
                        drop(frame_sender);
                        rendering_task
                            .join()
                            .expect("Failed to join printing task")?;
                        return Ok(());
                    }
                    _ => send(event),
                }
            }
        }
        send(Tick(start.elapsed()));
        handle_messages(|msg| widget.borrow_mut().on_message(msg));
        let (did_rebuild, element) = widget.borrow_mut().create_element();
        if did_rebuild {
            _ = frame_sender.send(element);
        }
        // saturating sub to prevent overflow when subtracting duration
        thread::sleep(Duration::from_millis(10).saturating_sub(tick_start.elapsed()));
    }
}
