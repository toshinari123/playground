use crate::prelude::{DisplayList, Element, Operation, Point, Size};

pub mod prelude {
    pub use super::BoxWrappingElement;
}

//make enum here 

pub struct BoxWrappingElement {
    pub child: Box<dyn Element>,
}

impl Element for BoxWrappingElement {
    fn draw(&self, constraint: Size, display_list: &mut DisplayList) {
        // Need at least 3x3 to draw a box with content
        if constraint.x < 3 || constraint.y < 3 {
            return;
        }

        let width = constraint.x as isize;
        let height = constraint.y as isize;

        // Draw top border
        display_list.0.push(Operation::PutChar('┌'));
        for x in 1..(width - 1) {
            display_list.0.push(Operation::MoveTo(Point { x, y: 0 }));
            display_list.0.push(Operation::PutChar('─'));
        }
        display_list.0.push(Operation::MoveTo(Point { x: width - 1, y: 0 }));
        display_list.0.push(Operation::PutChar('┐'));

        // Draw side borders
        for y in 1..(height - 1) {
            // Left border
            display_list.0.push(Operation::MoveTo(Point { x: 0, y }));
            display_list.0.push(Operation::PutChar('│'));
            
            // Right border
            display_list.0.push(Operation::MoveTo(Point { x: width - 1, y }));
            display_list.0.push(Operation::PutChar('│'));
        }

        // Draw bottom border
        display_list.0.push(Operation::MoveTo(Point { x: 0, y: height - 1 }));
        display_list.0.push(Operation::PutChar('└'));
        for x in 1..(width - 1) {
            display_list.0.push(Operation::MoveTo(Point { x, y: height - 1 }));
            display_list.0.push(Operation::PutChar('─'));
        }
        display_list.0.push(Operation::MoveTo(Point { x: width - 1, y: height - 1 }));
        display_list.0.push(Operation::PutChar('┘'));

        // Draw child with offset (1, 1) and reduced constraints
        display_list.0.push(Operation::SetAnchor(Point { x: 1, y: 1 }));
        self.child.draw(
            Size {
                x: constraint.x - 2,
                y: constraint.y - 2,
            },
            display_list,
        );
        display_list.0.push(Operation::SetAnchor(Point { x: -1, y: -1 }));
    }
}
