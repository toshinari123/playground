use crate::prelude::*;

pub mod prelude {
    pub use super::{Alignment, ContainerElement};
}

#[derive(Clone, Copy)]
pub enum Alignment {
    Start,
    Center,
    End,
    Stretch,
}

pub struct ContainerElement {
    pub child: Box<dyn Element>,
    pub alignment: Alignment,
    pub padding: u16,
    pub margin: u16,
    pub border: bool,
}

impl Element for ContainerElement {
    fn draw(&self, constraint: Size, display_list: &mut DisplayList) {
        let margin = self.margin as isize;
        let padding = self.padding as isize;
        let border_size = if self.border { 1 } else { 0 };

        let inner_width = (constraint.x - 2 * margin - 2 * border_size - 2 * padding).max(0);
        let inner_height = (constraint.y - 2 * margin - 2 * border_size - 2 * padding).max(0);

        let child_constraint = Size { x: inner_width, y: inner_height };

        let mut child_dl = DisplayList::new();
        self.child.draw(child_constraint, &mut child_dl);
        let child_width = child_dl.width();
        let child_height = child_dl.height();

        if self.border {
            let bx = margin;
            let by = margin;
            let bw = constraint.x - 2 * margin;
            let bh = constraint.y - 2 * margin;

            display_list.set(bx, by, '┌');
            for x in (bx + 1)..(bx + bw - 1) {
                display_list.set(x, by, '─');
            }
            display_list.set(bx + bw - 1, by, '┐');

            display_list.set(bx, by + bh - 1, '└');
            for x in (bx + 1)..(bx + bw - 1) {
                display_list.set(x, by + bh - 1, '─');
            }
            display_list.set(bx + bw - 1, by + bh - 1, '┘');

            for y in (by + 1)..(by + bh - 1) {
                display_list.set(bx, y, '│');
                display_list.set(bx + bw - 1, y, '│');
            }
        }

        let content_x = margin + border_size + padding;
        let content_y = margin + border_size + padding;

        let (offset_x, offset_y) = match self.alignment {
            Alignment::Start => (0, 0),
            Alignment::Center => (
                (inner_width - child_width).max(0) / 2,
                (inner_height - child_height).max(0) / 2,
            ),
            Alignment::End => (
                (inner_width - child_width).max(0),
                (inner_height - child_height).max(0),
            ),
            Alignment::Stretch => (0, 0),
        };

        display_list.merge(&child_dl, content_x + offset_x, content_y + offset_y);
    }
}