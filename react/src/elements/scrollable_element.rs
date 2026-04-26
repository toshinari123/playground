use crate::{
    prelude::{Constraint2, DisplayList, Element, Frame, FrameExt, Size, Vec2, draw_frame_portion},
    utils::OptionConstraintExt,
};

pub mod prelude {
    pub use super::ScrollableElement;
}

pub struct ScrollableElement {
    pub offset: Vec2,
    pub child: Box<dyn Element>,
}

impl Element for ScrollableElement {
    fn propose_size(&self, proposed_constraints: Constraint2) -> Constraint2 {
        self.child.propose_size(proposed_constraints)
    }
    fn draw(&self, constraint: Constraint2, display_list: &mut DisplayList) {
        let mut child_frame = Frame::of_size(Size {
            x: constraint.x.to_pixel(),
            y: constraint.y.to_pixel(),
        });
        let mut child_display_list = DisplayList::default();
        self.child.draw(constraint, &mut child_display_list);
        child_display_list.draw_on(&mut child_frame);
        draw_frame_portion(&child_frame, self.offset, Vec2::default(), display_list);
    }
}
