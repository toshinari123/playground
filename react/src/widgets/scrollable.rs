use crossterm::event::{KeyCode, KeyEvent};
use stdext::prelude::*;

use crate::prelude::{Axis, Component, ScrollableElement, Vec2, Widget};

pub fn scrollable(axis: Axis, child: Component) -> Component {
    Widget::elemental(
        (axis, Vec2::default(), child),
        |this, msg| {
            let axis = this.state.0;
            switch(msg).case(|event: &KeyEvent| match event.code {
                KeyCode::Up => {
                    if axis == Axis::Y {
                        this.set_state(|(_, offset, _)| offset.y = (offset.y - 1).max(0));
                    }
                }
                KeyCode::Down => {
                    if axis == Axis::Y {
                        this.set_state(|(_, offset, _)| offset.y += 1);
                    }
                }
                KeyCode::Left => {
                    if axis == Axis::X {
                        this.set_state(|(_, offset, _)| offset.x = (offset.x - 1).max(0));
                    }
                }
                KeyCode::Right => {
                    if axis == Axis::X {
                        this.set_state(|(_, offset, _)| offset.x += 1);
                    }
                }
                _ => {}
            });
            this.state.2.borrow_mut().on_message(msg);
        },
        |this| {
            // "needs rebuild" just means the state changed, there is no builder function and build() is not run
            let needs_rebuild = this.get_needs_rebuild();
            if needs_rebuild {
                this.mark_did_rebuild();
            }
            let (did_child_rebuild, child) = this.state.2.borrow_mut().create_element();
            (
                needs_rebuild || did_child_rebuild,
                Box::new(ScrollableElement {
                    offset: this.state.1,
                    child,
                }),
            )
        },
    )
}

pub fn scrollable_h(child: Component) -> Component {
    scrollable(Axis::X, child)
}

pub fn scrollable_v(child: Component) -> Component {
    scrollable(Axis::Y, child)
}

pub fn scrollable_2d(child: Component) -> Component {
    scrollable_h(scrollable_v(child))
}
