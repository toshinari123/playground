use crate::prelude::{DisplayList, Element, Frame, Operation, Point, Size};

pub mod prelude {
    pub use super::ColumnElement;
}

pub struct ColumnElement {
    pub children: Vec<Box<dyn Element>>,
}

impl Element for ColumnElement {
    fn draw(&self, constraint: Size, display_list: &mut DisplayList) {
        if self.children.is_empty() {
            return;
        }
        let child_height = constraint.y as usize / self.children.len();
        let mut y_offset = 0;
        for child in &self.children {
            let offset = Point {
                x: 0,
                y: y_offset as isize,
            };
            display_list.0.push(Operation::SetAnchor(offset));
            child.draw(
                Size {
                    x: constraint.x,
                    y: child_height as isize,
                },
                display_list,
            );
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
