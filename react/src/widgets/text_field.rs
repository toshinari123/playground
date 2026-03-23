use std::{cell::RefCell, fmt::Display, rc::Rc};

use crossterm::event::{KeyCode, KeyEvent};
use stdext::prelude::*;

use crate::{component::prelude::*, message::prelude::*, prelude::text_cursor, widget::prelude::*};

#[derive(Debug, Clone)]
struct TextField {
    buffer: Rc<RefCell<String>>,
    cursor: usize,
    show_cursor: bool,
}

impl TextField {
    fn insert(&mut self, c: char) {
        //eprintln!("[TEXT_FIELD::insert] Inserting char: {} to buffer {:?}", c, self.buffer);
        self.buffer.borrow_mut().insert(self.cursor, c);
        self.cursor += 1;
    }
    fn remove_left(&mut self) {
        if self.cursor > 0 {
            self.buffer.borrow_mut().remove(self.cursor - 1);
            self.cursor -= 1;
        }
    }
    fn insert_str(&mut self, s: &str) {
        self.buffer.borrow_mut().insert_str(self.cursor, s);
        self.cursor += s.len();
    }
    fn move_cursor_left(&mut self) {
        if self.cursor > 0 {
            self.cursor -= 1;
        }
    }
    fn move_cursor_right(&mut self) {
        if self.cursor < self.buffer.borrow().len() {
            self.cursor += 1;
        }
    }
    // does not work
    fn remove_word_left(&mut self) {
        let space_idx = self
            .buffer
            .borrow()
            .chars()
            .enumerate()
            .filter_map(|(i, c)| {
                if c == ' ' && i < self.cursor {
                    Some(i)
                } else {
                    None
                }
            })
            .max()
            .unwrap_or(0);
        while self.cursor > space_idx {
            self.remove_left();
        }
    }
}

pub fn text_field(initial: impl Display) -> (Component, Rc<RefCell<String>>) {
    let initial = initial.to_string();
    let len = initial.len();
    let initial = Rc::new(RefCell::new(initial));
    (
        Widget::focusable_stateful(
            TextField {
                cursor: len,
                buffer: initial.clone(),
                show_cursor: true,
            },
            |this, msg| {
                switch(msg).case(|event: &KeyEvent| match event.code {
                    KeyCode::Enter => this.set_state(|buffer| buffer.insert('\n')),
                    KeyCode::Backspace => this.set_state(|buffer| _ = buffer.remove_left()),
                    KeyCode::Char(c) => {
                        //eprintln!("[TEXT_FIELD] Inserting char: {}", c);
                        this.set_state(|buffer| buffer.insert(c));
                    },
                    KeyCode::Left => this.set_state(|state| state.move_cursor_left()),
                    KeyCode::Right => this.set_state(|state| state.move_cursor_right()),
                    KeyCode::Tab => this.set_state(|state| state.insert_str("    ")),
                    _ => {}
                });
                Intercept
            },
            |buffer| {
                //eprintln!("[TEXT_FIELD::builder] building");
                text_cursor(
                    buffer.buffer.borrow().clone(),
                    if buffer.show_cursor {
                        Some(buffer.cursor)
                    } else {
                        None
                    },
                )
            },
        ),
        initial,
    )
}
