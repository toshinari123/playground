use std::{cell::RefCell, fmt::Debug, rc::Rc};

use crate::{element::Element, prelude::Message};

pub mod prelude {
    pub use super::{_Component, Component, StaticComponent};
}

pub trait _Component: Debug {
    fn id(&self) -> usize;
    fn create_element(&mut self) -> (bool, Box<dyn Element>);
    fn on_message(&mut self, event: &Message);
}

pub type Component = Rc<RefCell<dyn _Component>>;
pub type StaticComponent = Rc<RefCell<dyn _Component + 'static>>;
