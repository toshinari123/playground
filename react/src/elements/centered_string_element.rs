use crate::prelude::{DisplayList, Element, Operation, Point, Size};

pub mod prelude {
    pub use super::CenteredStringElement;
}

pub struct CenteredStringElement {
    pub s: String,
}

impl Element for CenteredStringElement {
    fn draw(&self, constraint: Size, display_list: &mut DisplayList) {
        // Calculate string dimensions
        let mut width = 0;
        let mut height = 1;
        let mut current_line_width = 0;
        
        for c in self.s.chars() {
            if c == '\n' {
                height += 1;
                width = width.max(current_line_width);
                current_line_width = 0;
            } else {
                current_line_width += 1;
            }
        }
        // Check last line
        width = width.max(current_line_width);
        
        // If no content, nothing to draw
        if width == 0 || height == 0 {
            return;
        }
        
        // Calculate center offset
        let center_x = (constraint.x - width as isize) / 2;
        let center_y = (constraint.y - height as isize) / 2;
        
        // If string doesn't fit, don't draw
        if center_x < 0 || center_y < 0 {
            return;
        }
        
        // Set initial position
        let mut offset = Point { x: center_x, y: center_y };
        display_list.0.push(Operation::MoveTo(offset));
        
        // Draw string
        for c in self.s.chars() {
            if c == '\n' {
                offset.y += 1;
                offset.x = center_x;
                display_list.0.push(Operation::MoveTo(offset));
            } else {
                display_list.0.push(Operation::PutChar(c));
                offset.x += 1;
                // Update position for next character
                display_list.0.push(Operation::MoveTo(offset));
            }
        }
    }
}