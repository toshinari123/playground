pub mod prelude {
    pub use super::Axis;
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Axis {
    Horizontal,
    Vertical,
}
