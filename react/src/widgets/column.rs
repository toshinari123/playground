use crate::{component::prelude::*, elements::column_element::ColumnElement, widget::prelude::*};

// Takes any iterable of Components (could be vec, array, etc.)
// Returns a single Component that represents the column
pub fn column(children: impl IntoIterator<Item = Component>) -> Component {

    // Converts the iterable into a concrete Vec<Component> to store all children.
    let widgets = children.into_iter().collect::<Vec<_>>();

    // Creates an "elemental" widget (one that directly produces an Element):
    Widget::elemental(widgets, propagate, |this| {
        
        // For each child component, create its element
        let (did_rebuild, children): (Vec<_>, Vec<_>) = this
            .state
            .iter()
            .map(|child| child.borrow_mut().create_element())
            .unzip();
        // Check if any child rebuilt
        let did_any_child_rebuild = did_rebuild.into_iter().fold(false, |acc, e| acc || e);
        (
            did_any_child_rebuild,
            Box::new(ColumnElement {
                children,
            }),
        )
    })
}
