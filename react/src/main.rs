use std::io::Result;

use crossterm::event::{KeyCode, KeyEvent};
use react::prelude::*;
use stdext::prelude::*;

fn main() -> Result<()> {
    let options = [
        // 0. type in textfields
        row([
            column([timer(), text_field("").0]),
            column([text_field("").0, download("https://www.rust-lang.org")]),
        ]),
        // 1. `enter` key makes new texts
        todo_list(),
        // 2. `enter` key swaps
        swap(text("/---\\\n|   |\n\\---/"), text_field("").0),
    ];
    // change number below 
    render(options[2].clone())
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

//--------

fn swap(a: StaticComponent, b: StaticComponent) -> Component {
    Widget::stateful(
        false,
        |this, msg| {
            switch(msg)
                .case(|event: &KeyEvent| match event.code {
                    KeyCode::Enter => {
                        this.set_state(|swapped| *swapped = !*swapped);
                        Propagate
                    }
                    _ => Propagate,
                })
                .default(|| Propagate)
        },
        move |swapped| {
            let mut v = vec![a.clone(), b.clone()];
            if *swapped {
                v = vec![b.clone(), a.clone()];
            }
            row(v)
        }
    )
}

//-------- todo: need new convenience func in widget.rs cuz elemental has no state and stateful auto assume 1 child (currently)
/*
fn variable_numoftextfields() -> Component {
    Widget::stateful(
        1,
        |this, msg| {
            switch(msg)
                .case(|event: &KeyEvent| match event.code {
                    KeyCode::Char(c) if c.is_ascii_digit() => {
                        //change number of textfields
                        Intercept
                    }
                    _ => Propagate,
                })
                .default(|| Propagate)
        },
        move |swapped| {
            column(v)
        }
    )
}
*/