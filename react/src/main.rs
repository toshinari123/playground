use std::io::Result;

//use crossterm::event::{KeyCode, KeyEvent};
use react::prelude::*;
use react::widgets::streamed_counter::streamed_counter;
use crossterm::event::{KeyEvent, KeyCode};
use stdext::prelude::*;
use std::{cell::RefCell, fmt::Debug, ops::RangeFrom, rc::Rc, any::Any};

/*

    render(row([
        column([
            streamed_counter(), 
            text_field("").0, 
            animated_char()
        ]),
        column([
            text_field("").0,
            download("https://www.rust-lang.org")
        ]),
        col_of_animated_char(),
    ]))


fn col_of_animated_char() -> Component {
    Widget::stateful(1, |this, msg| {switch(msg).case(|event: &KeyEvent| match event.code {
        KeyCode::Char('+') => this.set_state(|no| *no += 1),
        _ => {}
    }); Intercept}, |&no| column((0..no).map(|_| animated_char())))
}

*/

fn main() -> Result<()> {
    render(todo())
}

enum TodoEvent {
    AddTask(String),
}

fn todo() -> Component { // actually a containerlike() with focus AND childpersistence logic; TODO factor the 2 logics out
    let (c, t) = text_field("type here to make new task...");
    Widget::<(usize, usize), dyn FocusableComponent>::containerlike(
        (0,1),
        vec![Rc::new(RefCell::new(CustomTextField{
            text_field: c,
            text: t.clone(),
            on_enter: Box::new(|this| send(TodoEvent::AddTask(this.text.borrow().to_string()))),
        }))],
        |this, msg| {
            let msg2 = msg.clone();
            switch(msg).case(|event: &TodoEvent| match event {
                TodoEvent::AddTask(initial_task_text) => {
                    let (c, t) = text_field(initial_task_text);
                    this.children.push(
                        Rc::new(RefCell::new(CustomTextField{
                            text_field: c,
                            text: t,
                            on_enter: Box::new(|this| {}),
                        }))
                    );
                    this.set_state(|state| state.1 = state.1+1);
                }
            }).case(|event: &KeyEvent| match event.code {
                KeyCode::Tab => this.set_state(|state| state.0 = (state.0+1).rem_euclid(state.1)),
                KeyCode::BackTab => this.set_state(|state| state.0 = (state.0+state.1-1).rem_euclid(state.1)),
                _ => this.children[this.state.0].borrow_mut().on_message(msg),
            });
            Intercept
        }, 
        |this| {
            let f = this.state.0;
            let mut did_any_child_rebuild = false;
            let mut child_elements = Vec::with_capacity(this.children.len());
            for (i, child) in this.children.iter().enumerate() {
                let (did_rebuild, elem) = if i == f {
                    child.borrow_mut().create_focused_element()
                } else {
                    child.borrow_mut().create_element()
                };
                did_any_child_rebuild = did_any_child_rebuild || did_rebuild;
                child_elements.push(elem);
            }
            // Split child_elements into footer (first) and main (rest)
            let mut child_elements = child_elements;
            let footer = child_elements.remove(0);
            let main = Box::new(ColumnElement{
                children: child_elements,
            });
            
            let element = Box::new(FooterMainElement {
                footer,
                main,
                footer_height: 1,
                box_main: true,
            });
            (this.get_and_reset_needs_rebuild() || did_any_child_rebuild, element)
        }
    )
}

struct CustomTextField { // Focusable and _Component
    text_field: Component,
    text: Rc<RefCell<String>>,
    on_enter: Box<dyn Fn(&Self) -> ()>, 
}

impl Debug for CustomTextField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Widget {{ id: {} }}", self.id())
    }
}

impl _Component for CustomTextField {
    fn id(&self) -> usize {
        self.text_field.borrow().id()
    }
    fn create_element(&mut self) -> (bool, Box<dyn Element>) {
        let (_, e1) = text(" -").borrow_mut().create_element();
        let (b2, e2) = self.text_field.borrow_mut().create_element();
        //eprintln!("customtextfield createelem {}", b2);
        (b2, Box::new(LeftRowElement {
            children: vec![e1, e2],
            sidebar_width: 3,
        }))
    }
    fn on_message(&mut self, msg: &Message) {
        switch(msg).case(|event: &KeyEvent| match event.code {
            KeyCode::Enter => (self.on_enter)(self), //TODO: on_enter should prolly return MessageType
            other => self.text_field.borrow_mut().on_message(msg),
        });
    }
}

impl Focusable for CustomTextField {
    fn create_focused_element(&mut self) -> (bool, Box<dyn Element>) {
        let (_, e1) = text("->").borrow_mut().create_element();
        let (b2, e2) = self.text_field.borrow_mut().create_element();
        //eprintln!("customtextfield createfocusedelem {}", b2);
        (b2, Box::new(LeftRowElement {
            children: vec![e1, e2],
            sidebar_width: 3,
        }))
    }
}

impl FocusableComponent for CustomTextField {}