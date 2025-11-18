use crate::{
    prelude::{Component, StringElement},
    widget::Widget,
};

pub fn single_line(s: String) -> Component {
    Widget::elemental(
        s,
        vec![],
        |_, _| {},
        #[inline]
        |this| {
            (false, Box::new(StringElement {
                s: this.state.clone(),
            }))
        },
    )
}
