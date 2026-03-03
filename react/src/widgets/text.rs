use std::fmt::Display;

use crate::{
    component::prelude::*,
    prelude::StringElement,
    widget::{FocusState, Widget},
};

#[inline]
pub fn text(s: impl Display + 'static) -> Component {
    Widget::focusable_elemental(
        s.to_string(),
        |_, _| {},
        |this| {
            (
                false,
                Box::new(StringElement {
                    s: match this.focused_child_index {
                        FocusState::SelfFocused => "focused".to_string(),
                        _ => this.state.clone(),
                    },
                    cursor: None,
                }),
            )
        },
    )
}
