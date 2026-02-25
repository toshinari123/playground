use crate::{component::prelude::*, elements::column_element::ColumnElement, widget::prelude::*};

pub fn column(children: impl IntoIterator<Item = Component>) -> Component {
    let widgets = children.into_iter().collect::<Vec<_>>();
    Widget::elemental(widgets, propagate, |this| {
        // Create elements for all children
        let (did_rebuild, child_elements): (Vec<_>, Vec<_>) = this
            .state
            .iter()
            .enumerate()
            .map(|(i, child)| {
                let (did_rebuild, element) = child.borrow_mut().create_element();
                (did_rebuild, element)
            })
            .unzip();
        
        let did_any_child_rebuild = did_rebuild.into_iter().fold(false, |acc, e| acc || e);
        (
            did_any_child_rebuild,
            Box::new(ColumnElement {
                children: child_elements,
            }),
        )
    })
}
