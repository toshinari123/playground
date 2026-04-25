use crate::prelude::{DisplayList, Element, Operation, Point, Constraint2, Axis, Realize, ConstraintSum, OptionConstraintExt};

pub mod prelude {
    pub use super::RowElement;
}

pub struct RowElement {
    pub children: Vec<Box<dyn Element>>,
}

impl Element for RowElement {
    fn propose_size(&self, proposed_constraints: Constraint2) -> Constraint2 {
        Constraint2 {
            x: self
                .children
                .iter()
                .map(|child| {
                    child
                        .propose_size(Constraint2 {
                            x: None,
                            y: proposed_constraints.y,
                        })
                })
                .realize(Axis::X, proposed_constraints.x)
                .sum_constraint_in_axis(Axis::X).pixels
            ,
            y: proposed_constraints.y,
        }
    }
    fn draw(&self, constraint: Constraint2, display_list: &mut DisplayList) {
        let mut x_offset = 0;
        let children_constraints = self
            .children
            .iter()
            .map(|child| {
                child
                    .propose_size(Constraint2 {
                        x: None,
                        y: constraint.y,
                    })
            })
            .realize(Axis::X, constraint.x);
        for (child, child_constraint) in self.children.iter().zip(children_constraints) {
            let child_size = Constraint2 {
                x: child_constraint
                    .x,
                y: constraint.y,
            };
            let offset = Point {
                x: x_offset as isize,
                y: 0,
            };
            display_list.0.push(Operation::SetAnchor(offset));
            child.draw(child_size, display_list);
            display_list.0.push(Operation::SetAnchor(-offset));
            x_offset += child_size.x.to_pixel();
        }
    }
}
