use std::ops::{Add, AddAssign, Neg, Sub, SubAssign};

use crate::frame::{Frame, Token};

pub mod prelude {
    pub use super::{Direction, DisplayList, Operation, Point, Size, Vec2};
}

#[derive(Debug, Clone, Copy, Hash, Default, PartialEq)]
pub struct Vec2 {
    pub x: isize,
    pub y: isize,
}

impl Vec2 {
    pub fn adjacent(self, direction: Direction) -> Option<Self> {
        Some(Self {
            x: match direction {
                Direction::Start => self.x - 1,
                Direction::End => self.x + 1,
                _ => self.x,
            },
            y: match direction {
                Direction::Up => self.y - 1,
                Direction::Down => self.y + 1,
                _ => self.y,
            },
        })
    }

    pub fn within_constraint(&self, constraint: &Self) -> bool {
        self.x < constraint.x && self.y < constraint.y
    }
}

impl Add for Vec2 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl AddAssign for Vec2 {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl Sub for Vec2 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl SubAssign for Vec2 {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl Neg for Vec2 {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
        }
    }
}

pub type Point = Vec2;
pub type Size = Vec2;

#[derive(Debug, Clone, Copy, Hash)]
pub enum Direction {
    Start,
    End,
    Up,
    Down,
}

#[derive(Debug, Clone, Hash)]
pub enum Operation {
    PutChar(char),
    DrawCursor,
    MoveTo(Point),
    Move(Direction),
    SetAnchor(Point),
}

impl Operation {
    pub fn realize(self, anchor: &mut Point, offset: &mut Point, buffer: &mut Frame) {
        match self {
            Operation::PutChar(c) => {
                let target = *anchor + *offset;
                if target.y >= 0
                    && let Some(row) = buffer.get_mut(target.y as usize)
                {
                    if target.x >= 0
                        && let Some(col) = row.get_mut(target.x as usize)
                    {
                        *col = match col {
                            Token::Char(_) => Token::Char(c),
                            Token::AnnotatedChar(s1, _, s2) => Token::AnnotatedChar(s1, c, s2),
                        };
                    }
                }
            }
            Operation::MoveTo(point) => {
                *offset = point;
            }
            Operation::Move(direction) => {
                if let Some(new_offset) = offset.adjacent(direction) {
                    *offset = new_offset;
                }
            }
            Operation::SetAnchor(point) => {
                *anchor += point;
                *offset = Point::default();
            }
            Operation::DrawCursor => {
                let target = *anchor + *offset;
                if target.y >= 0
                    && let Some(row) = buffer.get_mut(target.y as usize)
                {
                    if target.x >= 0
                        && let Some(col) = row.get_mut(target.x as usize)
                    {
                        *col = match col {
                            Token::AnnotatedChar(_, c, _) | Token::Char(c) => {
                                Token::AnnotatedChar("\x1b[48;2;146;146;146m", *c, "\x1b[0m")
                            }
                        };
                    }
                }
            }
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct DisplayList(pub Vec<Operation>);

impl<T: Into<Vec<Operation>>> From<T> for DisplayList {
    fn from(value: T) -> Self {
        Self(value.into())
    }
}

impl DisplayList {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn draw_on(self, buffer: &mut Frame) {
        let mut anchor = Point::default();
        let mut offset = Point::default();
        self.0
            .into_iter()
            .for_each(|op| op.realize(&mut anchor, &mut offset, buffer));
    }

    /// Calculates the height (number of rows) this display list occupies.
    pub fn height(&self) -> isize {
        let mut anchor = Point::default();
        let mut offset = Point::default();
        let mut min_y: isize = isize::MAX;
        let mut max_y: isize = isize::MIN;

        for op in &self.0 {
            match op {
                Operation::PutChar(_) | Operation::DrawCursor => {
                    let pos = anchor + offset;
                    min_y = min_y.min(pos.y);
                    max_y = max_y.max(pos.y);
                }
                Operation::MoveTo(point) => offset = *point,
                Operation::Move(direction) => {
                    if let Some(new_offset) = offset.adjacent(*direction) {
                        offset = new_offset;
                    }
                }
                Operation::SetAnchor(point) => {
                    anchor += *point;
                    offset = Point::default();
                }
            }
        }

        if min_y == isize::MAX {
            0
        } else {
            max_y - min_y + 1
        }
    }
    
    /// Calculates the width (number of columns) this display list occupies.
    pub fn width(&self) -> isize {
        let mut anchor = Point::default();
        let mut offset = Point::default();
        let mut min_x: isize = isize::MAX;
        let mut max_x: isize = isize::MIN;

        for op in &self.0 {
            match op {
                Operation::PutChar(_) | Operation::DrawCursor => {
                    let pos = anchor + offset;
                    min_x = min_x.min(pos.x);
                    max_x = max_x.max(pos.x);
                }
                Operation::MoveTo(point) => offset = *point,
                Operation::Move(direction) => {
                    if let Some(new_offset) = offset.adjacent(*direction) {
                        offset = new_offset;
                    }
                }
                Operation::SetAnchor(point) => {
                    anchor += *point;
                    offset = Point::default();
                }
            }
        }

        if min_x == isize::MAX {
            0
        } else {
            max_x - min_x + 1
        }
    }

    /// Sets a character at a specific position
    pub fn set(&mut self, x: isize, y: isize, c: char) {
        self.0.push(Operation::MoveTo(Point { x, y }));
        self.0.push(Operation::PutChar(c));
    }

    /// Appends another display list with an offset
    pub fn merge(&mut self, other: &DisplayList, offset_x: isize, offset_y: isize) {
        self.0.push(Operation::SetAnchor(Point { x: offset_x, y: offset_y }));
        self.0.extend(other.0.clone());
        self.0.push(Operation::SetAnchor(Point { x: -offset_x, y: -offset_y }));
    }
    
    /// Merges operations from `source` into `self`, but only keeps content
    /// within a clip rectangle. Useful for partial rendering (e.g., scrolling).
    ///
    /// - `x_offset`, `y_offset`: where to place the clipped content in `self`
    /// - `clip_width`, `clip_height`: size of the visible region (0,0 to clip_width,clip_height)
    pub fn merge_clipped(
        &mut self,
        source: &DisplayList,
        x_offset: isize,
        y_offset: isize,
        clip_width: isize,
        clip_height: isize,
    ) {
        // anchor = origi pos. relative to parent
        // Track the cumulative anchor offset (anchors stack/accumulate)
        let mut anchor_x: isize = 0;
        let mut anchor_y: isize = 0;

        // Track the current cursor position (where the next char would go)
        let mut cursor_x: isize = 0;
        let mut cursor_y: isize = 0;

        for op in &source.0 {
            match op {
                // Anchors accumulate - they're relative offsets that stack
                Operation::SetAnchor(point) => {
                    anchor_x += point.x;
                    anchor_y += point.y;
                }

                // MoveTo is relative to the current anchor
                Operation::MoveTo(point) => {
                    cursor_x = anchor_x + point.x;
                    cursor_y = anchor_y + point.y;
                }

                // Move shifts cursor by 1 in the given direction
                Operation::Move(direction) => {
                    match direction {
                        Direction::Up => cursor_y -= 1,
                        Direction::Down => cursor_y += 1,
                        Direction::Start => cursor_x -= 1,
                        Direction::End => cursor_x += 1,
                    }
                }

                // PutChar: only emit if cursor is inside the clip rectangle
                Operation::PutChar(ch) => {
                    if cursor_x >= 0 && cursor_x < clip_width
                        && cursor_y >= 0 && cursor_y < clip_height
                    {
                        // Emit with offset applied - translates to final position in `self`
                        self.0.push(Operation::MoveTo(Point {
                            x: cursor_x + x_offset,
                            y: cursor_y + y_offset,
                        }));
                        self.0.push(Operation::PutChar(*ch));
                    }
                    // Cursor always advances, even if clipped
                    cursor_x += 1;
                }

                // DrawCursor: same clipping logic as PutChar
                Operation::DrawCursor => {
                    if cursor_x >= 0 && cursor_x < clip_width
                        && cursor_y >= 0 && cursor_y < clip_height
                    {
                        self.0.push(Operation::MoveTo(Point {
                            x: cursor_x + x_offset,
                            y: cursor_y + y_offset,
                        }));
                        self.0.push(Operation::DrawCursor);
                    }
                }
            }
        }
    }
}

// #[cfg(test)]
// mod test {
//     use super::*;
//     use stdext::prelude::*;

//     #[test]
//     fn test6() {
//         let mut buffer = vec![vec![' '; 5]; 5];
//         DisplayList::default().draw_on(&mut buffer);
//         buffer.must_be(vec![
//             vec![' ', ' ', ' ', ' ', ' '],
//             vec![' ', ' ', ' ', ' ', ' '],
//             vec![' ', ' ', ' ', ' ', ' '],
//             vec![' ', ' ', ' ', ' ', ' '],
//             vec![' ', ' ', ' ', ' ', ' '],
//         ]);
//     }
//     #[test]
//     fn test7() {
//         let mut buffer = vec![vec![' '; 5]; 5];
//         DisplayList::from([Operation::PutChar('a')]).draw_on(&mut buffer);
//         buffer.must_be(vec![
//             vec![' ', ' ', ' ', ' ', ' '],
//             vec![' ', ' ', 'a', ' ', ' '],
//             vec![' ', ' ', ' ', ' ', ' '],
//             vec![' ', ' ', ' ', ' ', ' '],
//             vec![' ', ' ', ' ', ' ', ' '],
//         ]);
//     }
//     #[test]
//     fn test8() {
//         let mut buffer = vec![vec![' '; 5]; 5];
//         DisplayList(vec![
//             Operation::PutChar('a'),
//             Operation::Move(Direction::End),
//             Operation::PutChar('b'),
//             Operation::Move(Direction::End),
//             Operation::PutChar('c'),
//         ])
//         .draw_on(&mut buffer);
//         buffer.must_be(vec![
//             vec![' ', ' ', ' ', ' ', ' '],
//             vec![' ', ' ', 'a', 'b', 'c'],
//             vec![' ', ' ', ' ', ' ', ' '],
//             vec![' ', ' ', ' ', ' ', ' '],
//             vec![' ', ' ', ' ', ' ', ' '],
//         ]);
//     }
// }
