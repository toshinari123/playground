use crate::prelude::Size;

pub mod prelude {
    pub use super::{Frame, FrameExt};
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
/// All Tokens must take up exactly one space in a terminal
pub enum Token {
    Char(char),
    AnnotatedChar(&'static str, char, &'static str),
}

impl Default for Token {
    fn default() -> Self {
        Self::Char(' ')
    }
}

pub trait TokensExt {
    fn to_string(&self) -> String;
}

impl TokensExt for Vec<Token> {
    fn to_string(&self) -> String {
        let mut s = String::new();
        for token in self {
            match token {
                Token::Char(c) => s.push(*c),
                Token::AnnotatedChar(s1, c, s2) => {
                    s.push_str(s1);
                    s.push(*c);
                    s.push_str(s2);
                }
            }
        }
        s
    }
}

pub type Frame = Vec<Vec<Token>>;

pub trait FrameExt {
    fn of_size(size: Size) -> Frame;
    fn height(&self) -> usize;
    fn first_width(&self) -> usize;
    fn max_width(&self) -> usize;
    fn align_width(&mut self);
    fn expand_to_height(&mut self, target: usize);
}

impl FrameExt for Frame {
    fn of_size(size: Size) -> Frame {
        vec![vec![Token::default(); size.x.abs() as usize]; size.y.abs() as usize]
    }
    fn height(&self) -> usize {
        self.len()
    }
    fn first_width(&self) -> usize {
        self.get(0).and_then(|row| Some(row.len())).unwrap_or(0)
    }
    fn max_width(&self) -> usize {
        self.iter().map(Vec::len).max().unwrap_or(0)
    }
    fn align_width(&mut self) {
        let max_width = self.max_width();
        for row in self.iter_mut() {
            let diff = max_width - row.len();
            row.append(&mut vec![Token::Char(' '); diff]);
        }
    }
    fn expand_to_height(&mut self, target: usize) {
        let width = self.first_width();
        let diff = target - self.height();
        if diff > 0 {
            for _ in 0..diff {
                self.push(vec![Token::Char(' '); width]);
            }
        }
    }
}
