use crate::{component::prelude::*, prelude::{RowElement, MessageFlow::Propagate}, widget::prelude::*};

pub fn row(children: impl IntoIterator<Item = Component>) -> Component {
    let children_vec = children.into_iter().collect::<Vec<_>>();
    
    Widget::stateful_container(
        (),
        |_, _| Propagate,
        move |_| children_vec.clone(),
        |_, child_elements| {
            // Create RowElement from child elements
            // The rebuild flag is already handled by create_children
            (
                false, // custom_did_rebuild - RowElement creation doesn't need rebuild
                Box::new(RowElement {
                    children: child_elements,
                }),
            )
        },
    )
}
