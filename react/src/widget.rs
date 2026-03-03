use std::{cell::RefCell, fmt::Debug, ops::RangeFrom, rc::Rc};

use stdext::prelude::switch;
use tokio::sync::mpsc::{UnboundedSender, unbounded_channel};

use crate::{
    component::prelude::*,
    message::prelude::*,
    prelude::Element,
    render::Tick,
    runtime::{Stream, Task, go},
    // tree,
};

pub mod prelude {
    pub use super::{Widget, propagate};
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FocusState {
    NotFocused,
    ChildFocused(usize),
    SelfFocused,
}

thread_local! {
    pub(crate) static COUNTER: RefCell<RangeFrom<usize>> = RefCell::new(0..);
}

pub fn uid() -> usize {
    COUNTER.with_borrow_mut(|counter| counter.next().expect("Ran out of UIDs for Widgets"))
}

pub struct Widget<State> {
    id: usize,
    pub state: State,
    pub children: Vec<Component>,
    needs_rebuild: bool,
    builder: Box<dyn Fn(&State) -> Vec<Component>>,
    on_message: Rc<dyn Fn(&mut Self, &Message)>,
    create_element: Rc<dyn Fn(&mut Self) -> (bool, Box<dyn Element>)>,
    pub focused_child_index: FocusState,
    is_focusable: bool,
}

impl<State> Debug for Widget<State> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Widget(id: {}, children_len: {})",
            self.id,
            self.children.len()
        )
    }
}

impl<State> Widget<State>
where
    State: 'static,
{
    pub fn stateful(
        state: State,
        on_message: impl Fn(&mut Self, &Message) -> MessageFlow + 'static,
        builder: impl Fn(&State) -> Component + 'static,
    ) -> Component {
        let id = uid();
        // Register widget in tree (no parent yet, will be set when added as child)
        // tree::register(id, None);
        
        Rc::new(RefCell::new(Widget {
            id,
            state: state,
            children: Vec::new(),
            needs_rebuild: true,
            builder: Box::new(move |state| vec![builder(state)]),
            on_message: Rc::new(move |this, msg| {
                if let Propagate = on_message(this, msg) {
                    // Propagate to all children
                    for child in &this.children {
                        child.borrow_mut().on_message(msg);
                    }
                }
            }),
            create_element: Rc::new(create_child_always_replace),
            focused_child_index: FocusState::NotFocused,
            is_focusable: false,
        }))
    }
    
    pub fn stateful_container(
        state: State,
        on_message: impl Fn(&mut Self, &Message) -> MessageFlow + 'static,
        builder: impl Fn(&State) -> Vec<Component> + 'static,
        create_element: impl Fn(&mut Self, Vec<Box<dyn Element>>) -> (bool, Box<dyn Element>) + 'static,
    ) -> Component {
        let id = uid();
        // Register widget in tree (no parent yet, will be set when added as child)
        // tree::register(id, None);
        
        Rc::new(RefCell::new(Widget {
            id,
            state: state,
            children: Vec::new(),
            needs_rebuild: true,
            builder: Box::new(builder),
            on_message: Rc::new(move |this, msg| {
                if let Propagate = on_message(this, msg) {
                    // Propagate to all children
                    for child in &this.children {
                        child.borrow_mut().on_message(msg);
                    }
                }
            }),
            create_element: Rc::new(create_children(create_element)),
            focused_child_index: FocusState::NotFocused,
            is_focusable: false,
        }))
    }
    
    pub fn elemental(
        state: State,
        on_message: impl Fn(&mut Self, &Message) + 'static,
        create_element: impl Fn(&mut Self) -> (bool, Box<dyn Element>) + 'static,
    ) -> Component {
        let id = uid();
        // Register widget in tree (no parent yet, will be set when added as child)
        // tree::register(id, None);
        
        Rc::new(RefCell::new(Widget {
            id,
            state: state,
            children: Vec::new(),
            needs_rebuild: true,
            builder: Box::new(|_| Vec::new()),
            on_message: Rc::new(on_message),
            create_element: Rc::new(create_element),
            focused_child_index: FocusState::NotFocused,
            is_focusable: false,
        }))
    }
    
    pub fn focusable_elemental(
        state: State,
        on_message: impl Fn(&mut Self, &Message) + 'static,
        create_element: impl Fn(&mut Self) -> (bool, Box<dyn Element>) + 'static,
    ) -> Component {
        let id = uid();
        // Register widget in tree (no parent yet, will be set when added as child)
        // tree::register(id, None);
        
        Rc::new(RefCell::new(Widget {
            id,
            state: state,
            children: Vec::new(),
            needs_rebuild: true,
            builder: Box::new(|_| Vec::new()),
            on_message: Rc::new(on_message),
            create_element: Rc::new(create_element),
            focused_child_index: FocusState::NotFocused,
            is_focusable: true,
        }))
    }
    
    fn _build(&mut self) -> (bool, Vec<Component>) {
        if !self.needs_rebuild && !self.children.is_empty() {
            // Return cached children if no rebuild needed
            (false, self.children.clone())
        } else {
            let new_children = (self.builder)(&self.state);
            self.needs_rebuild = false;
            (true, new_children)
        }
    }
    
    #[inline]
    pub fn set_state(&mut self, f: impl FnOnce(&mut State)) {
        f(&mut self.state);
        self.needs_rebuild = true;
    }
}

impl<T: 'static + Send + Sync> Widget<Task<T>> {
    pub fn future(
        task: impl Future<Output = T> + Send + Sync + 'static,
        on_message: impl Fn(&mut Self, &Message) -> MessageFlow + 'static,
        builder: impl Fn(&Task<T>) -> Component + 'static,
    ) -> Component {
        let id = uid();
        // Register widget in tree (no parent yet, will be set when added as child)
        // tree::register(id, None);
        
        Rc::new(RefCell::new(Widget {
            id,
            state: Task::Running(go(task)),
            children: Vec::new(),
            needs_rebuild: true,
            builder: Box::new(move |state| vec![builder(state)]),
            on_message: Rc::new(move |this, msg| {
                switch(msg).case(|&Tick(_)| {
                    if this.state.check() {
                        this.set_state(|_| {});
                    }
                });
                if let Propagate = on_message(this, msg) {
                    // Propagate to all children
                    for child in &this.children {
                        child.borrow_mut().on_message(msg);
                    }
                }
            }),
            create_element: Rc::new(create_child_always_replace),
            focused_child_index: FocusState::NotFocused,
            is_focusable: false,
        }))
    }
}

impl<T: 'static + Send + Sync, TaskRet: Send + Sync + 'static> Widget<Stream<T, TaskRet>> {
    pub fn stream<F: Future<Output = TaskRet> + Send + Sync + 'static>(
        generator: impl FnOnce(UnboundedSender<T>) -> F,
        on_message: impl Fn(&mut Self, &Message) -> MessageFlow + 'static,
        builder: impl Fn(&Stream<T, TaskRet>) -> Component + 'static,
    ) -> Component {
        let id = uid();
        // Register widget in tree (no parent yet, will be set when added as child)
        // tree::register(id, None);
        
        let (sender, receiver) = unbounded_channel();
        Rc::new(RefCell::new(Widget {
            id,
            state: Stream {
                task: Task::Running(go(generator(sender))),
                receiver,
                current: None,
            },
            children: Vec::new(),
            needs_rebuild: true,
            builder: Box::new(move |state| vec![builder(state)]),
            on_message: Rc::new(move |this, msg| {
                switch(msg).case(|&Tick(_)| {
                    if this.state.check() {
                        this.set_state(|_| {});
                    }
                });
                if let Propagate = on_message(this, msg) {
                    // Propagate to all children
                    for child in &this.children {
                        child.borrow_mut().on_message(msg);
                    }
                }
            }),
            create_element: Rc::new(create_child_always_replace),
            focused_child_index: FocusState::NotFocused,
            is_focusable: false,
        }))
    }
}

/// Reconciliation algorithm for widget children using Vec
/// Returns (did_rebuild, updated_children)
fn reconcile_children_vec(
    old_children: &mut Vec<Component>,
    new_children: Vec<Component>,
) -> (bool, Vec<Component>) {
    let mut did_rebuild = false;
    let mut updated_children = Vec::with_capacity(new_children.len());
    
    for (i, new_child) in new_children.into_iter().enumerate() {
        if i < old_children.len() {
            // Compare IDs at same index
            let old_id = old_children[i].borrow().id();
            let new_id = new_child.borrow().id();
            
            //if old_id == new_id {
                // Same component, reuse it
                updated_children.push(old_children[i].clone());
            //} else {
                // Different component, replace it
            //    did_rebuild = true;
            //    updated_children.push(new_child);
            //}
        } else {
            // New child beyond current length
            did_rebuild = true;
            updated_children.push(new_child);
        }
    }
    
    // If old children had more elements than new children, we need to rebuild
    if old_children.len() > updated_children.len() {
        did_rebuild = true;
        // Truncate old_children to match new length
        old_children.truncate(updated_children.len());
    }
    
    // Replace old children with updated ones
    *old_children = updated_children.clone();
    
    (did_rebuild, updated_children)
}

fn create_child_always_replace<T: 'static>(this: &mut Widget<T>) -> (bool, Box<dyn Element>) {
    let (did_build, new_children) = this._build();
    
    // Reconcile children
    //let (did_reconcile, reconciled_children) = reconcile_children_vec(
    //    &mut this.children,
    //    new_children,
    //);
    
    // Update widget's children
    this.children = new_children;
    
    // Create elements for all children
    let mut child_elements = Vec::new();
    let mut any_child_rebuilt = false;
    
    for child in &this.children {
        let (child_did_rebuild, child_element) = child.borrow_mut().create_element();
        child_elements.push(child_element);
        any_child_rebuilt = any_child_rebuilt || child_did_rebuild;
    }
    
    // For now, we'll return a simple container element
    // In a real implementation, this would create the appropriate element type
    let did_any_rebuild = did_build || any_child_rebuilt;
    (did_any_rebuild, Box::new(SimpleContainer { children: child_elements }))
}

fn create_children<T, F>(create_element: F) -> impl Fn(&mut Widget<T>) -> (bool, Box<dyn Element>)
where
    T: 'static,
    F: Fn(&mut Widget<T>, Vec<Box<dyn Element>>) -> (bool, Box<dyn Element>) + 'static,
{
    move |this: &mut Widget<T>| {
        let (did_build, new_children) = this._build();
        
        // Reconcile children
        let (did_reconcile, reconciled_children) = reconcile_children_vec(
            &mut this.children,
            new_children,
        );
        
        // Update widget's children
        this.children = reconciled_children;
        
        // Create elements for all children
        let mut child_elements = Vec::new();
        let mut any_child_rebuilt = false;
        
        for child in &this.children {
            let (child_did_rebuild, child_element) = child.borrow_mut().create_element();
            child_elements.push(child_element);
            any_child_rebuilt = any_child_rebuilt || child_did_rebuild;
        }
        
        // Call the custom create_element with child_elements
        let (custom_did_rebuild, custom_element) = create_element(this, child_elements);
        
        // Combine rebuild flags
        let did_any_rebuild = did_build || did_reconcile || any_child_rebuilt || custom_did_rebuild;
        (did_any_rebuild, custom_element)
    }
}

/// Simple container element for demonstration
/// In real code, this would be replaced with proper element types
struct SimpleContainer {
    children: Vec<Box<dyn Element>>,
}

impl Element for SimpleContainer {
    fn draw(&self, size: crate::prelude::Size, display_list: &mut crate::prelude::DisplayList) {
        // Simple implementation: draw all children
        for child in &self.children {
            child.draw(size, display_list);
        }
    }
}

impl<State> _Component for Widget<State> {
    #[inline]
    fn id(&self) -> usize {
        self.id
    }
    
    #[inline]
    fn create_element(&mut self) -> (bool, Box<dyn Element>) {
        (self.create_element.clone())(self)
    }
    
    #[inline]
    fn on_message(&mut self, event: &Message) {
        (self.on_message.clone())(self, event);
    }
    
    fn change_focus(&mut self, dir: Dir) -> bool {
        //eprintln!("change_focus entered: {}, is_focusable: {}, numchild: {}", self.id, self.is_focusable, self.children.len());
        if self.is_focusable && self.focused_child_index == FocusState::NotFocused {
            self.focused_child_index = FocusState::SelfFocused;
            return true;
        }
        
        // Clear current focus
        let old_focus = std::mem::replace(&mut self.focused_child_index, FocusState::NotFocused);

        if self.children.len() == 0 {
            return false;
        }
        
        let starting_idx = if let FocusState::ChildFocused(idx) = old_focus {
            idx
        } else {
            match dir {
                Dir::Positive => 0,
                Dir::Negative => self.children.len() - 1,
            }
        };

        let mut idx = starting_idx;
        while 0 <= idx && idx < self.children.len() {
            self.focused_child_index = FocusState::ChildFocused(idx);
            if self.children[idx].borrow_mut().change_focus(dir) {
                return true;
            }
            if idx == 0 && dir == Dir::Negative {
                return false;
            }
            idx = match dir {
                Dir::Positive => idx + 1,
                Dir::Negative => idx - 1,
            }
        }

        return false;
    }
}

pub fn propagate(this: &mut Widget<Vec<Component>>, msg: &Message) {
    this.children
        .iter()
        .for_each(|child| child.borrow_mut().on_message(msg));
}
