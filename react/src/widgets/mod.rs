pub mod animated_char;
pub mod column;
pub mod counter;
pub mod delayed;
pub mod download;
pub mod fast_counter;
pub mod number;
pub mod row;
pub mod scrollable;
pub mod single_char;
pub mod single_line;
pub mod stack;
pub mod streamed_counter;
pub mod text;
pub mod text_cursor;
pub mod text_field;
pub mod timer;

pub mod prelude {
    pub use super::{
        animated_char::animated_char,
        column::column,
        counter::counter,
        delayed::delayed,
        download::download,
        fast_counter::fast_counter,
        number::number,
        row::row,
        scrollable::{scrollable, scrollable_2d, scrollable_h, scrollable_v},
        single_char::single_char,
        single_line::single_line,
        stack::stack,
        text::text,
        text_cursor::text_cursor,
        text_field::text_field,
        timer::timer,
    };
}
