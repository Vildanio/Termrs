mod styled;

pub use crossterm::style::{Attribute, Attributes, Color};
pub use styled::*;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Style {
    pub background: Color,
    pub foreground: Color,
    pub underline_color: Color,
    pub attributes: Attributes,
}

impl Default for Style {
    fn default() -> Self {
        Self::reset()
    }
}

impl Style {
    pub fn new(
        background: Color,
        foreground: Color,
        underline_color: Color,
        attributes: Attributes,
    ) -> Self {
        Self {
            background,
            foreground,
            underline_color,
            attributes,
        }
    }

    pub fn background(mut self, color: Color) -> Self {
        self.background = color;
        self
    }

    pub fn foreground(mut self, color: Color) -> Self {
        self.foreground = color;
        self
    }

    pub fn underline_color(mut self, color: Color) -> Self {
        self.underline_color = color;
        self
    }

    pub fn attributes(mut self, attributes: Attributes) -> Self {
        self.attributes = attributes;
        self
    }

    pub fn reset() -> Style {
        Self {
            background: Color::Reset,
            foreground: Color::Reset,
            underline_color: Color::Reset,
            attributes: Default::default(),
        }
    }
}
