use crossterm::event::{KeyCode, KeyEvent};
use stdext::prelude::switch;

use crate::{
    message::MessageFlow::{Intercept, Propagate},
    prelude::{Component, column},
    style::{Style, height, width},
    utils::Constraint::Flex,
    widget::Widget,
};

pub fn adjustable_column(child1: Component, child2: Component) -> Component {
    Widget::stateful(
        (10, 10, child1, child2),
        |this, msg| {
            switch(msg)
                .case(|event: &KeyEvent| match event.code {
                    KeyCode::Up => {
                        this.set_state(|offset| {
                            offset.0 = (offset.0 - 1).max(0);
                            offset.1 = (offset.1 + 1).min(20);
                        });
                        Intercept
                    }
                    KeyCode::Down => {
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
            column([
                child1.clone().style(height(Flex(*left_flex))),
                child2.clone().style(height(Flex(*right_flex))),
            ])
        },
    )
}
