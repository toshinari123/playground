use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use stdext::prelude::switch;

use crate::{
    component::{Component, Dir},
    message::MessageFlow,
    widget::{Widget, MessageToFocused},
};

pub fn focus_root(child: Component) -> Component {
    let child_clone = child.clone();
    Widget::stateful_container(
        (),
        move |this, msg| {
            switch(msg).case(|event: &KeyEvent| {
                match event.code {
                    // TODO: specific focuschange event (or can even specify which focusroot if theres multiple)
                    KeyCode::BackTab => _ = this.children[0].borrow_mut().change_focus(Dir::Negative),
                    KeyCode::Tab => _ = this.children[0].borrow_mut().change_focus(Dir::Positive),
                    _ => {
                        // Send FocusWrapped message to child
                        let wrapped = MessageToFocused {
                            internal: stdext::prelude::any(event.clone()),
                        };
                        this.children[0].borrow_mut().on_message(&stdext::prelude::any(wrapped));
                    }
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
