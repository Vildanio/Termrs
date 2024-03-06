use crate::{
    buffer::WriteBuffer,
    input::VisualInput,
    style::{Style, Styled},
    visual::{Draw, Visual},
    Position, Size,
};

pub struct TextBlock<'a> {
    text: &'a str,
    style: Style,
}

impl<'a> TextBlock<'a> {
    pub fn new(text: &'a str) -> Self {
        if text.len() > u16::MAX as usize {
            panic!("text length should be less than u16.MAX");
        }

        Self {
            text,
            style: Style::default(),
        }
    }
}

impl<'a> Styled for TextBlock<'a> {
    type Item = TextBlock<'a>;

    fn style(&self) -> Style {
        self.style
    }

    fn set_style(mut self, style: Style) -> Self::Item {
        self.style = style;
        self
    }
}

impl<'a> VisualInput for TextBlock<'a> {}

impl<'a> Draw for TextBlock<'a> {
    fn draw(&self, buffer: &mut dyn WriteBuffer, available_size: Size) -> Size {
        let width = (self.text.len() as u16).min(available_size.width);

        buffer
            .write_symbols(Position::default(), self.text, self.style)
            .expect("Cannot write to buffer");

        Size::new(width, 1)
    }

    fn measure(&self, constraints: Size) -> Size {
        let width = (self.text.len() as u16).min(constraints.width);

        Size::new(width, 1)
    }
}

impl<'a> Visual for TextBlock<'a> {}
