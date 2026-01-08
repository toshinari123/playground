use std::io::Result;

use react::prelude::*;
use react::widgets::streamed_counter::streamed_counter;
use crossterm::event::{KeyEvent, KeyCode};
use stdext::prelude::*;
use std::{cell::RefCell, fmt::Debug, ops::RangeFrom, rc::Rc, any::Any};


fn main() -> Result<()> {
    /*render(row([
        column([streamed_counter(), text_field("").0, animated_char()]),
        column([text_field("").0, download("https://www.rust-lang.org")]),
        col_of_animated_char(),
    ]))*/
    render(todo())
}

fn col_of_animated_char() -> Component {
    Widget::stateful(1, |this, msg| {switch(msg).case(|event: &KeyEvent| match event.code {
        KeyCode::Char('+') => this.set_state(|no| *no += 1),
        _ => {}
    }); Intercept}, |&no| column((0..no).map(|_| animated_char())))
}

enum TodoEvent {
    AddTask,
}

fn todo() -> Component { // actually a containerlike() with focus AND childpersistence logic; TODO factor the 2 logics out
    Widget::<(usize, usize), dyn FocusableComponent>::containerlike(
        (0,1),
        vec![Rc::new(RefCell::new(CustomTextField{
            inner_text_field: text_field("input").0,
            on_enter: Box::new(|| send(TodoEvent::AddTask)),
        }))],
        |this, msg| {
            let msg2 = msg.clone();
            switch(msg).case(|event: &TodoEvent| {
                    eprintln!("todo received addtask case");
                match event {
                TodoEvent::AddTask => {
                    eprintln!("todo received addtask");
                    this.children.push(
                        Rc::new(RefCell::new(CustomTextField{
                            inner_text_field: text_field("new").0,
                            on_enter: Box::new(|| {}),
                        }))
                    );
                    this.set_state(|state| state.1 = state.1+1);
                }
            }}).case(|event: &KeyEvent| match event.code {
                KeyCode::Tab => this.set_state(|state| state.0 = (state.0+1).rem_euclid(state.1)),
                KeyCode::BackTab => this.set_state(|state| state.0 = (state.0+state.1-1).rem_euclid(state.1)),
                KeyCode::Enter => {},
                other => {
                    eprintln!("todo received other keypress");
                    this.children[this.state.0].borrow_mut().on_message(msg);
                }
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
            let element = Box::new(BottomColumnElement {
                children: child_elements,
                footer_height: 1,
            });
            (did_any_child_rebuild, element)
        }
    )
}

struct CustomTextField { // Focusable and _Component
    inner_text_field: Component,
    on_enter: Box<dyn Fn() -> ()>, 
}

impl Debug for CustomTextField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Widget {{ id: {} }}", self.id())
    }
}

impl _Component for CustomTextField {
    fn id(&self) -> usize {
        self.inner_text_field.borrow().id()
    }
    fn create_element(&mut self) -> (bool, Box<dyn Element>) {
        let (_, e1) = text(" - ").borrow_mut().create_element();
        let (b2, e2) = self.inner_text_field.borrow_mut().create_element();
        (b2, Box::new(LeftRowElement {
            children: vec![e1, e2],
            sidebar_width: 3,
        }))
    }
    //fn needs_rebuild(&mut self) -> bool {
    //    self.inner_text_field.borrow_mut().needs_rebuild()
    //}
    fn on_message(&mut self, msg: &Message) {
        //if let Some(event) = msg.downcast_ref::<KeyCode>() {
        switch(msg).case(|event: &KeyEvent| match event.code {
            KeyCode::Enter => (self.on_enter)(),
            other => {
                eprintln!("customtextefield received other keypress");
                self.inner_text_field.borrow_mut().on_message(msg);
            }
        });
    //}
    }
}

impl Focusable for CustomTextField {
    fn create_focused_element(&mut self) -> (bool, Box<dyn Element>) {
        let (_, e1) = text("-> ").borrow_mut().create_element();
        let (b2, e2) = self.inner_text_field.borrow_mut().create_element();
        (b2, Box::new(LeftRowElement {
            children: vec![e1, e2],
            sidebar_width: 3,
        }))
    }
}

impl FocusableComponent for CustomTextField {}