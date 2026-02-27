use std::fmt::Display;

use crate::{component::prelude::*, focus, prelude::StringElement, widget::Widget};

#[inline]
pub fn text(s: impl Display + 'static) -> Component {
    Widget::elemental(
        s.to_string(),
        |_, _| {},
        |this| {
            (
                false,
                Box::new(StringElement {
                    s: this.state.clone(),
                    cursor: None,
                }),
            )
        },
    )
}
