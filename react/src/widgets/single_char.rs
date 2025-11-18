use crate::{component::prelude::*, prelude::CharElement, widget::Widget};

pub fn single_char(c: char) -> Component {
    Widget::elemental((), vec![], |_, _| (), move |_| (false, Box::new(CharElement { c })))
}
