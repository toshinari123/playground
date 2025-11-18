use std::{cell::RefCell, rc::Rc};

use react::{
    prelude::*,
    widgets::{streamed_counter::streamed_counter, timer::timer},
};
use stdext::prelude::*;

fn main() -> Result<()> {
    render(row([
        column([
            counter(12),
            text_field(""),
        ]),
        column([
            text_field(""),
            text("   ..eeeee..\n e8\"   8   \"8e\nd8     8     8b\n8!   .dWb.   !8\nY8 .e* 8 *e. 8P\n *8*   8   *8*\n   **ee8ee**"),
        ]),
    ]))
    // render(timer())
    // render(streamed_counter())
}
