use std::io::Result;

use crossterm::event::{KeyCode, KeyEvent};
use react::prelude::*;
use stdext::prelude::*;

fn main() -> Result<()> {
    render(row([
        column([counter(12), text_field("").0]),
        column([text_field("").0, download("https://www.rust-lang.org")]),
    ]))
    // render(todo_list())
}

struct AddTask(String);

fn todo_list() -> Component {
    Widget::stateful(
        vec![],
        |this, msg| {
            switch(msg)
                .case(|AddTask(task)| {
                    eprintln!("New task!");
                    this.set_state(|tasks| tasks.push(task.clone()));
                    Intercept
                })
                .default(|| Propagate)
        },
        |tasks| {
            column(
                tasks
                    .iter()
                    .map(|task| text(task.clone()))
                    .chain([add_task()]),
            )
        },
    )
}

fn add_task() -> Component {
    // let (textfield, buffer) = text_field("");
    Widget::stateful(
        text_field(""),
        |this, msg| {
            switch(msg)
                .case(|event: &KeyEvent| match event.code {
                    KeyCode::Enter => {
                        eprintln!("Pressed enter!");
                        let (_, buffer) = &this.state;
                        send(AddTask(buffer.borrow().clone()));
                        Intercept
                    }
                    _ => Propagate,
                })
                .default(|| Propagate)
        },
        |(textfield, _)| textfield.clone(),
    )
}
