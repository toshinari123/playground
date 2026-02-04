use std::{cell::RefCell, collections::HashMap, fmt::Debug, ops::RangeFrom, rc::Rc};

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
    pub use super::{Widget, propagate, single_child, children_with_indices};
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
    children: HashMap<String, Component>,
    needs_rebuild: bool,
    builder: Box<dyn Fn(&State) -> HashMap<String, Component>>,
    on_message: Rc<dyn Fn(&mut Self, &Message)>,
    create_element: Rc<dyn Fn(&mut Self) -> (bool, Box<dyn Element>)>,
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
            children: HashMap::new(),
            needs_rebuild: true,
            builder: Box::new(move |state| single_child(builder(state))),
            on_message: Rc::new(move |this, msg| {
                if let Propagate = on_message(this, msg) {
                    // Propagate to all children
                    for child in this.children.values() {
                        child.borrow_mut().on_message(msg);
                    }
                }
            }),
            create_element: Rc::new(create_child),
        }))
    }
    
    pub fn stateful_container(
        state: State,
        on_message: impl Fn(&mut Self, &Message) -> MessageFlow + 'static,
        builder: impl Fn(&State) -> HashMap<String, Component> + 'static,
        create_element: impl Fn(&mut Self, Vec<(String, Box<dyn Element>)>) -> (bool, Box<dyn Element>) + 'static,
    ) -> Component {
        let id = uid();
        // Register widget in tree (no parent yet, will be set when added as child)
        // tree::register(id, None);
        
        Rc::new(RefCell::new(Widget {
            id,
            state: state,
            children: HashMap::new(),
            needs_rebuild: true,
            builder: Box::new(builder),
            on_message: Rc::new(move |this, msg| {
                if let Propagate = on_message(this, msg) {
                    // Propagate to all children
                    for child in this.children.values() {
                        child.borrow_mut().on_message(msg);
                    }
                }
            }),
            create_element: Rc::new(create_children(create_element)),
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
            children: HashMap::new(),
            needs_rebuild: true,
            builder: Box::new(|_| HashMap::new()),
            on_message: Rc::new(on_message),
            create_element: Rc::new(create_element),
        }))
    }
    
    fn _build(&mut self) -> (bool, HashMap<String, Component>) {
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
            children: HashMap::new(),
            needs_rebuild: true,
            builder: Box::new(move |state| single_child(builder(state))),
            on_message: Rc::new(move |this, msg| {
                switch(msg).case(|&Tick(_)| {
                    if this.state.check() {
                        this.set_state(|_| {});
                    }
                });
                if let Propagate = on_message(this, msg) {
                    // Propagate to all children
                    for child in this.children.values() {
                        child.borrow_mut().on_message(msg);
                    }
                }
            }),
            create_element: Rc::new(create_child),
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
            children: HashMap::new(),
            needs_rebuild: true,
            builder: Box::new(move |state| single_child(builder(state))),
            on_message: Rc::new(move |this, msg| {
                switch(msg).case(|&Tick(_)| {
                    if this.state.check() {
                        this.set_state(|_| {});
                    }
                });
                if let Propagate = on_message(this, msg) {
                    // Propagate to all children
                    for child in this.children.values() {
                        child.borrow_mut().on_message(msg);
                    }
                }
            }),
            create_element: Rc::new(create_child),
        }))
    }
}

/// Reconciliation algorithm for widget children
/// Returns (did_rebuild, updated_children)
fn reconcile_children(
    parent_id: usize,
    old_children: &mut HashMap<String, Component>,
    new_children: HashMap<String, Component>,
) -> (bool, HashMap<String, Component>) {
    let mut did_rebuild = false;
    let mut updated_children = HashMap::new();
    
    // Process new children
    for (key, new_child) in new_children {
        if let Some(old_child) = old_children.remove(&key) {
            // Compare IDs
            let old_id = old_child.borrow().id();
            let new_id = new_child.borrow().id();
            
                updated_children.insert(key.clone(), old_child);
            
            // Update tree registry with the child we're actually using
            let child_id = updated_children.get(&key).unwrap().borrow().id();
            // tree::add_child(parent_id, key.clone(), child_id);
        } else {
            // New key, add new component
            did_rebuild = true;
            updated_children.insert(key.clone(), new_child.clone());
            
            // Update tree registry
            // tree::add_child(parent_id, key.clone(), new_child.borrow().id());
        }
    }
    
    // Remove old children that are no longer present
    for (key, old_child) in old_children.drain() {
        did_rebuild = true;
        // Remove from tree registry
        // tree::remove_child(parent_id, &key);
    }
    
    (did_rebuild, updated_children)
}

fn create_child<T: 'static>(this: &mut Widget<T>) -> (bool, Box<dyn Element>) {
    let (did_build, new_children) = this._build();
    
    // Reconcile children
    // let (did_reconcile, reconciled_children) = reconcile_children(
    //     this.id,
    //     &mut this.children,
    //     new_children,
    // );
    
    // Update widget's children
    this.children = new_children;
    
    // Create elements for all children
    let mut child_elements = Vec::new();
    let mut any_child_rebuilt = false;
    
    for (key, child) in &this.children {
        let (child_did_rebuild, child_element) = child.borrow_mut().create_element();
        child_elements.push((key.clone(), child_element));
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
    F: Fn(&mut Widget<T>, Vec<(String, Box<dyn Element>)>) -> (bool, Box<dyn Element>) + 'static,
{
    move |this: &mut Widget<T>| {
        let (did_build, new_children) = this._build();
        
        // Reconcile children
        let (did_reconcile, reconciled_children) = reconcile_children(
            this.id,
            &mut this.children,
            new_children,
        );
        
        // Update widget's children
        this.children = reconciled_children;
        
        // Create elements for all children
        let mut child_elements = Vec::new();
        let mut any_child_rebuilt = false;
        
        for (key, child) in &this.children {
            let (child_did_rebuild, child_element) = child.borrow_mut().create_element();
            child_elements.push((key.clone(), child_element));
            any_child_rebuilt = any_child_rebuilt || child_did_rebuild;
        }
        child_elements.sort_by(|(a, _), (c, _)| a.cmp(c));
        
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
    children: Vec<(String, Box<dyn Element>)>,
}

impl Element for SimpleContainer {
    fn draw(&self, size: crate::prelude::Size, display_list: &mut crate::prelude::DisplayList) {
        // Simple implementation: draw all children
        for (_, child) in &self.children {
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
}

pub fn propagate(this: &mut Widget<Vec<Component>>, msg: &Message) {
    this.state
        .iter()
        .for_each(|child| child.borrow_mut().on_message(msg));
}

/// Helper function to convert a single component to a hashmap with empty key
pub fn single_child(child: Component) -> HashMap<String, Component> {
    let mut map = HashMap::new();
    map.insert("".to_string(), child);
    map
}

/// Helper function to convert a vector of components to a hashmap with index keys
pub fn children_with_indices(children: Vec<Component>) -> HashMap<String, Component> {
    children
        .into_iter()
        .enumerate()
        .map(|(i, child)| (i.to_string(), child))
        .collect()
}
