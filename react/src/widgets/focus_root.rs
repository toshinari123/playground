use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use stdext::prelude::switch;

use crate::{
    component::{Component, Dir},
    message::MessageFlow,
    widget::Widget,
};

pub fn focus_root(child: Component) -> Component {
    let child_clone = child.clone();
    Widget::stateful_container(
        (),
        move |this, msg| {
            switch(msg).case(|event: &KeyEvent| {
                match (event.modifiers, event.code) {
                    (KeyModifiers::SHIFT, KeyCode::BackTab) => {
                        this.children[0].borrow_mut().change_focus(Dir::Negative);
                        this.set_state(|_| ());
                    }
                    (KeyModifiers::NONE, KeyCode::Tab) => {
                        this.children[0].borrow_mut().change_focus(Dir::Positive);
                        this.set_state(|_| ());
                    }
                    _ => {}
                }
            });
            MessageFlow::Intercept
        },
        move |_| vec![child_clone.clone()],
        |this, mut children_elem| {
            if !children_elem.is_empty() {
                (false, children_elem.remove(0))
            } else {
                (false, Box::new(crate::elements::string_element::StringElement {
                    s: "focusroot.create_element received empty Vec for childrenelements".to_string(),
                    cursor: None,
                }))
            }
        },
    )
}
