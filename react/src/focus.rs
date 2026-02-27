use std::ops::{Deref, DerefMut};

/// Position of focus relative to this widget
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FocusPosition {
    /// A child at the given index is focused
    FocusedChild(usize),
    /// This widget itself is focused
    ThisWidgetFocused,
    /// This widget is not on the current focus path
    NotOnFocusPath,
}

/// Wrapper state that adds focus capabilities to any inner state
#[derive(Debug, Clone)]
pub struct FocusWrapperState<Inner> {
    /// The inner widget state
    pub inner: Inner,
    /// Current focus position relative to this widget
    pub focus_position: FocusPosition,
    /// Whether this widget can receive focus
    pub focusable: bool,
}

impl<Inner> FocusWrapperState<Inner> {
    /// Create a new focus wrapper state
    pub fn new(inner: Inner, focusable: bool) -> Self {
        Self {
            inner,
            focus_position: FocusPosition::NotOnFocusPath,
            focusable,
        }
    }
    
    /// Check if this widget or any descendant is focused
    pub fn is_focused(&self) -> bool {
        matches!(self.focus_position, FocusPosition::ThisWidgetFocused)
    }
    
    /// Check if a child is focused
    pub fn is_child_focused(&self) -> Option<usize> {
        match self.focus_position {
            FocusPosition::FocusedChild(idx) => Some(idx),
            _ => None,
        }
    }
}

impl<Inner> Deref for FocusWrapperState<Inner> {
    type Target = Inner;
    
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<Inner> DerefMut for FocusWrapperState<Inner> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

use std::{cell::RefCell, rc::Rc};

use crate::{
    component::prelude::*,
    elements::{ascii_box_elements::AsciiBoxElement, column_element::ColumnElement},
    message::prelude::*,
    widget::prelude::*,
};

/// Handle focus navigation for a widget with focus wrapper state
fn handle_focus_next<Inner>(
    this: &mut Widget<FocusWrapperState<Inner>>,
) -> MessageFlow {
    match this.state.focus_position {
        FocusPosition::NotOnFocusPath => {
            // Focus not in our subtree
            if this.state.focusable {
                // Focus ourselves
                this.state.focus_position = FocusPosition::ThisWidgetFocused;
                Intercept
            } else {
                // Not focusable, propagate to parent
                Propagate
            }
        }
        
        FocusPosition::ThisWidgetFocused => {
            // We're focused, try to move to first focusable child
            for i in 0..this.children.len() {
                // Check if child is a focusable widget
                let child = &this.children[i];
                let mut child_borrow = child.borrow_mut();
                
                // Try to focus the child by checking if it has FocusWrapperState
                // We need to downcast to see if it's a Widget<FocusWrapperState<T>>
                // For now, we'll assume all children are focusable if they have the right type
                // This is a simplification - in a real implementation we'd need type checking
                this.state.focus_position = FocusPosition::FocusedChild(i);
                return Intercept;
            }
            // No focusable children, can't move focus
            Propagate
        }
        
        FocusPosition::FocusedChild(idx) => {
            // A child is focused
            if idx < this.children.len() {
                // Try to move focus within the child's subtree
                // For now, we'll just move to next sibling
                for i in (idx + 1)..this.children.len() {
                    this.state.focus_position = FocusPosition::FocusedChild(i);
                    return Intercept;
                }
                // No more siblings, focus back to ourselves
                this.state.focus_position = FocusPosition::ThisWidgetFocused;
                return Intercept;
            }
            Propagate
        }
    }
}

/// Create a focusable widget container
///
/// Wraps any inner state with focus capabilities and provides focus navigation.
pub fn focusable<InnerState>(
    inner_state: InnerState,
    on_message: impl Fn(&mut Widget<FocusWrapperState<InnerState>>, &Message) -> MessageFlow + 'static,
    builder: impl Fn(&FocusWrapperState<InnerState>) -> Vec<Component> + 'static,
) -> Component
where
    InnerState: 'static,
{
    let focus_state = FocusWrapperState {
        inner: inner_state,
        focus_position: FocusPosition::NotOnFocusPath,
        focusable: true,
    };
    
    Widget::stateful_container(
        focus_state,
        move |this, msg| {
            // Handle Tab key for focus navigation
            use crossterm::event::{KeyCode, KeyEvent};
            if let Some(event) = msg.downcast_ref::<KeyEvent>() {
                if event.code == KeyCode::Tab {
                    return handle_focus_next(this);
                }
            }
            
            // Call user's message handler
            on_message(this, msg)
        },
        move |this| builder(this),
        |this, child_elements| {
            // Render based on focus position
            let inner_element = Box::new(ColumnElement { 
                children: child_elements 
            });
            
            match this.state.focus_position {
                FocusPosition::ThisWidgetFocused => {
                    // Draw with ASCII box
                    (false, Box::new(AsciiBoxElement::new(inner_element)))
                }
                _ => (false, inner_element),
            }
        },
    )
}
/*
/// Simple wrapper state for focusing a single child widget
#[derive(Debug, Clone)]
struct FocusChildState {
    child: Component,
}

/// Create a focusable wrapper around an existing widget
///
/// This is a convenience function for making any existing widget focusable.
pub fn focusable_widget(
    child: Component,
) -> Component {
    focusable(
        FocusChildState { child: child.clone() },
        |this, msg| {
            // Forward messages to child when focused
            match this.state.focus_position {
                FocusPosition::ThisWidgetFocused => {
                    this.state.child.borrow_mut().on_message(msg);
                    Intercept
                }
                _ => Propagate,
            }
        },
        |state| vec![state.child.clone()],
    )
}*/

/// Create a focus root widget
///
/// This is a focusable widget that starts with initial focus, serving as the root
/// of the focus hierarchy. When Tab is pressed, focus moves from the root to its
/// first focusable child.
pub fn focus_root(child: Component) -> Component {
    let focus_state = FocusWrapperState {
        inner: (),
        focus_position: FocusPosition::ThisWidgetFocused, // Start focused!
        focusable: true,
    };
    
    Widget::stateful_container(
        focus_state,
        move |this, msg| {
            // Handle Tab key for focus navigation
            use crossterm::event::{KeyCode, KeyEvent};
            if let Some(event) = msg.downcast_ref::<KeyEvent>() {
                if event.code == KeyCode::Tab {
                    this.set_state(|_| ());
                    return handle_focus_next(this);
                }
            }
            
            // Propagate messages to children
            Propagate
        },
        move |_| vec![child.clone()],
        |this, child_elements| {
            // child_elements is Vec<Box<dyn Element>>, we need to take ownership
            // of the first element
            let inner_element = child_elements.into_iter().next().unwrap();
            match this.state.focus_position {
                FocusPosition::ThisWidgetFocused => {
                    // Draw with ASCII box when root is focused
                    (false, Box::new(AsciiBoxElement::new(inner_element)))
                }
                _ => (false, inner_element),
            }
        },
    )
}

/// Module exports
pub mod prelude {
    pub use super::{FocusPosition, FocusWrapperState, focusable, focus_root};
}