use std::{cell::RefCell, fmt::Debug, ops::RangeFrom, rc::Rc};

use stdext::prelude::switch;
use tokio::sync::mpsc::{UnboundedSender, unbounded_channel};

use crate::{
    prelude::*,
    component::prelude::*,
    message::prelude::*,
    prelude::Element,
    render::Tick,
    runtime::{Stream, Task, go},
    prelude::Size,
};

pub mod prelude {
    pub use super::{Widget, propagate, ComponentExt};
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
    prev: Option<Component>,
    needs_rebuild: bool,
    builder: Box<dyn Fn(&State) -> Component>,
    on_message: Rc<dyn Fn(&mut Self, &Message)>,
    create_element: Rc<dyn Fn(&mut Self) -> (bool, Box<dyn Element>)>,
}

impl<State> Debug for Widget<State> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Contains prev: {}; Child: {:?}",
            self.prev.is_some(),
            self.prev
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
        Rc::new(RefCell::new(Widget {
            id: uid(),
            state: state,
            prev: None,
            needs_rebuild: true,
            builder: Box::new(builder),
            on_message: Rc::new(move |this, msg| {
                if let Propagate = on_message(this, msg)
                    && let Some(prev) = &this.prev
                {
                    prev.borrow_mut().on_message(msg);
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
            prev: None,
            needs_rebuild: true,
            builder: Box::new(|_| panic!()),
            on_message: Rc::new(on_message),
            create_element: Rc::new(create_element),
        }))
    }
    fn _build(&mut self) -> (bool, Component) {
        if !self.needs_rebuild
            && let Some(prev) = &self.prev
        {
            (false, prev.clone())
        } else {
            let new_widget = (self.builder)(&self.state);
            self.prev = Some(new_widget.clone());
            self.needs_rebuild = false;
            (true, new_widget)
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
        Rc::new(RefCell::new(Widget {
            id: uid(),
            state: Task::Running(go(task)),
            prev: None,
            needs_rebuild: true,
            builder: Box::new(builder),
            on_message: Rc::new(move |this, msg| {
                switch(msg).case(|&Tick(_)| {
                    if this.state.check() {
                        this.set_state(|_| {});
                    }
                });
                if let Propagate = on_message(this, msg)
                    && let Some(prev) = &this.prev
                {
                    prev.borrow_mut().on_message(msg);
                }
            }),
            create_element: Rc::new(create_child),
        }))
    }
}

fn create_child<T: 'static>(this: &mut Widget<T>) -> (bool, Box<dyn Element>) {
    let (did_rebuild, widget) = this._build();
    let (did_child_rebuild, child_element) = widget.borrow_mut().create_element();
    (did_rebuild || did_child_rebuild, child_element)
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
            prev: None,
            needs_rebuild: true,
            builder: Box::new(builder),
            id: uid(),
            on_message: Rc::new(move |this, msg| {
                switch(msg).case(|&Tick(_)| {
                    if this.state.check() {
                        this.set_state(|_| {});
                    }
                });
                if let Propagate = on_message(this, msg)
                    && let Some(prev) = &this.prev
                {
                    prev.borrow_mut().on_message(msg);
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

pub trait ComponentExt {
    fn constraint(self, width: isize, height: isize) -> Component;
}

impl ComponentExt for Component {
    fn constraint(self, width: isize, height: isize) -> Component {
        Widget::elemental(
            self,
            |this, msg| {
                this.state.borrow_mut().on_message(msg);
            },
            move |this| {
                let (did_rebuild, inner_element) = this.state.borrow_mut().create_element();
                (
                    did_rebuild,
                    Box::new(ConstraintElement {
                        inner: inner_element,
                        size: Size { x: width, y: height },
                    }),
                )
            },
        )
    }
}

struct ConstraintElement {
    inner: Box<dyn Element>,
    size: Size,
}

impl Element for ConstraintElement {
    fn draw(&self, constraint: Size, display_list: &mut DisplayList) {
        self.inner.draw(constraint, display_list);
    }

    fn preferred_size(&self) -> Option<Size> {
        Some(self.size)
    }
}