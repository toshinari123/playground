use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use stdext::prelude::switch;

use crate::{
    component::{_Component, Component, Dir},
    message::MessageFlow,
    widget::{Widget, MessageToFocused, FocusState},
};

pub fn focus_root(child: Component) -> Component {
    let child_clone = child.clone();
    Widget::stateful_container(
        (),
        move |this, msg| {
            let mut iskey = false;
            switch(msg).case(|event: &KeyEvent| {
                iskey = true;
                match event.code {
                    KeyCode::BackTab => {
                        this.change_focus(Dir::Negative);
                        if matches!(this.focused_child_index, FocusState::NotFocused) { this.change_focus(Dir::Negative); }
                        this.mark_needs_rebuild();
                    },
                    KeyCode::Tab => {
                        this.change_focus(Dir::Positive);
                        if matches!(this.focused_child_index, FocusState::NotFocused) { this.change_focus(Dir::Positive); }
                        this.mark_needs_rebuild();
                    },
                    _ => {
                        let wrapped = MessageToFocused {
                            internal: stdext::prelude::any(event.clone()),
                        };
                        this.children[0].borrow_mut().on_message(&stdext::prelude::any(wrapped));
                    }
                }
            });
            if iskey { MessageFlow::Intercept } else { MessageFlow::Propagate }
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
