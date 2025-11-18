use crate::{component::prelude::*, elements::column_element::ColumnElement, widget::prelude::*};

pub fn column(children: impl IntoIterator<Item = Component>) -> Component {
    // remove this line below if Widget changed to use generic type Children
    let children_vec = children.into_iter().collect::<Vec<_>>(); 
    Widget::elemental(
        (),
        children_vec,
        propagate_msg,
        statelessly_childfully_create_element_functional(Box::new(move |children| {
            Box::new(ColumnElement { children: children })
        })),
    )
}
