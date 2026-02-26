use crate::{component::prelude::*, elements::column_element::ColumnElement, prelude::MessageFlow::Propagate, widget::prelude::*};

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
