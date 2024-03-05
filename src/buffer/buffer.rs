use std::{error::Error, ops::Index};

use crate::{Attribute, Color, Position, Rect, Size};

pub trait Buffer: ReadBuffer + WriteBuffer {}

/// Represents two-demensional array of styled symbols
pub trait ReadBuffer: Index<u16, Output = char> {
    // Gets width and height of the buffer in cells
    fn size(&self) -> Size;

    fn bytes(&self, position: Position, length: u16) -> &[u8];
    fn characters(&self, position: Position, length: u16) -> &[char];

    /// Gets symbol in cell in the given position
    fn symbol(&self, position: Position) -> Result<&str, Box<dyn Error>>;

    /// Get foreground color of cell in the given position
    fn forecolor(&self, position: Position) -> Result<Color, Box<dyn Error>>;

    /// Get background color of cell in the given position
    fn backcolor(&self, position: Position) -> Result<Color, Box<dyn Error>>;

    /// Gets attibute of the cell in the given position
    fn attribute(&self, position: Position) -> Result<Attribute, Box<dyn Error>>;
}

/// Provides methods for mutating buffer.
pub trait WriteBuffer {
    /// Writes content of the given buffer.
    fn write_buffer(
        &mut self,
        position: Position,
        buffer: &dyn ReadBuffer,
    ) -> Result<(), Box<dyn Error>>;

    /// Set foreground color of cell in the given position
    fn set_forecolor(&mut self, position: Position, color: Color) -> Result<(), Box<dyn Error>>;

    /// Set background color of cell in the given position
    fn set_backcolor(&mut self, position: Position, color: Color) -> Result<(), Box<dyn Error>>;

    /// Sets symbol in cell in the given position
    fn set_symbol(&mut self, position: Position, symbol: char) -> Result<(), Box<dyn Error>>;

    fn write_symbols(&mut self, position: Position, symbols: &str) -> Result<(), Box<dyn Error>>;

    /// Sets attribute in cell in the given position
    fn set_attribute(
        &mut self,
        position: Position,
        attribute: Attribute,
    ) -> Result<(), Box<dyn Error>>;

    /// Clears the whole buffer
    fn clear(&mut self) -> Result<(), Box<dyn Error>>;

    /// Clears the given region of buffer
    fn clear_region(&mut self, region: Rect) -> Result<(), Box<dyn Error>>;
}
