use crate::{
    prelude::*,
    widget::{Widget, propagate},
};

pub fn stack(children: Vec<Component>) -> Component {
    Widget::stateful(children, |this, msg| {
        propagate(this, msg);
        Propagate
    }, |children| {
        Widget::elemental(
            children.clone(),
            |this, msg| {
                this.state
                    .iter()
                    .for_each(|child| child.borrow_mut().on_message(msg));
            },
            |this| {
                let children: Vec<_> = this
                    .state
                    .iter()
                    .map(|child| child.borrow_mut().create_element())
                    .collect();

                let did_rebuild = children.iter().any(|(did_rebuild, _)| *did_rebuild);
                let children = children.into_iter().map(|(_, element)| element).collect();

                (did_rebuild, Box::new(StackElement { children }))
            },
        )
    })
}