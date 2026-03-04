use std::ops::{Add, AddAssign, Neg, Sub, SubAssign};

use crate::frame::{Frame, Token};

pub mod prelude {
    pub use super::{Direction, DisplayList, Operation, Point, Size, Vec2};
}

#[derive(Debug, Clone, Copy, Hash, Default, PartialEq, Eq)]
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

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Direction {
    Start,
    End,
    Up,
    Down,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum Operation {
    PutChar(char),
    PutToken(Token),
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
            Operation::PutToken(token) => {
                let target = *anchor + *offset;
                if target.y >= 0
                    && let Some(row) = buffer.get_mut(target.y as usize)
                {
                    if target.x >= 0
                        && let Some(col) = row.get_mut(target.x as usize)
                    {
                        *col = token;
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

#[derive(Debug, Clone, Default, PartialEq)]
pub struct DisplayList(pub Vec<Operation>);

impl<T: Into<Vec<Operation>>> From<T> for DisplayList {
    fn from(value: T) -> Self {
        Self(value.into())
    }
}

impl DisplayList {
    pub fn draw_on(self, buffer: &mut Frame) {
        let mut anchor = Point::default();
        let mut offset = Point::default();
        self.0
            .into_iter()
            .for_each(|op| op.realize(&mut anchor, &mut offset, buffer));
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
