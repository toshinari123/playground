use crate::prelude::{DisplayList, Frame, FrameExt, Operation, Point, Vec2};

pub mod prelude {
    pub use super::draw_frame_portion;
}

pub fn draw_frame_portion(
    frame: &Frame,
    offset_top_start: Vec2,
    offset_bottom_end: Vec2,
    display_list: &mut DisplayList,
) {
    let height = frame.height();
    let width = frame.first_width();
    for (y, row) in frame[offset_top_start.y as usize..(height - offset_bottom_end.y as usize)]
        .iter()
        .enumerate()
    {
        for (x, col) in row[offset_top_start.x as usize..(width - offset_bottom_end.y as usize)]
            .iter()
            .enumerate()
        {
            display_list.0.push(Operation::MoveTo(Point {
                x: x as isize,
                y: y as isize,
            }));
            display_list.0.push(Operation::PutToken(col.clone()));
        }
    }
}
