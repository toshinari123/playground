use std::{cell::RefCell, fmt::Debug, rc::Rc};

use crate::prelude::{Element, Message};

#[derive(Clone, Copy, PartialEq)]
pub enum Dir {
    Positive = 1,
    Negative = -1,
}

pub mod prelude {
    pub use super::{_Component, Component, Dir, StaticComponent};
}

pub trait _Component: Debug {
    fn id(&self) -> usize;
    fn create_element(&mut self) -> (bool, Box<dyn Element>);
    fn on_message(&mut self, event: &Message);
    fn change_focus(&mut self, shift: Dir) -> crate::widget::FocusState;
}

pub type Component = Rc<RefCell<dyn _Component>>;
pub type StaticComponent = Rc<RefCell<dyn _Component + 'static>>;
