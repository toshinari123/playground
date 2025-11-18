use crate::{component::prelude::*, prelude::RowElement, widget::prelude::*};

pub fn row(children: impl IntoIterator<Item = Component>) -> Component {
    // remove this line below if Widget changed to use generic type Children
    let children_vec = children.into_iter().collect::<Vec<_>>(); 
    Widget::elemental(
        (),
        children_vec,
        propagate_msg,
        statelessly_childfully_create_element_functional(Box::new(move |children| {
            Box::new(RowElement { children: children })
        })),
    )
}
