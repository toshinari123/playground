use std::fmt::Display;

use crate::{
    component::prelude::*,
    prelude::{BoxWrappingElement, CenteredStringElement},
    widget::{FocusState, Widget},
};

#[inline]
pub fn focusable_centered_text(s: impl Display + 'static) -> Component {
    Widget::focusable_elemental(
        s.to_string(),
        |_, _| {},
        |this| {
            let string_element = Box::new(CenteredStringElement {
                s: this.state.clone(),
                //cursor: None,
            });
            
            (false, match this.focused_child_index {
                FocusState::SelfFocused => {
                    // When focused, wrap the string element in a box
                    Box::new(BoxWrappingElement {
                        child: string_element,
                    })
                }
                _ => string_element,
            })
        },
    )
}
