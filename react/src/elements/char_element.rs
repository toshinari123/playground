use crate::prelude::{Constraint2, DisplayList, Element, Operation, Pixel};

pub mod prelude {
    pub use super::CharElement;
}

pub struct CharElement {
    pub c: char,
}

impl Element for CharElement {
    fn propose_size(&self, _: Constraint2) -> Constraint2 {
        Constraint2 {
            x: Some(Pixel(1)),
            y: Some(Pixel(1)),
        }
    }
    fn draw(&self, constraint: Constraint2, display_list: &mut DisplayList) {
        constraint.is_pixels_or_none_then(|constraint| match (constraint.x, constraint.y) {
            (Some(Pixel(0)), Some(Pixel(0))) => {}
            _ => display_list.0.push(Operation::PutChar(self.c)),
        });
    }
}
