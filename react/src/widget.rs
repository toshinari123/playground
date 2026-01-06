use std::{cell::RefCell, fmt::Debug, ops::RangeFrom, rc::Rc};

use stdext::prelude::switch;
use tokio::sync::mpsc::{UnboundedSender, unbounded_channel};

use crate::{
    component::prelude::*,
    message::prelude::*,
    prelude::Element,
    render::Tick,
    runtime::{Stream, Task, go},
};

pub mod prelude {
    pub use super::{Widget, propagate};
}

thread_local! {
    pub(crate) static COUNTER: RefCell<RangeFrom<usize>> = RefCell::new(0..);
}

pub fn uid() -> usize {
    COUNTER.with(|counter| {
        counter
            .borrow_mut()
            .next()
            .expect("Ran out of UIDs for Widgets")
    })
}

pub struct Widget<State> {
    id: usize,
    pub state: State,
    builder: Box<dyn Fn(&State) -> Component>,
    on_message: Rc<dyn Fn(&mut Self, &Message)>,
    create_element: Rc<dyn Fn(&mut Self) -> (bool, Box<dyn Element>)>,
}

impl<State> Debug for Widget<State> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Widget {{ id: {} }}", self.id)
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
        Rc::new(RefCell::new(Widget {
            id: uid(),
            state: state,
            builder: Box::new(builder),
            on_message: Rc::new(move |this, msg| {
                if let Propagate = on_message(this, msg) {
                    // Messages propagate through the widget tree naturally
                    // since we always rebuild with new widgets
                }
            }),
            create_element: Rc::new(create_child),
        }))
    }
    pub fn elemental(
        state: State,
        on_message: impl Fn(&mut Self, &Message) + 'static,
        create_element: impl Fn(&mut Self) -> (bool, Box<dyn Element>) + 'static,
    ) -> Component {
        Rc::new(RefCell::new(Widget {
            id: uid(),
            state: state,
            builder: Box::new(|_| panic!()),
            on_message: Rc::new(on_message),
            create_element: Rc::new(create_element),
        }))
    }
    fn _build(&mut self) -> (bool, Component) {
        let new_widget = (self.builder)(&self.state);
        (true, new_widget)
    }
    #[inline]
    pub fn set_state(&mut self, f: impl FnOnce(&mut State)) {
        f(&mut self.state);
    }
}

impl<T: 'static + Send + Sync> Widget<Task<T>> {
    pub fn future(
        task: impl Future<Output = T> + Send + Sync + 'static,
        on_message: impl Fn(&mut Self, &Message) -> MessageFlow + 'static,
        builder: impl Fn(&Task<T>) -> Component + 'static,
    ) -> Component {
        Rc::new(RefCell::new(Widget {
            id: uid(),
            state: Task::Running(go(task)),
            builder: Box::new(builder),
            on_message: Rc::new(move |this, msg| {
                switch(msg).case(|&Tick(_)| {
                    if this.state.check() {
                        this.set_state(|_| {});
                    }
                });
                if let Propagate = on_message(this, msg) {
                    // Messages propagate through the widget tree naturally
                }
            }),
            create_element: Rc::new(create_child),
        }))
    }
}

fn create_child<T: 'static>(this: &mut Widget<T>) -> (bool, Box<dyn Element>) {
    let (_, widget) = this._build();
    let (did_child_rebuild, child_element) = widget.borrow_mut().create_element();
    (true || did_child_rebuild, child_element)
}

impl<T: 'static + Send + Sync, TaskRet: Send + Sync + 'static> Widget<Stream<T, TaskRet>> {
    pub fn stream<F: Future<Output = TaskRet> + Send + Sync + 'static>(
        generator: impl FnOnce(UnboundedSender<T>) -> F,
        on_message: impl Fn(&mut Self, &Message) -> MessageFlow + 'static,
        builder: impl Fn(&Stream<T, TaskRet>) -> Component + 'static,
    ) -> Component {
        let (sender, receiver) = unbounded_channel();
        Rc::new(RefCell::new(Widget {
            state: Stream {
                task: Task::Running(go(generator(sender))),
                receiver,
                current: None,
            },
            builder: Box::new(builder),
            id: uid(),
            on_message: Rc::new(move |this, msg| {
                switch(msg).case(|&Tick(_)| {
                    if this.state.check() {
                        this.set_state(|_| {});
                    }
                });
                if let Propagate = on_message(this, msg) {
                    // Messages propagate through the widget tree naturally
                }
            }),
            create_element: Rc::new(create_child),
        }))
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
