use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use documents::prelude::*;
use react::prelude::*;
use stdext::prelude::*;

pub fn app(file: Document) -> Component {
    let (textfield, buffer) = text_field(file.content().unwrap_or("".to_string()));
    Widget::stateful(
        (textfield, buffer),
        move |this, msg| {
            switch(msg)
                .case({
                    let mut file = file.clone();
                    move |event: &KeyEvent| match (event.modifiers, event.code) {
                        (KeyModifiers::CONTROL, KeyCode::Char('s')) => {
                            _ = file.replace_with(this.state.1.borrow().as_bytes());
                            Intercept
                        }
                        _ => Propagate,
                    }
                })
                .default(|| Propagate)
        },
        move |(textfield, _)| scrollable_2d(textfield.clone()),
    )
}
