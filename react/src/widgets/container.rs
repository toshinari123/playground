use crate::{component::prelude::*, elements::column_element::ColumnElement, widget::prelude::*};

/**
Container:
A single-child layout widget

properties:
- alignment: Alignment enum (Start, Center, End, Stretch)
- padding: u16 (number of spaces around the child)  
- margin: u16 (number of spaces outside the border)
- border: bool (whether to draw a border around the container)
- optionally: background_color/image
*/

// Takes a component child and returns a single component
pub fn container(child: Component, alignment: Alignment, padding: u16, margin: u16, border: bool) -> Component {

    // Converts the iterable into a concrete Vec<Component> to store the child.
    let widgets = vec![child];

    // Creates an "elemental" widget (one that directly produces an Element):
    Widget::elemental(widgets, propagate, |this| {
        
        // For each child component, create its element
        let (did_rebuild, children): (Vec<_>, Vec<_>) = this
            .state
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
