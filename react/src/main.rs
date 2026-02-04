use std::io::Result;

use crossterm::event::{KeyCode, KeyEvent};
use react::prelude::*;
use stdext::prelude::*;
//use crate::elements::column_element::ColumnElement;

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
        // 3. variable number of textfields (type a number to change)
        variable_numoftextfields(),
        // 4. timer
        timer()
    ];
    // change number below 
    render(options[3].clone())
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
    let (textfield, buffer) = text_field("");
    Widget::stateful(
        (textfield, buffer),
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

fn column_create_element(
    _this: &mut Widget<usize>,
    child_elements: Vec<(String, Box<dyn Element>)>,
) -> (bool, Box<dyn Element>) {
    // Extract just the elements, dropping the keys
    let elements: Vec<Box<dyn Element>> = child_elements.into_iter().map(|(_, el)| el).collect();
    (false, Box::new(ColumnElement { children: elements }))
}

fn variable_numoftextfields() -> Component {
    Widget::stateful_container(
        1,
        |this, msg| {
            switch(msg)
                .case(|event: &KeyEvent| match event.code {
                    KeyCode::Char(c) if c.is_ascii_digit() => {
                        //change number of textfields to c
                        let num = c.to_digit(10).unwrap() as usize;
                        this.set_state(|state| *state = num);
                        Intercept
                    },
                    _ => {
                    eprintln!("Other key pressed: {:?}, propagating", event.code);
                    Propagate
                }
                //    _ => Propagate,
            })
                .default(|| Propagate)
        },
        move |numtextfields| {
            //create new hashmap with keys being "var1" "var2"...
            //and each component is empty textfield
            let mut map = std::collections::HashMap::new();
            for i in 1..=*numtextfields {
                let key = format!("var{}", i);
                let component = text_field("").0;
                map.insert(key, component);
            }
            map
        },
        column_create_element,
    )
}

// potential problems with the approach right now:
// - same key, different widget type (like TextField -> Timer)
//   - dont care, widget.rs reconcile still returns TextField
// - 