use crate::prelude::{DisplayList, Element, Operation, Point, Size, BoxElement};

pub mod prelude {
    pub use super::FooterMainElement;
}

pub struct FooterMainElement {
    pub footer: Box<dyn Element>,
    pub main: Box<dyn Element>,
    pub footer_height: isize,
    pub box_main: bool,
}

impl Element for FooterMainElement {
    fn draw(&self, constraint: Size, display_list: &mut DisplayList) {
        let total_height = constraint.y;
        let footer_height = self.footer_height.min(total_height).max(0);
        let remaining_height = total_height - footer_height;

        // Draw main content at top (optionally boxed)
        if remaining_height > 0 {
            let offset = Point { x: 0, y: 0 };
            display_list.0.push(Operation::SetAnchor(offset));
            
            if self.box_main && remaining_height >= 3 && constraint.x >= 3 {
                // Draw main content with box
                BoxElement::draw_boxed(&*self.main,
                    Size {
                        x: constraint.x,
                        y: remaining_height,
                    },
                    display_list,
                );
            } else {
                // Draw main content without box
                self.main.draw(
                    Size {
                        x: constraint.x,
                        y: remaining_height,
                    },
                    display_list,
                );
            }
            
            display_list.0.push(Operation::SetAnchor(-offset));
        }

        // Draw footer at bottom
        if footer_height > 0 {
            let y_offset = remaining_height;
            let offset = Point {
                x: 0,
                y: y_offset,
            };
            display_list.0.push(Operation::SetAnchor(offset));
            self.footer.draw(
                Size {
                    x: constraint.x,
                    y: footer_height,
                },
                display_list,
            );
            display_list.0.push(Operation::SetAnchor(-offset));
        }
    }
}