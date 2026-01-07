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
    pub use super::{Widget, propagate, Focusable, FocusableComponent};
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

pub struct Widget<State, C = dyn _Component>
where
    C: _Component + ?Sized,
{
    id: usize,
    pub state: State, // pub: can modify without set state
    pub children: Vec<Rc<RefCell<C>>>, // is pub bad?
    needs_rebuild: bool,
    builder: Box<dyn Fn(&State) -> Rc<RefCell<C>>>,
    on_message: Rc<dyn Fn(&mut Self, &Message)>,
    create_element: Rc<dyn Fn(&mut Self) -> (bool, Box<dyn Element>)>,
}

impl<State, C> Debug for Widget<State, C>
where
    C: _Component + ?Sized,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Widget {{ id: {} }}", self.id)
    }
}

/*impl Widget<()> { // stateless
    pub fn containerlike( // have children, no state, auto propagate
        children: Vec<Component>,
        on_message: impl Fn(&mut Self, &Message) -> MessageFlow + 'static,
        create_element: impl Fn(&mut Self) -> (bool, Box<dyn Element>) + 'static,
    ) -> Component {
        Rc::new(RefCell::new(Widget {
            id: uid(),
            state: (),
            children,
            needs_rebuild: false,
            builder: Box::new(|_| panic!()),
            on_message: Rc::new(move |this, msg| {
                let flow = on_message(this, msg);
                match flow {
                    MessageFlow::Propagate => propagate(this, msg),
                    MessageFlow::Intercept => {}
                }
            }),
            create_element: Rc::new(create_element),
        }))
    }
}*/

impl<State, C> Widget<State, C>
where
    State: 'static,
    C: _Component + ?Sized + 'static
{
    pub fn containerlike( // have children, have state, auto propagate
        state: State,
        children: Vec<Rc<RefCell<C>>>,
        on_message: impl Fn(&mut Self, &Message) -> MessageFlow + 'static,
        create_element: impl Fn(&mut Self) -> (bool, Box<dyn Element>) + 'static,
    ) -> Component {
        let widget = Widget {
            id: uid(),
            state,
            children,
            needs_rebuild: false,
            builder: Box::new(|_| panic!()),
            on_message: Rc::new(move |this, msg| {
                let flow = on_message(this, msg);
                match flow {
                    MessageFlow::Propagate => propagate(this, msg),
                    MessageFlow::Intercept => {}
                }
            }),
            create_element: Rc::new(create_element),
        };
        Rc::new(RefCell::new(widget)) as Component
    }
}
impl<State> Widget<State, dyn _Component>
where
    State: 'static,
{
    pub fn stateful( // no children? (for now)
        state: State,
        on_message: impl Fn(&mut Self, &Message) -> MessageFlow + 'static,
        builder: impl Fn(&State) -> Component + 'static,
    ) -> Component {
        Rc::new(RefCell::new(Widget {
            id: uid(),
            state: state,
            children: Vec::new(),
            needs_rebuild: false,
            builder: Box::new(builder),
            on_message: Rc::new(move |this, msg| {
                on_message(this, msg); // no children for now
            }),
            create_element: Rc::new(create_child), // currently just roundabout way to create elem
        }))
    }
    pub fn elemental( // no children
        state: State,
        on_message: impl Fn(&mut Self, &Message) + 'static,
        create_element: impl Fn(&mut Self) -> (bool, Box<dyn Element>) + 'static,
    ) -> Component {
        Rc::new(RefCell::new(Widget {
            id: uid(),
            state: state,
            children: Vec::new(),
            needs_rebuild: false,
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
            children: Vec::new(),
            needs_rebuild: false,
            builder: Box::new(builder),
            on_message: Rc::new(move |this, msg| {
                switch(msg).case(|&Tick(_)| {
                    if this.state.check() {
                        this.set_state(|_| {});
                    }
                });
                on_message(this, msg); // no children for now
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
            children: Vec::new(),
            needs_rebuild: false,
            builder: Box::new(builder),
            id: uid(),
            on_message: Rc::new(move |this, msg| {
                switch(msg).case(|&Tick(_)| {
                    if this.state.check() {
                        this.set_state(|_| {});
                    }
                });
                on_message(this, msg); // no children for now
            }),
            create_element: Rc::new(create_child),
        }))
    }
}

impl<State, C> _Component for Widget<State, C>
where
    C: _Component + ?Sized,
{
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
    //#[inline]
    //fn needs_rebuild(&mut self) -> bool {
    //    self.needs_rebuild
    //}
}

pub fn propagate<State, C>(this: &mut Widget<State, C>, msg: &Message)
where
    C: _Component + ?Sized,
{
    this.children
        .iter()
        .for_each(|child| child.borrow_mut().on_message(msg));
}

// ------------- focus ------------------

pub trait Focusable {
    fn create_focused_element(&mut self) -> (bool, Box<dyn Element>);
}

pub trait FocusableComponent: _Component + Focusable {}   // need for it has error when dyn _Component + Focusable

impl<State> Widget<State, dyn FocusableComponent>  // find how to not do this every time
where
    State: 'static,
{
    #[inline]
    pub fn set_state(&mut self, f: impl FnOnce(&mut State)) {
        f(&mut self.state);
        self.needs_rebuild = true;
    }
}