use crate::{
    buffer::WriteBuffer,
    visual::{input::VisualInput, Draw, Visual},
    Position, Size,
};

pub struct TextBlock<'a> {
    text: &'a str,
}

impl<'a> TextBlock<'a> {
    pub fn new(text: &'a str) -> Self {
        if text.len() > u16::MAX as usize {
            panic!("text length should be less than u16.MAX");
        }

        Self { text }
    }
}

impl<'a> VisualInput for TextBlock<'a> {}

impl<'a> Draw for TextBlock<'a> {
    fn draw(&self, buffer: &mut dyn WriteBuffer, available_size: Size) -> Size {
        let width = (self.text.len() as u16).min(available_size.width);

        buffer.write_symbols(Position::default(), self.text);

        Size::new(width, 1)
    }

    fn measure(&self, constraints: Size) -> Size {
        let width = (self.text.len() as u16).min(constraints.width);

        Size::new(width, 1)
    }
}

impl<'a> Visual for TextBlock<'a> {}
