use crate::prelude::{DisplayList, Constraint2};

pub mod prelude {
    pub use super::Element;
}

pub trait Element: Send {
    fn propose_size(&self, proposed_constraints: Constraint2) -> Constraint2;
    fn draw(&self, constraint: Constraint2, display_list: &mut DisplayList);
}
