use core::panic;
use std::ops::{Add, AddAssign, Neg, Sub, SubAssign};

pub mod prelude {
    pub(crate) use super::Pipe;
    pub use super::{
        Axis, Constraint, Constraint::Flex, Constraint::Pixel, Constraint2, Direction, Pipeline,
        Point, Size, Vec2, Realize, ConstraintSum, OptionConstraintExt
    };
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Axis {
    X,
    Y,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Constraint2 {
    pub x: Option<Constraint>,
    pub y: Option<Constraint>,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Constraint {
    Pixel(isize),
    Flex(isize),
}
use Constraint::{Flex, Pixel};

impl Constraint {
    pub fn is_pixel(self) -> bool {
        if let Pixel(_) = self { true } else { false }
    }
    pub fn is_flex(self) -> bool {
        if let Flex(_) = self { true } else { false }
    }
    pub fn to_pixel(self) -> isize {
        match self {
            Pixel(pix) => pix,
            Flex(_) => 0,
        }
    }
    pub fn to_pixel_constraint(self) -> Self {
        Pixel(self.to_pixel())
    }
    pub fn to_flex_constraint(self) -> Self {
        Flex(self.to_flex())
    }
    pub fn to_flex(self) -> isize {
        match self {
            Pixel(_) => 0,
            Flex(flex) => flex,
        }
    }
    pub fn add_pixels(self, rhs: Self) -> isize {
        self.to_pixel() + rhs.to_pixel()
    }
    pub fn add_flex(self, rhs: Self) -> isize {
        self.to_flex() + rhs.to_flex()
    }
}

pub trait OptionConstraintExt {
    fn is_pixel_or_none(self) -> bool;
    fn is_flex_or_none(self) -> bool;
    fn is_pixel(self) -> bool;
    fn is_flex(self) -> bool;
    fn to_pixel(self) -> isize;
    fn to_flex(self) -> isize;
    fn unwrap_pixel_or(self, default: isize) -> isize;
    fn unwrap_flex_or(self, default: isize) -> isize;
    fn add_pixels(self, rhs: Option<Constraint>) -> Option<Constraint>;
    fn add_pixels_or_none(self, rhs: Option<Constraint>) -> Option<Constraint>;
    fn add_flex_or_none(self, rhs: Option<Constraint>) -> Option<Constraint>;
    fn add_flex(self, rhs: Option<Constraint>) -> Option<Constraint>;
    fn expect_pixel_or_none(self, message: impl std::fmt::Display) -> Option<isize>;
}

impl OptionConstraintExt for Option<Constraint> {
    fn is_pixel_or_none(self) -> bool {
        match self {
            Some(Flex(_)) => false,
            _ => true
        }
    }
    fn is_flex_or_none(self) -> bool {
        match self {
            Some(Pixel(_)) => false,
            _ => true
        }
    }
    fn is_pixel(self) -> bool {
        match self {
            Some(Pixel(_)) => true,
            _ => false
        }
    }
    fn is_flex(self) -> bool {
        match self {
            Some(Flex(_)) => true,
            _ => false
        }
    }
    fn to_pixel(self) -> isize {
        self.unwrap_pixel_or(0)
    }
    fn to_flex(self) -> isize {
        self.unwrap_flex_or(0)
    }
    fn unwrap_pixel_or(self, default: isize) -> isize {
        match self {
            Some(Pixel(pix)) => pix,
            _ => default
        }
    }
    fn unwrap_flex_or(self, default: isize) -> isize {
        match self {
            Some(Flex(flex)) => flex,
            _ => default
        }
    }
    fn add_pixels(self, rhs: Option<Constraint>) -> Option<Constraint> {
        match (self, rhs) {
            (Some(dim1), Some(dim2)) => Some(Pixel(dim1.add_pixels(dim2))),
            _ => None,
        }
    }
    fn add_flex(self, rhs: Option<Constraint>) -> Option<Constraint> {
        match (self, rhs) {
            (Some(dim1), Some(dim2)) => Some(Flex(dim1.add_flex(dim2))),
            _ => None,
        }
    }
    fn add_pixels_or_none(self, rhs: Option<Constraint>) -> Option<Constraint> {
        match (self, rhs) {
            (Some(Pixel(dim1)), Some(Pixel(dim2))) => Some(Pixel(dim1 + dim2)),
            _ => None,
        }
    }
    fn add_flex_or_none(self, rhs: Option<Constraint>) -> Option<Constraint> {
        match (self, rhs) {
            (Some(Flex(dim1)), Some(Flex(dim2))) => Some(Flex(dim1 + dim2)),
            _ => None,
        }
    }
    fn expect_pixel_or_none(self, message: impl std::fmt::Display) -> Option<isize> {
        match self {
            Some(Pixel(pix)) => Some(pix),
            Some(Flex(_)) => panic!("{message}"),
            None => None,
        }
    }
}

pub(crate) trait IteratorOptionIsizeExt {
    fn sum_or_none(self) -> Option<isize>;
}

impl<It: Iterator<Item = Option<isize>>> IteratorOptionIsizeExt for It {
    fn sum_or_none(self) -> Option<isize> {
        let mut sum = 0;
        for opt in self {
            if let Some(i) = opt {
                sum += i;
            } else {
                return None;
            }
        }
        Some(sum)
    }
}

pub trait Realize {
    fn realize(
        self,
        axis: Axis,
        axis_constraint: Option<Constraint>,
    ) -> impl Iterator<Item = Constraint2>;
}

pub trait ConstraintSum {
    fn sum_constraint_in_axis(self, axis: Axis) -> PixelFlexSum;
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct PixelFlexSum {
    pub pixels: Option<Constraint>,
    pub flex: Option<Constraint>,
}

impl<T: IntoIterator<Item = Constraint2>> ConstraintSum for T {
    fn sum_constraint_in_axis(self, axis: Axis) -> PixelFlexSum {
        self.into_iter().fold(
            PixelFlexSum {
                pixels: Some(Pixel(0)),
                flex: Some(Flex(0)),
            },
            |acc, e| PixelFlexSum {
                pixels: acc.pixels.add_pixels(
                        match axis {
                            Axis::X => e.x,
                            Axis::Y => e.y,
                        }
                ),
                flex: acc.flex.add_flex(
                        match axis {
                            Axis::X => e.x,
                            Axis::Y => e.y,
                        }
                    )
            },
        )
    }
}

impl<T: IntoIterator<Item = Constraint2>> Realize for T
where
    T::IntoIter: Clone,
{
    fn realize(
        self,
        axis: Axis,
        axis_constraint: Option<Constraint>,
    ) -> impl Iterator<Item = Constraint2> {
        let dims = self.into_iter();
        let PixelFlexSum { pixels, flex } = dims.clone().sum_constraint_in_axis(axis);
        assert!(pixels.is_none_or(Constraint::is_pixel));
        assert!(flex.is_none_or(Constraint::is_flex));
        let pixels_for_flex = match axis_constraint {
            Some(Pixel(constraint)) => Some(constraint - pixels.to_pixel()),
            _ => None,
        };
        let flex_sum = flex;
        let realized_constraint = move |dim| match dim {
            Some(Pixel(pix)) => Some(Pixel(pix)),
            Some(Flex(flex)) => match (flex_sum, pixels_for_flex) {
                (Some(Flex(flex_sum)), Some(pixels_for_flex)) => {
                    dbg!(Some(Pixel(flex * (pixels_for_flex / flex_sum))))
                }
                _ => dbg!(Some(Flex(flex))),
            },
            None => None,
        };
        dims.map(move |dim| match axis {
            Axis::X => Constraint2 {
                x: realized_constraint(dim.x),
                y: dim.y
            },
            Axis::Y => Constraint2 {
                x: dim.x,
                y: realized_constraint(dim.y),
            }
        })
    }
}

impl Constraint2 {
    pub fn is_pixels_or_none_then(&self, f: impl FnOnce(&Self)) {
        if self.x.is_pixel_or_none() && self.y.is_pixel_or_none() {
            f(self)
        }
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Direction {
    Start,
    End,
    Up,
    Down,
}

pub trait Pipeline<Arg, Ret> {
    fn apply(self, arg: Arg) -> Ret;
}

impl<Arg, Ret, F> Pipeline<Arg, Ret> for F
where
    F: FnOnce(Arg) -> Ret,
{
    #[inline]
    fn apply(self, arg: Arg) -> Ret {
        self(arg)
    }
}

impl<Arg, Ret, A, F1, F2> Pipeline<Arg, Ret> for (F1, F2)
where
    F1: FnOnce(Arg) -> A,
    F2: FnOnce(A) -> Ret,
{
    #[inline]
    fn apply(self, arg: Arg) -> Ret {
        self.1(self.0.apply(arg))
    }
}

impl<Arg, Ret, A, B, F1, F2, F3> Pipeline<Arg, Ret> for (F1, F2, F3)
where
    F1: FnOnce(Arg) -> A,
    F2: FnOnce(A) -> B,
    F3: FnOnce(B) -> Ret,
{
    #[inline]
    fn apply(self, arg: Arg) -> Ret {
        self.2((self.0, self.1).apply(arg))
    }
}

impl<Arg, Ret, A, B, C, F1, F2, F3, F4> Pipeline<Arg, Ret> for (F1, F2, F3, F4)
where
    F1: FnOnce(Arg) -> A,
    F2: FnOnce(A) -> B,
    F3: FnOnce(B) -> C,
    F4: FnOnce(C) -> Ret,
{
    #[inline]
    fn apply(self, arg: Arg) -> Ret {
        self.3((self.0, self.1, self.2).apply(arg))
    }
}

impl<Arg, Ret, A, B, C, D, F1, F2, F3, F4, F5> Pipeline<Arg, Ret> for (F1, F2, F3, F4, F5)
where
    F1: FnOnce(Arg) -> A,
    F2: FnOnce(A) -> B,
    F3: FnOnce(B) -> C,
    F4: FnOnce(C) -> D,
    F5: FnOnce(D) -> Ret,
{
    #[inline]
    fn apply(self, arg: Arg) -> Ret {
        self.4((self.0, self.1, self.2, self.3).apply(arg))
    }
}

impl<Arg, Ret, A, B, C, D, E, F1, F2, F3, F4, F5, F6> Pipeline<Arg, Ret>
    for (F1, F2, F3, F4, F5, F6)
where
    F1: FnOnce(Arg) -> A,
    F2: FnOnce(A) -> B,
    F3: FnOnce(B) -> C,
    F4: FnOnce(C) -> D,
    F5: FnOnce(D) -> E,
    F6: FnOnce(E) -> Ret,
{
    #[inline]
    fn apply(self, arg: Arg) -> Ret {
        self.5((self.0, self.1, self.2, self.3, self.4).apply(arg))
    }
}

pub(crate) trait Pipe<Arg, Ret> {
    fn pipe(self, pipeline: impl Pipeline<Arg, Ret>) -> Ret;
}

impl<T, Ret> Pipe<T, Ret> for T {
    fn pipe(self, pipeline: impl Pipeline<T, Ret>) -> Ret {
        pipeline.apply(self)
    }
}
