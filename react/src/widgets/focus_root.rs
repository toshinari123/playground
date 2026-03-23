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
        Option::<Component>::None,
        move |this, msg| {
            switch(msg).case(|event: &KeyEvent| {
                match event.code {
                    // TODO: specific focuschange event (or can even specify which focusroot if theres multiple)
                    KeyCode::BackTab => {
                        let result = this.children[0].borrow_mut().change_focus(Dir::Negative);
                        if let crate::widget::FocusState::ChildFocused { component, .. } = result {
                            this.state = Some(component);
                        }
                        let state_clone = this.state.clone();
                        this.set_state(|state| *state = state_clone);
                    }
                    KeyCode::Tab => {
                        let result = this.children[0].borrow_mut().change_focus(Dir::Positive);
                        if let crate::widget::FocusState::ChildFocused { component, .. } = result {
                            this.state = Some(component);
                        }
                        let state_clone = this.state.clone();
                        this.set_state(|state| *state = state_clone);
                    }
                    _ => {
                        if let Some(ref focused) = this.state {
                            focused.borrow_mut().on_message(msg);
                        }
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
