use crate::displaylist::{DisplayList, Size};

pub mod prelude {
    pub use super::Element;
}

pub trait Element: Send {
    fn draw(&self, constraint: Size, 
        display_list: &mut DisplayList);
}
