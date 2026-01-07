use crate::prelude::{DisplayList, Element, Operation, Point, Size};

pub mod prelude {
    pub use super::LeftRowElement;
}

pub struct LeftRowElement {
    pub children: Vec<Box<dyn Element>>,
    pub sidebar_width: isize,
}

impl Element for LeftRowElement {
    fn draw(&self, constraint: Size, display_list: &mut DisplayList) {
        let total_width = constraint.x;
        let sidebar_width = self.sidebar_width.min(total_width).max(0);
        let remaining_width = total_width - sidebar_width;
        let num_other_children = self.children.len().saturating_sub(1);
        
        // Draw sidebar (children[0]) at left
        if let Some(sidebar) = self.children.first() {
            let offset = Point {
                x: 0,
                y: 0,
            };
            display_list.0.push(Operation::SetAnchor(offset));
            sidebar.draw(
                Size {
                    x: sidebar_width,
                    y: constraint.y,
                },
                display_list,
            );
            display_list.0.push(Operation::SetAnchor(-offset));
        }
        
        // Draw other children (children[1..]) from left to right after sidebar
        if num_other_children > 0 {
            let other_child_width = remaining_width / num_other_children as isize;
            let mut x_offset = sidebar_width;
            for child in self.children.iter().skip(1) {
                let offset = Point {
                    x: x_offset,
                    y: 0,
                };
                display_list.0.push(Operation::SetAnchor(offset));
                child.draw(
                    Size {
                        x: other_child_width,
                        y: constraint.y,
                    },
                    display_list,
                );
                display_list.0.push(Operation::SetAnchor(-offset));
                x_offset += other_child_width;
            }
        }
    }
}
