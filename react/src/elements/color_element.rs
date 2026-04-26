use crate::prelude::{Color, Element, DisplayList, Constraint2};

pub mod prelude {

}

pub struct ColorElement {
    pub color: Color,
    pub child: Box<dyn Element>
}

impl Element for ColorElement {
    fn propose_size(&self, proposed_constraints: Constraint2) -> Constraint2 {
        self.child.propose_size(proposed_constraints)
    }
    fn draw(&self, constraint: Constraint2, display_list: &mut DisplayList) {
        
    }
}