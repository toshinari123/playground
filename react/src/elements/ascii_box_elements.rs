use crate::prelude::*;

pub mod prelude {
    pub use super::AsciiBoxElement;
}

pub struct AsciiBoxElement {
    inner: Box<dyn Element>,
}

impl AsciiBoxElement {
    /// Create a new ASCII box element wrapping the given inner element
    pub fn new(inner: Box<dyn Element>) -> Self {
        Self { inner }
    }
}

impl Element for AsciiBoxElement {
    fn draw(&self, size: Size, display_list: &mut DisplayList) {
        // If the size is too small to draw a proper border, just draw the inner element
        if size.x < 2 || size.y < 2 {
            self.inner.draw(size, display_list);
            return;
        }
        
        // Draw top border
        display_list.0.push(Operation::PutChar('+'));
        for x in 1..size.x - 1 {
            display_list.0.push(Operation::MoveTo(Point { x: x as isize, y: 0 }));
            display_list.0.push(Operation::PutChar('-'));
        }
        display_list.0.push(Operation::MoveTo(Point { x: size.x as isize - 1, y: 0 }));
        display_list.0.push(Operation::PutChar('+'));
        
        // Draw side borders
        for y in 1..size.y - 1 {
            display_list.0.push(Operation::MoveTo(Point { x: 0, y: y as isize }));
            display_list.0.push(Operation::PutChar('|'));
            display_list.0.push(Operation::MoveTo(Point { x: size.x as isize - 1, y: y as isize }));
            display_list.0.push(Operation::PutChar('|'));
        }
        
        // Draw bottom border
        display_list.0.push(Operation::MoveTo(Point { x: 0, y: size.y as isize - 1 }));
        display_list.0.push(Operation::PutChar('+'));
        for x in 1..size.x - 1 {
            display_list.0.push(Operation::MoveTo(Point { x: x as isize, y: size.y as isize - 1 }));
            display_list.0.push(Operation::PutChar('-'));
        }
        display_list.0.push(Operation::MoveTo(Point { x: size.x as isize - 1, y: size.y as isize - 1 }));
        display_list.0.push(Operation::PutChar('+'));
        
        // Draw inner content (offset by 1,1 to account for border)
        let inner_size = Size { x: size.x - 2, y: size.y - 2};
        // We need to create a new display list for the inner content with offset
        let mut inner_display_list = DisplayList::default();
        self.inner.draw(inner_size, &mut inner_display_list);
        
        // Apply offset to all operations in the inner display list
        for op in inner_display_list.0 {
            match op {
                Operation::MoveTo(mut point) => {
                    point.x += 1;
                    point.y += 1;
                    display_list.0.push(Operation::MoveTo(point));
                }
                Operation::SetAnchor(mut point) => {
                    point.x += 1;
                    point.y += 1;
                    display_list.0.push(Operation::SetAnchor(point));
                }
                other => display_list.0.push(other),
            }
        }
    }
}
