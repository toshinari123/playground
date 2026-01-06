use std::io::Result;

use react::prelude::*;
use react::widgets::streamed_counter::streamed_counter;

fn main() -> Result<()> {
    render(row([
        column([streamed_counter(), text_field("").0]),
        column([text_field("").0, download("https://www.rust-lang.org")]),
    ]))
}
