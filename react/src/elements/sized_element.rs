use crate::prelude::{Element, Constraint2, DisplayList};

pub mod prelude {
    pub use super::SizedElement;
}

pub struct SizedElement {
    pub size: Constraint2,
    pub child: Box<dyn Element>,
}

impl Element for SizedElement {
    fn propose_size(&self, proposed_constraints: Constraint2) -> Constraint2 {
        match (self.size.x, self.size.y) {
            (Some(_), Some(_)) => self.size,
            (x @ Some(_), None) => Constraint2 {
                x,
                y: self
                    .child
                    .propose_size(Constraint2 {
                        x,
                        y: proposed_constraints.y,
                    })
                    .y,
            },
            (None, y @ Some(_)) => Constraint2 {
                x: self.child.propose_size(Constraint2 {
                    x: proposed_constraints.x,
                    y,
                }).x,
                y,
            },
            _ => self.child.propose_size(proposed_constraints)
        }
    }
    fn draw(
        &self,
        constraint: Constraint2,
        display_list: &mut DisplayList,
    ) {
        self.child.draw(constraint, display_list);
    }
}
