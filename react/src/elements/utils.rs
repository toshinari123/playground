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
    let start_y = (offset_top_start.y as usize).min(height);
    let end_y = (height - offset_bottom_end.y as usize).max(start_y);
    let start_x = (offset_top_start.x as usize).min(width);
    let end_x = (width - offset_bottom_end.x as usize).max(start_x);
    for (y, row) in frame[start_y..end_y]
        .iter()
        .enumerate()
    {
        for (x, col) in row[start_x..end_x]
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
