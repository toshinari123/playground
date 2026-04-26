use std::isize;

use crate::prelude::{
    Constraint2, DisplayList, Element, Operation, OptionConstraintExt, Pixel, Point,
};

pub mod prelude {
    pub use super::StringElement;
}

pub struct StringElement {
    pub s: String,
    pub cursor: Option<usize>,
}

impl Element for StringElement {
    fn propose_size(&self, proposed_constraints: Constraint2) -> Constraint2 {
        match (proposed_constraints.x, proposed_constraints.y) {
            (Some(x_constraint), None) => Constraint2 {
                x: Some(x_constraint),
                y: if let Pixel(x_constraint_pix) = x_constraint
                    && x_constraint_pix != 0
                {
                    Some(Pixel(count_lines(&self.s, x_constraint_pix)))
                } else {
                    None
                },
            },
            (None, Some(y_constraint)) => Constraint2 {
                x: if let Pixel(y_constraint_pix) = y_constraint
                    && y_constraint_pix != 0
                {
                    Some(Pixel(count_cols(&self.s, y_constraint_pix)))
                } else {
                    None
                },
                y: Some(y_constraint),
            },
            _ => proposed_constraints, // If parent has no x_constraint it makes more sense to let parent determine x than to write everything on a single line
        }
    }
    fn draw(&self, constraint: Constraint2, display_list: &mut DisplayList) {
        let (constraint_x, constraint_y) = (
            constraint.x.unwrap_pixel_or(isize::MAX),
            constraint.y.unwrap_pixel_or(isize::MAX),
        );
        let mut offset = Point::default();
        for (i, c) in self.s.chars().enumerate() {
            if c != '\n' {
                display_list.0.push(Operation::PutChar(c));
            }
            if let Some(cursor) = self.cursor
                && cursor == i
            {
                if c == '\n' {
                    display_list.0.push(Operation::PutChar(' '));
                    display_list.0.push(Operation::DrawCursor);
                } else {
                    display_list.0.push(Operation::DrawCursor);
                }
            }
            if c == '\n' {
                offset.y += 1;
                offset.x = 0;
            } else {
                offset.x += 1;
            }
            if offset.x >= constraint_x {
                offset.y += 1;
                offset.x = 0;
                if offset.y >= constraint_y {
                    display_list.0.push(Operation::PutChar('…'));
                    break;
                }
            }
            display_list.0.push(Operation::MoveTo(offset));
        }
        if let Some(cursor) = self.cursor
            && cursor == self.s.len()
        {
            display_list.0.push(Operation::DrawCursor);
        }
    }
}

fn count_lines(s: &str, x_constraint: isize) -> isize {
    s.split("\n")
        .map(|line| (((line.len() as f64) / (x_constraint as f64)).ceil() as isize).max(1)) // The newline takes up at least one row
        .sum()
}

fn count_cols(s: &str, y_constraint: isize) -> isize {
    count_lines(s, y_constraint) // Same as counting lines, but imagine it rotated by 90deg
}
