use crate::{component::prelude::*, prelude::RowElement, widget::prelude::*};

pub fn row(children: impl IntoIterator<Item = Component>) -> Component {
    let widgets = children.into_iter().collect::<Vec<_>>();
    Widget::elemental(widgets, propagate, move |this| {
        // Convert children to hashmap with index keys for reconciliation
        let children_map = children_with_indices(this.state.clone());
        
        // Reconcile children (this will update this.children internally via create_child)
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
            Box::new(RowElement {
                children: child_elements,
            }),
        )
    })
}
