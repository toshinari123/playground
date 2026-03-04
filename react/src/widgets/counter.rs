use crate::{
    component::prelude::*, message::MessageFlow::Propagate, prelude::text, widget::prelude::*,
};
use crossterm::event::{KeyCode, KeyEvent};
use stdext::prelude::*;

pub fn counter(i: i32) -> Component {
    Widget::stateful(
        i,
        |this, msg| {
            switch(msg).case(|event: &KeyEvent| match event.code {
                KeyCode::Char('+') => this.set_state(|i| *i += 1),
                KeyCode::Char('-') => this.set_state(|i| *i -= 1),
                _ => {}
            });
            Propagate
        },
        |i| text(*i),
    )
}
