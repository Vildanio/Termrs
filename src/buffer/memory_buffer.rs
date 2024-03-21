use crate::{
    buffer::{ReadBuffer, WriteBuffer},
    style::{Color, Style},
    Attribute, Position, Rect,
};

pub struct VirtualBuffer<'a> {
    original: &'a mut dyn WriteBuffer,
    region: Rect,
}

impl<'a> VirtualBuffer<'a> {
    pub fn new(original: &'a mut dyn WriteBuffer, region: Rect) -> Self {
        Self { original, region }
    }

    fn to_actual_position(&self, position: Position) -> Position {
        let position = Position::new(position.x + self.region.x, position.y + self.region.y);

        if !self.region.contains(position) {
            panic!("position was out of range of valid values")
        }

        position
    }
}

impl<'a> WriteBuffer for VirtualBuffer<'a> {
    fn write_buffer(
        &mut self,
        position: Position,
        buffer: &dyn ReadBuffer,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let size = buffer.size();

        if position.x + size.width > self.region.width
            || position.y + size.height > self.region.height
        {
            panic!()
        }

        let position = self.to_actual_position(position);

        self.original.write_buffer(position, buffer)
    }

    fn set_forecolor(
        &mut self,
        position: Position,
        color: Color,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let position = self.to_actual_position(position);

        self.original.set_forecolor(position, color)
    }

    fn set_backcolor(
        &mut self,
        position: Position,
        color: Color,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let position = self.to_actual_position(position);

        self.original.set_backcolor(position, color)
    }

    fn set_symbol(
        &mut self,
        position: Position,
        symbol: char,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let position = self.to_actual_position(position);

        self.original.set_symbol(position, symbol)
    }

    fn set_underline_color(
        &mut self,
        position: Position,
        color: Color,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let position = self.to_actual_position(position);

        self.original.set_underline_color(position, color)
    }

    fn set_attribute(
        &mut self,
        position: Position,
        attribute: Attribute,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let position = self.to_actual_position(position);

        self.original.set_attribute(position, attribute)
    }

    fn set_attributes(
        &mut self,
        position: Position,
        attributes: crossterm::style::Attributes,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let position = self.to_actual_position(position);

        self.original.set_attributes(position, attributes)
    }

    fn set_style(
        &mut self,
        position: Position,
        style: Style,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let position = self.to_actual_position(position);

        self.original.set_style(position, style)
    }

    fn clear(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.clear_region(self.region)
    }

    fn clear_region(&mut self, rect: Rect) -> Result<(), Box<dyn std::error::Error>> {
        if rect.right() > self.region.width || rect.bottom() > self.region.height {
            panic!()
        }

        let rect = Rect {
            x: rect.x + self.region.x,
            y: rect.y + self.region.y,
            width: rect.width,
            height: rect.height,
        };

        self.original.clear_region(rect)
    }

    fn write_symbols(
        &mut self,
        position: Position,
        symbols: &str,
        style: Style,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let symbols_len = symbols.len();

        if symbols_len > u16::MAX as usize {
            panic!()
        }

        if position.x + symbols_len as u16 > self.region.width
            || position.y + 1 > self.region.height
        {
            panic!()
        }

        let position = self.to_actual_position(position);

        self.original.write_symbols(position, symbols, style)
    }
}
