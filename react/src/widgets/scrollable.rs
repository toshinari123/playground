use crate::{
    elements::scrollable_column_element::{ScrollState, ScrollableColumnElement},
    prelude::*,
    widget::prelude::*,
};
use crossterm::event::{KeyCode, KeyEvent};
use std::sync::{Arc, Mutex};
use stdext::prelude::*;

// Creates a scrollable column widget. Use arrow keys to scroll up/down.
pub fn scrollable(children: Vec<Component>) -> Component {
    let scroll_state = Arc::new(Mutex::new(ScrollState::new()));

    Widget::elemental(
        (children, scroll_state),
        |this, msg| {
            switch(msg).case(|event: &KeyEvent| {
                let (_, scroll_state) = &this.state;
                let mut state = scroll_state.lock().unwrap();
                match event.code {
                    KeyCode::Down => state.scroll_down(),
                    KeyCode::Up => state.scroll_up(),
                    _ => {}
                }
            });
            let (children, _) = &this.state;
            for child in children {
                child.borrow_mut().on_message(msg);
            }
        },
        |this| {
            let (children, scroll_state) = &this.state;
            let child_elements: Vec<Box<dyn Element>> = children
                .iter()
                .map(|c| c.borrow_mut().create_element().1)
                .collect();

            (
                true,
                Box::new(ScrollableColumnElement {
                    children: child_elements,
                    scroll_state: scroll_state.clone(),
                }),
            )
        },
    )
}