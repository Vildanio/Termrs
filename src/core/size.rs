use crate::Rect;

/// A simple size struct
///
/// The width and height are stored as `u16` values and represent the number of columns and rows
/// respectively.
#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Hash)]
pub struct Size {
    /// The width in columns
    pub width: u16,
    /// The height in rows
    pub height: u16,
}

impl Size {
    /// Create a new `Size` struct
    pub const fn new(width: u16, height: u16) -> Self {
        Size { width, height }
    }

    pub fn clip(&self, other: Size) -> Size {
        Size {
            width: self.width.min(other.width),
            height: self.height.min(other.height),
        }
    }
}

impl From<(u16, u16)> for Size {
    fn from((width, height): (u16, u16)) -> Self {
        Size { width, height }
    }
}

impl From<Rect> for Size {
    fn from(rect: Rect) -> Self {
        rect.as_size()
    }
}
