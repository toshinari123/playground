use crate::prelude::{DisplayList, Element, Frame, Operation, Point, Size, Direction};

pub mod prelude {
    pub use super::ContainerElement;
}

pub struct ContainerElement {
    pub children: Vec<Box<dyn Element>>,
}

struct Margin {
    u16,
    top: u16,
    bottom: u16,
    left: u16,
    right: u16,
}

impl Element for ContainerElement {

    fn draw(&self, constraint: Size, display_list: &mut DisplayList) {
        
        let child_height = constraint.y as usize / self.children.len();
        let mut y_offset = 0;
        
        // draw border
        drawBorder(constraint, display_list);

        // drawing each child
        for child in &self.children {
            let offset = Point {
                x: 0,
                y: y_offset as isize,
            };

            // move down ot child position
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
    // fn draw(&self) -> Frame {
    //     self.children
    //         .iter()
    //         .map(|child| {
    //             let mut frame = child.draw();
    //             frame.align_width();
    //             frame
    //         })
    //         .reduce(|mut acc, mut frame| {
    //             acc.append(&mut frame);
    //             acc
    //         })
    //         .unwrap_or_else(|| vec![vec![]])
    // }
}

fn drawBorder(constraint: Size, display_list: &mut DisplayList) {
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
            display_list.0.push(Operation::MoveTo(Point { x: constraint.x, y }));
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