use crate::{component::prelude::*, prelude::RowElement, widget::prelude::*, message::MessageFlow};

pub fn row(children: impl IntoIterator<Item = Component>) -> Component {
    let widgets = children.into_iter().collect::<Vec<_>>();
    Widget::containerlike(widgets, |_, _| MessageFlow::Propagate, |this| {
        let (did_rebuild, children): (Vec<_>, Vec<_>) = this
            .children
            .iter()
            .map(|child| child.borrow_mut().create_element())
            .unzip();
        let did_any_child_rebuild = did_rebuild.into_iter().fold(false, |acc, e| acc || e);
        (
            did_any_child_rebuild,
            Box::new(RowElement {
                children,
            }),
        )
    })
}
