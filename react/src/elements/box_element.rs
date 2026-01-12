use crate::prelude::{DisplayList, Element, Operation, Point, Size};

pub mod prelude {
    pub use super::BoxElement;
}

pub struct BoxElement {
    pub child: Box<dyn Element>,
}

impl BoxElement {
    /// Draw a border around the given child element without taking ownership.
    /// This can be used when you need to conditionally draw a box around an element
    /// that you only have a reference to.
    pub fn draw_boxed(child: &dyn Element, constraint: Size, display_list: &mut DisplayList) {
        // If constraint is too small for a border, draw child without border
        if constraint.x < 3 || constraint.y < 3 {
            child.draw(constraint, display_list);
            return;
        }

        // Draw top border
        display_list.0.push(Operation::PutChar('┌'));
        for x in 1..constraint.x - 1 {
            display_list.0.push(Operation::MoveTo(Point { x, y: 0 }));
            display_list.0.push(Operation::PutChar('─'));
        }
        display_list.0.push(Operation::MoveTo(Point { x: constraint.x - 1, y: 0 }));
        display_list.0.push(Operation::PutChar('┐'));

        // Draw side borders
        for y in 1..constraint.y - 1 {
            display_list.0.push(Operation::MoveTo(Point { x: 0, y }));
            display_list.0.push(Operation::PutChar('│'));
            display_list.0.push(Operation::MoveTo(Point { x: constraint.x - 1, y }));
            display_list.0.push(Operation::PutChar('│'));
        }

        // Draw child with offset (1, 1) and reduced size
        let offset = Point { x: 1, y: 1 };
        display_list.0.push(Operation::SetAnchor(offset));
        child.draw(
            Size {
                x: constraint.x - 2,
                y: constraint.y - 2
            },
            display_list,
        );
        display_list.0.push(Operation::SetAnchor(-offset));

        // Draw bottom border
        display_list.0.push(Operation::MoveTo(Point { x: 0, y: constraint.y - 1 }));
        display_list.0.push(Operation::PutChar('└'));
        for x in 1..constraint.x - 1 {
            display_list.0.push(Operation::MoveTo(Point { x, y: constraint.y - 1 }));
            display_list.0.push(Operation::PutChar('─'));
        }
        display_list.0.push(Operation::MoveTo(Point { x: constraint.x - 1, y: constraint.y - 1 }));
        display_list.0.push(Operation::PutChar('┘'));
    }
}

impl Element for BoxElement {
    fn draw(&self, constraint: Size, display_list: &mut DisplayList) {
        Self::draw_boxed(&*self.child, constraint, display_list);
    }
}