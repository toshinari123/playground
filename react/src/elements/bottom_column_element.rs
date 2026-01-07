use crate::prelude::{DisplayList, Element, Frame, Operation, Point, Size};

pub mod prelude {
    pub use super::BottomColumnElement;
}

pub struct BottomColumnElement {
    pub children: Vec<Box<dyn Element>>,
    pub footer_height: isize,
}

impl Element for BottomColumnElement {
    fn draw(&self, constraint: Size, display_list: &mut DisplayList) {
        let total_height = constraint.y;
        let footer_height = self.footer_height.min(total_height).max(0);
        let remaining_height = total_height - footer_height;
        let num_other_children = self.children.len().saturating_sub(1);
        
        // Draw other children (children[1..]) from top to bottom
        if num_other_children > 0 {
            let other_child_height = remaining_height / num_other_children as isize;
            let mut y_offset = 0;
            for child in self.children.iter().skip(1) {
                let offset = Point {
                    x: 0,
                    y: y_offset,
                };
                display_list.0.push(Operation::SetAnchor(offset));
                child.draw(
                    Size {
                        x: constraint.x,
                        y: other_child_height,
                    },
                    display_list,
                );
                display_list.0.push(Operation::SetAnchor(-offset));
                y_offset += other_child_height;
            }
        }
        
        // Draw footer (children[0]) at bottom
        if let Some(footer) = self.children.first() {
            let y_offset = remaining_height;
            let offset = Point {
                x: 0,
                y: y_offset,
            };
            display_list.0.push(Operation::SetAnchor(offset));
            footer.draw(
                Size {
                    x: constraint.x,
                    y: footer_height,
                },
                display_list,
            );
            display_list.0.push(Operation::SetAnchor(-offset));
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
