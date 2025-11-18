use std::{cell::RefCell, rc::Rc};

use stdext::prelude::switch;
use tokio::{
    sync::mpsc::{UnboundedReceiver, UnboundedSender, unbounded_channel},
    task::{JoinError, JoinHandle},
};

use crate::{
    component::prelude::*,
    message::prelude::*,
    prelude::Element,
    render::Tick,
    runtime::{Stream, Task, go},
};

pub mod prelude {
    pub use super::{Widget, propagate_msg, statelessly_childfully_create_element_functional};
}

thread_local! {
    pub(crate) static COUNTER: RefCell<usize> = RefCell::new(0);
}

pub fn uid() -> usize {
    COUNTER.with(|counter| {
        let id = *counter.borrow();
        *counter.borrow_mut() += 1;
        id
    })
}

/*   for full imp use generic? on one hand most of the time its Vec on other hand if
 *   really need to be generic will be annoying to change in the future

pub struct Widget<State, Children>
where
    Children: IntoIterator<Item = Component>,
{

*/

pub type Children = Vec<Component>;

pub struct Widget<State>
{
    id: usize,
    pub state: State,
    children: Children,
    prev: Option<Component>,
    needs_rebuild: bool,
    builder: Box<dyn Fn(&State) -> Component>,
    on_message: Rc<dyn Fn(&mut Self, &Message)>,
    create_element: Rc<dyn Fn(&mut Self) -> (bool, Box<dyn Element>)>,
}

impl<State> Widget<State>
where
    State: 'static,
{
    pub fn stateful( // CURRENTLY CHILDLESS
        state: State,
        on_message: impl Fn(&mut Self, &Message) -> MessageFlow + 'static,
        builder: impl Fn(&State) -> Component + 'static,
    ) -> Component {
        Rc::new(RefCell::new(Widget {
            id: uid(),
            state: state,
            children: vec![],
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
            create_element: Rc::new(statefully_childlessly_create_element),
        }))
    }
    pub fn elemental( // change to nonstateful? actually isnt this just stateful() above but assume
                      // set_state is never called like the panic wont panic cuz _build is not used 
                      // here anyways (_build is used in create_child which is not used here)
        state: State,
        children: Children,
        on_message: impl Fn(&mut Self, &Message) + 'static,
        create_element: impl Fn(&mut Self) -> (bool, Box<dyn Element>) + 'static,
    ) -> Component {
        Rc::new(RefCell::new(Widget {
            id: uid(),
            state: state,
            children: children,
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
            _ = self.prev.take();
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
            children: vec![],
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
            create_element: Rc::new(statefully_childlessly_create_element),
        }))
    }
}

fn statefully_childlessly_create_element<T: 'static>(this: &mut Widget<T>) -> (bool, Box<dyn Element>) {
    let (did_rebuild, widget) = this._build();
    let (did_elem_rebuild, element) = widget.borrow_mut().create_element();
    (did_rebuild || did_elem_rebuild, element)
}

pub fn statelessly_childfully_create_element_functional<T: 'static>
    (element_creator: Box<dyn Fn(Vec<Box<dyn Element>>) -> Box<dyn Element>>) 
    -> Box<dyn Fn(&mut Widget<T>) -> (bool, Box<dyn Element>)>
{
    Box::new(move |this| {
        let (did_rebuilds, children_elems): (Vec<_>, Vec<_>) = this
            .children
            .iter()
            .map(|child| child.borrow_mut().create_element())
            .unzip();
        (
            did_rebuilds.iter().any(|&did_rebuild| did_rebuild),
            element_creator(children_elems),
        )
    })
}

impl<T: 'static + Send + Sync, TaskRet: Send + Sync + 'static> Widget<Stream<T, TaskRet>> {
    pub fn stream<F: Future<Output = TaskRet> + Send + Sync + 'static>(
        generator: impl FnOnce(UnboundedSender<T>) -> F,
        on_message: impl Fn(&mut Self, &Message) -> MessageFlow + 'static,
        builder: impl Fn(&Stream<T, TaskRet>) -> Component + 'static,
    ) -> Component {
        let (sender, receiver) = unbounded_channel();
        Rc::new(RefCell::new(Widget {
            id: uid(),
            state: Stream {
                task: Task::Running(go(generator(sender))),
                receiver,
                current: None,
            },
            children: vec![],
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
            create_element: Rc::new(statefully_childlessly_create_element),
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

pub fn propagate_msg<State> (this: &mut Widget<State>, msg: &Message) {
    this.children
        .iter()
        .for_each(|child| child.borrow_mut().on_message(msg));
}
