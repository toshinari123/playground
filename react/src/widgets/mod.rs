pub mod adjustable_column;
pub mod adjustable_row;
pub mod animated_char;
pub mod column;
pub mod counter;
pub mod delayed;
pub mod download;
pub mod fast_counter;
pub mod focus_root;
pub mod focusable_centered_text;
pub mod number;
pub mod row;
pub mod scrollable;
pub mod single_char;
pub mod single_line;
pub mod streamed_counter;
pub mod text;
pub mod text_cursor;
pub mod text_field;
pub mod timer;
//pub mod footer;

pub mod prelude {
    pub use super::{
        adjustable_column::adjustable_column,
        adjustable_row::adjustable_row,
        animated_char::animated_char,
        column::column,
        counter::counter,
        delayed::delayed,
        download::download,
        fast_counter::fast_counter,
        focus_root::focus_root,
        focusable_centered_text::focusable_centered_text,
        number::number,
        row::row,
        scrollable::{scrollable, scrollable_2d, scrollable_h, scrollable_v},
        single_char::single_char,
        single_line::single_line,
        text::text,
        text_cursor::text_cursor,
        text_field::text_field,
        timer::timer,
        //footer::footer,
    };
}
