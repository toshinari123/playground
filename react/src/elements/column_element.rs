use crate::prelude::{DisplayList, Element, Operation, Point, Size, Direction};

pub mod prelude {
    pub use super::ColumnElement;
}

pub struct ColumnElement {
    pub children: Vec<Box<dyn Element>>,
}

impl Element for ColumnElement {
    fn draw(&self, constraint: Size, display_list: &mut DisplayList) {
        let child_height = constraint.y as usize / self.children.len();
        let mut y_offset = 0;
        
        // draw border
        draw_column_border(constraint, display_list);

        // drawing each child
        for child in &self.children {
            let offset = Point {
                x: 0,
                y: y_offset as isize,
            };

            // move down to child position
            display_list.0.push(Operation::SetAnchor(offset));

            // draw child
            child.draw(
                Size {
                    x: constraint.x,
                    y: child_height as isize,
                },
                display_list,
            );

            // move back to original position
            display_list.0.push(Operation::SetAnchor(-offset));

            y_offset += child_height;
        }
    }
}                      

fn draw_column_border(constraint: Size, display_list: &mut DisplayList) {
        // Draw top border
        display_list.0.push(Operation::PutChar('┌'));
        for _ in 1..constraint.x-1 {
            display_list.0.push(Operation::Move(Direction::End));
            display_list.0.push(Operation::PutChar('-'));
        }
        display_list.0.push(Operation::Move(Direction::End));
        display_list.0.push(Operation::PutChar('┐'));

        // Draw side borders
        for y in 1..constraint.y-1 {
            display_list.0.push(Operation::MoveTo(Point { x: 0, y }));
            display_list.0.push(Operation::PutChar('│'));
            display_list.0.push(Operation::MoveTo(Point { x: constraint.x-1, y }));
            display_list.0.push(Operation::PutChar('│'));
        }

        // Draw bottom border
        display_list.0.push(Operation::MoveTo(Point { x: 0, y: constraint.y-1 })); // y-1 because 0-indexed
        display_list.0.push(Operation::PutChar('└'));
        for _ in 1..constraint.x-1 {
            display_list.0.push(Operation::Move(Direction::End));
            display_list.0.push(Operation::PutChar('-'));
        }
        display_list.0.push(Operation::Move(Direction::End));
        display_list.0.push(Operation::PutChar('┘'));
    
    }