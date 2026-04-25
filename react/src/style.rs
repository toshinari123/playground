use crate::{
    prelude::{Component, Pipe, Pipeline, SizedElement, Widget, Constraint2, Constraint},
};

pub mod prelude {
    pub use super::{Style, Color, sized, width, height};
}

pub trait Style {
    fn style(self, styles: impl Pipeline<Component, Component>) -> Component;
}

impl Style for Component {
    fn style(self, styles: impl Pipeline<Component, Component>) -> Component {
        self.pipe(styles)
    }
}

pub fn sized(x: Option<Constraint>, y: Option<Constraint>) -> impl Fn(Component) -> Component + Clone {
    move |child| {
        Widget::elemental(
            child,
            |this, msg| this.state.borrow_mut().on_message(msg),
            move |this| {
                let (did_child_rebuild, child) = this.state.borrow_mut().create_element();
                (did_child_rebuild, Box::new(SizedElement { size: Constraint2 { x, y }, child }))
            },
        )
    }
}

pub fn width(x: Constraint) -> impl Fn(Component) -> Component + Clone {
    sized(Some(x), None)
}

pub fn height(y: Constraint) -> impl Fn(Component) -> Component + Clone {
    sized(None, Some(y))
}

#[derive(Debug, Clone, Copy, Hash)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8
}