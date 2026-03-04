use crate::prelude::{DisplayList, Element, Operation, Point, Size};

pub mod prelude {
    pub use super::StackElement;
}

pub struct StackElement {
    pub children: Vec<Box<dyn Element>>,
}

impl Element for StackElement {
    fn draw(&self, constraint: Size, display_list: &mut DisplayList) {
        for child in &self.children {
            display_list.0.push(Operation::MoveTo(Point::default()));
            child.draw(constraint, display_list);
        }
    }

    fn preferred_size(&self) -> Option<Size> {
        let mut max_width: Option<isize> = None;
        let mut max_height: Option<isize> = None;

        for child in &self.children {
            if let Some(size) = child.preferred_size() {
                max_width = Some(max_width.map_or(size.x, |w| w.max(size.x)));
                max_height = Some(max_height.map_or(size.y, |h| h.max(size.y)));
            }
        }

        match (max_width, max_height) {
            (Some(w), Some(h)) => Some(Size { x: w, y: h }),
            _ => None,
        }
    }
}