use crossterm::event::{KeyCode, KeyEvent};
use stdext::prelude::switch;

use crate::{
    message::MessageFlow::{Intercept, Propagate},
    prelude::{Component, row},
    style::{Style, width},
    utils::Constraint::Flex,
    widget::Widget,
};

pub fn adjustable_row(child1: Component, child2: Component) -> Component {
    Widget::stateful(
        (10, 10, child1, child2),
        |this, msg| {
            switch(msg)
                .case(|event: &KeyEvent| match event.code {
                    KeyCode::Left => {
                        this.set_state(|offset| {
                            offset.0 = (offset.0 - 1).max(0);
                            offset.1 = (offset.1 + 1).min(20);
                        });
                        Intercept
                    }
                    KeyCode::Right => {
                        this.set_state(|offset| {
                            offset.0 = (offset.0 + 1).min(20);
                            offset.1 = (offset.1 - 1).max(0);
                        });
                        Intercept
                    }
                    _ => Propagate,
                })
                .default(|| Propagate)
        },
        |(left_flex, right_flex, child1, child2)| {
            row([
                child1.clone().style(width(Flex(*left_flex))),
                child2.clone().style(width(Flex(*right_flex))),
            ])
        },
    )
}
