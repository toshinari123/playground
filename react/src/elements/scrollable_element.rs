use crate::prelude::{DisplayList, Element, Frame, FrameExt, Size, Vec2, draw_frame_portion};

pub mod prelude {
    pub use super::ScrollableElement;
}

pub struct ScrollableElement {
    pub offset: Vec2,
    pub child: Box<dyn Element>,
}

impl Element for ScrollableElement {
    fn draw(&self, constraint: Size, display_list: &mut DisplayList) {
        let mut child_frame = Frame::of_size(constraint);
        let mut child_display_list = DisplayList::default();
        self.child.draw(constraint, &mut child_display_list);
        child_display_list.draw_on(&mut child_frame);
        draw_frame_portion(&child_frame, self.offset, Vec2::default(), display_list);
    }
}
