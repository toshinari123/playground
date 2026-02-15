use crate::prelude::*;
use crate::elements::container_element::{Alignment, ContainerElement};

pub struct Container {
    child: Component,
    alignment: Alignment,
    padding: u16,
    margin: u16,
    border: bool,
}

impl Container {
    pub fn alignment(mut self, alignment: Alignment) -> Self {
        self.alignment = alignment;
        self
    }

    pub fn padding(mut self, padding: u16) -> Self {
        self.padding = padding;
        self
    }

    pub fn margin(mut self, margin: u16) -> Self {
        self.margin = margin;
        self
    }

    pub fn border(mut self) -> Self {
        self.border = true;
        self
    }

    pub fn build(self) -> Component {
        let Self { child, alignment, padding, margin, border } = self;
        
        Widget::elemental(child, |this, msg| {
            this.state.borrow_mut().on_message(msg);
        }, move |this| {
            let (did_rebuild, inner_element) = this.state.borrow_mut().create_element();
            (
                did_rebuild,
                Box::new(ContainerElement {
                    child: inner_element,
                    alignment,
                    padding,
                    margin,
                    border,
                }),
            )
        })
    }
}

pub fn container(child: Component) -> Container {
    Container {
        child,
        alignment: Alignment::Start,
        padding: 0,
        margin: 0,
        border: false,
    }
}