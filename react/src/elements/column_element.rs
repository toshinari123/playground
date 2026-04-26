use crate::prelude::{
    Axis, Constraint2, ConstraintSum, DisplayList, Element, Operation, OptionConstraintExt, Point,
    Realize,
};

pub mod prelude {
    pub use super::ColumnElement;
}

pub struct ColumnElement {
    pub children: Vec<Box<dyn Element>>,
}

impl Element for ColumnElement {
    fn propose_size(&self, proposed_constraints: Constraint2) -> Constraint2 {
        Constraint2 {
            x: proposed_constraints.x,
            y: self
                .children
                .iter()
                .map(|child| {
                    child.propose_size(Constraint2 {
                        x: proposed_constraints.x,
                        y: None,
                    })
                })
                .realize(Axis::Y, proposed_constraints.y)
                .sum_constraint_in_axis(Axis::Y)
                .pixels,
        }
    }
    fn draw(&self, constraint: Constraint2, display_list: &mut DisplayList) {
        let mut y_offset = 0;
        let children_constraints = self
            .children
            .iter()
            .map(|child| {
                child.propose_size(Constraint2 {
                    x: constraint.x,
                    y: None,
                })
            })
            .realize(Axis::Y, constraint.y);
        for (child, child_constraint) in self.children.iter().zip(children_constraints) {
            let child_size = Constraint2 {
                x: constraint.x,
                y: child_constraint.y,
            };
            let offset = Point {
                x: 0,
                y: y_offset as isize,
            };
            display_list.0.push(Operation::SetAnchor(offset));
            child.draw(child_size, display_list);
            display_list.0.push(Operation::SetAnchor(-offset));
            y_offset += child_size.y.to_pixel();
        }
    }
}
