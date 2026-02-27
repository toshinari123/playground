use crate::{
    component::prelude::*, elements::column_element::ColumnElement, focus,
    prelude::MessageFlow::Propagate, widget::prelude::*,
};

pub fn column(children: impl IntoIterator<Item = Component>) -> Component {
    let children_vec = children.into_iter().collect::<Vec<_>>();
    
    Widget::stateful_container(
        (),
        |_, _| Propagate,
        move |_| children_vec.clone(),
        |_, child_elements| {
            // Create ColumnElement from child elements
            (
                false,
                Box::new(ColumnElement {
                    children: child_elements,
                }),
            )
        },
    )
}

/// Create a focusable column widget
///
/// This is a column that can receive keyboard focus and navigate between its children
/// using the Tab key.
pub fn focusable_column(children: impl IntoIterator<Item = Component>) -> Component {
    let children_vec = children.into_iter().collect::<Vec<_>>();
    
    focus::focusable(
        (),
        |_, _| Propagate,
        move |_| children_vec.clone(),
    )
}
