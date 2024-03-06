use std::{
    error::Error,
    io::{self, stdout, Write},
};

use crossterm::{
    cursor::{Hide, MoveTo, Show},
    queue,
    style::{
        Colors, SetAttribute, SetAttributes, SetBackgroundColor, SetColors, SetForegroundColor,
        SetUnderlineColor,
    },
    terminal::{self, Clear, ClearType},
    QueueableCommand,
};

use super::{ReadBuffer, WriteBuffer};
use crate::{style::Style, Attribute, Color, Position, Rect, Size};

pub struct Terminal<W: Write> {
    /// The writer used to send commands to the terminal.
    writer: W,

    /// Value indicating whether cursor is visible or not.
    /// This is needed to get error when attempt to
    /// restore cursor state when terminal gets dropped fails.
    cursor_visible: bool,
}

impl<W: Write> Drop for Terminal<W> {
    fn drop(&mut self) {
        // Attempt to restore the cursor state
        if !self.cursor_visible {
            if let Err(err) = self.show_cursor() {
                eprintln!("Failed to show the cursor: {err}");
            }
        }
    }
}

impl<W: Write> Terminal<W> {
    pub fn new(writer: W) -> Terminal<W> {
        Self {
            writer,
            cursor_visible: true,
        }
    }

    pub fn from_stdout() -> Terminal<io::Stdout> {
        Terminal::<io::Stdout>::new(stdout())
    }

    pub fn size(&self) -> Size {
        let (width, height) = terminal::size().unwrap();

        Size::new(width, height)
    }

    pub fn hide_cursor(&mut self) -> Result<(), std::io::Error> {
        queue!(self.writer, Hide)
    }

    pub fn show_cursor(&mut self) -> Result<(), std::io::Error> {
        queue!(self.writer, Show)
    }

    pub fn get_cursor(&mut self) -> std::io::Result<Position> {
        match crossterm::cursor::position() {
            Ok(position) => Ok(Position::new(position.0, position.1)),
            Err(error) => Err(error),
        }
    }

    pub fn set_cursor(&mut self, position: Position) -> Result<(), std::io::Error> {
        queue!(self.writer, MoveTo(position.x, position.y))
    }
}

impl<W: Write> Write for Terminal<W> {
    /// Writes a buffer of bytes to the underlying buffer.
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.writer.write(buf)
    }

    /// Flushes the underlying buffer.
    fn flush(&mut self) -> io::Result<()> {
        self.writer.flush()
    }
}

// Use "queue" instead of "execute", because its expected that
// Terminal owned by an App and these methods called by its visual in draw method
// and then flushed by the app.

impl<W: Write> WriteBuffer for Terminal<W> {
    fn write_buffer(
        &mut self,
        position: Position,
        buffer: &dyn ReadBuffer,
    ) -> Result<(), Box<dyn Error>> {
        // #[not_tested]
        // check bounds
        let width = buffer.size().width;
        let bottom = position.y + buffer.size().height;
        let mut position = position;

        while position.y < bottom {
            self.set_cursor(position)?;
            self.writer.write(buffer.bytes(position, width))?;

            position.y += 1;
        }

        Ok(())
    }

    fn set_forecolor(&mut self, position: Position, color: Color) -> Result<(), Box<dyn Error>> {
        self.set_cursor(position)?;

        match self.writer.queue(SetForegroundColor(color)) {
            Ok(_) => Ok(()),
            Err(error) => Err(Box::new(error)),
        }
    }

    fn set_backcolor(&mut self, position: Position, color: Color) -> Result<(), Box<dyn Error>> {
        self.set_cursor(position)?;

        match self.writer.queue(SetBackgroundColor(color)) {
            Ok(_) => Ok(()),
            Err(error) => Err(Box::new(error)),
        }
    }

    fn set_symbol(&mut self, position: Position, symbol: char) -> Result<(), Box<dyn Error>> {
        self.set_cursor(position)?;

        let mut utf_8: [u8; 4] = [0; 4];
        symbol.encode_utf8(&mut utf_8);

        match self.writer.write(&utf_8) {
            Ok(_) => Ok(()),
            Err(error) => Err(Box::new(error)),
        }
    }

    fn set_underline_color(
        &mut self,
        position: Position,
        color: Color,
    ) -> Result<(), Box<dyn Error>> {
        self.set_cursor(position)?;

        match self.writer.queue(SetUnderlineColor(color)) {
            Ok(_) => Ok(()),
            Err(error) => Err(Box::new(error)),
        }
    }

    fn set_attribute(
        &mut self,
        position: Position,
        attribute: Attribute,
    ) -> Result<(), Box<dyn Error>> {
        self.set_cursor(position)?;

        match self.writer.queue(SetAttribute(attribute)) {
            Ok(_) => Ok(()),
            Err(error) => Err(Box::new(error)),
        }
    }

    fn clear(&mut self) -> Result<(), Box<dyn Error>> {
        match self.writer.queue(Clear(ClearType::All)) {
            Ok(_) => Ok(()),
            Err(error) => Err(Box::new(error)),
        }
    }

    fn clear_region(&mut self, rect: Rect) -> Result<(), Box<dyn Error>> {
        // #[not_tested]
        // check bounds

        let whitespaces: &[u8] = &vec![' ' as u8; rect.width as usize];

        let mut position = Position::new(rect.x, rect.y);

        while position.y < rect.bottom() {
            self.set_cursor(position)?;
            self.writer.write(whitespaces)?;

            position.y += 1;
        }

        Ok(())
    }

    fn write_symbols(
        &mut self,
        position: Position,
        symbols: &str,
        style: Style,
    ) -> Result<(), Box<dyn Error>> {
        self.set_cursor(position)?;
        self.set_style(position, style)?;

        match self.writer.write(symbols.as_bytes()) {
            Ok(_) => Ok(()),
            Err(error) => Err(Box::new(error)),
        }
    }

    fn set_style(&mut self, position: Position, style: Style) -> Result<(), Box<dyn Error>> {
        self.set_cursor(position)?;

        self.writer
            .queue(SetColors(Colors::new(style.foreground, style.background)))?;
        self.writer
            .queue(SetUnderlineColor(style.underline_color))?;

        match self.writer.queue(SetAttributes(style.attributes)) {
            Ok(_) => Ok(()),
            Err(error) => Err(Box::new(error)),
        }
    }

    fn set_attributes(
        &mut self,
        position: Position,
        attributes: crossterm::style::Attributes,
    ) -> Result<(), Box<dyn Error>> {
        self.set_cursor(position)?;

        match self.writer.queue(SetAttributes(attributes)) {
            Ok(_) => Ok(()),
            Err(error) => Err(Box::new(error)),
        }
    }
}
