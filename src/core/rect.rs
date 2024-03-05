use std::{cmp::{max, min}, fmt};

use crate::{visual::{Margin, Offset}, Position, Size};

/// A Rectangular area.
///
/// A simple rectangle used in the computation of the layout and to give widgets a hint about the
/// area they are supposed to render to.
#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Hash)]
pub struct Rect {
    /// The x coordinate of the top left corner of the rect.
    pub x: u16,
    /// The y coordinate of the top left corner of the rect.
    pub y: u16,
    /// The width of the rect.
    pub width: u16,
    /// The height of the rect.
    pub height: u16,
}

impl fmt::Display for Rect {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}x{}+{}+{}", self.width, self.height, self.x, self.y)
    }
}

impl Rect {
    /// Creates a new rect, with width and height limited to keep the area under max u16. If
    /// clipped, aspect ratio will be preserved.
    pub fn new(x: u16, y: u16, width: u16, height: u16) -> Rect {
        let max_area = u16::max_value();
        let (clipped_width, clipped_height) =
            if u32::from(width) * u32::from(height) > u32::from(max_area) {
                let aspect_ratio = f64::from(width) / f64::from(height);
                let max_area_f = f64::from(max_area);
                let height_f = (max_area_f / aspect_ratio).sqrt();
                let width_f = height_f * aspect_ratio;
                (width_f as u16, height_f as u16)
            } else {
                (width, height)
            };
        Rect {
            x,
            y,
            width: clipped_width,
            height: clipped_height,
        }
    }

    /// The area of the rect. If the area is larger than the maximum value of u16, it will be
    /// clamped to u16::MAX.
    pub const fn area(self) -> u16 {
        self.width.saturating_mul(self.height)
    }

    /// Returns true if the rect has no area.
    pub const fn is_empty(self) -> bool {
        self.width == 0 || self.height == 0
    }

    /// Returns the left coordinate of the rect.
    pub const fn left(self) -> u16 {
        self.x
    }

    /// Returns the right coordinate of the rect. This is the first coordinate outside of the rect.
    ///
    /// If the right coordinate is larger than the maximum value of u16, it will be clamped to
    /// u16::MAX.
    pub const fn right(self) -> u16 {
        self.x.saturating_add(self.width)
    }

    /// Returns the top coordinate of the rect.
    pub const fn top(self) -> u16 {
        self.y
    }

    /// Returns the bottom coordinate of the rect. This is the first coordinate outside of the rect.
    ///
    /// If the bottom coordinate is larger than the maximum value of u16, it will be clamped to
    /// u16::MAX.
    pub const fn bottom(self) -> u16 {
        self.y.saturating_add(self.height)
    }

    /// Returns a new rect inside the current one, with the given margin on each side.
    ///
    /// If the margin is larger than the rect, the returned rect will have no area.
    pub fn inner(self, margin: &Margin) -> Rect {
        let margin_horizontal = margin.left + margin.right;
        let margin_vertical = margin.top + margin.bottom;

        if self.width < margin_horizontal || self.height < margin_vertical {
            Rect::default()
        } else {
            Rect {
                x: self.x.saturating_add(margin.left),
                y: self.y.saturating_add(margin.top),
                width: self.width.saturating_sub(margin_horizontal),
                height: self.height.saturating_sub(margin_vertical),
            }
        }
    }

    /// Moves the `Rect` without modifying its size.
    ///
    /// Moves the `Rect` according to the given offset without modifying its [`width`](Rect::width)
    /// or [`height`](Rect::height).
    /// - Positive `x` moves the whole `Rect` to the right, negative to the left.
    /// - Positive `y` moves the whole `Rect` to the bottom, negative to the top.
    ///
    /// See [`Offset`] for details.
    pub fn offset(self, offset: Offset) -> Rect {
        Rect {
            x: i32::from(self.x)
                .saturating_add(offset.x)
                .clamp(0, (u16::MAX - self.width) as i32) as u16,
            y: i32::from(self.y)
                .saturating_add(offset.y)
                .clamp(0, (u16::MAX - self.height) as i32) as u16,
            ..self
        }
    }

    /// Returns a new rect that contains both the current one and the given one.
    pub fn union(self, other: Rect) -> Rect {
        let x1 = min(self.x, other.x);
        let y1 = min(self.y, other.y);
        let x2 = max(self.right(), other.right());
        let y2 = max(self.bottom(), other.bottom());
        Rect {
            x: x1,
            y: y1,
            width: x2.saturating_sub(x1),
            height: y2.saturating_sub(y1),
        }
    }

    /// Returns a new rect that is the intersection of the current one and the given one.
    ///
    /// If the two rects do not intersect, the returned rect will have no area.
    pub fn intersection(self, other: Rect) -> Rect {
        let x1 = max(self.x, other.x);
        let y1 = max(self.y, other.y);
        let x2 = min(self.right(), other.right());
        let y2 = min(self.bottom(), other.bottom());
        Rect {
            x: x1,
            y: y1,
            width: x2.saturating_sub(x1),
            height: y2.saturating_sub(y1),
        }
    }

    /// Returns true if the two rects intersect.
    pub const fn intersects(self, other: Rect) -> bool {
        self.x < other.right()
            && self.right() > other.x
            && self.y < other.bottom()
            && self.bottom() > other.y
    }

    pub const fn contains(self, position: Position) -> bool {
        position.x >= self.x
            && position.x < self.right()
            && position.y >= self.y
            && position.y < self.bottom()
    }

    pub fn clamp(self, other: Rect) -> Rect {
        let width = self.width.min(other.width);
        let height = self.height.min(other.height);
        let x = self.x.clamp(other.x, other.right().saturating_sub(width));
        let y = self.y.clamp(other.y, other.bottom().saturating_sub(height));
        Rect::new(x, y, width, height)
    }
    
    pub const fn as_position(self) -> Position {
        Position {
            x: self.x,
            y: self.y,
        }
    }

    /// Converts the rect into a size struct.
    pub const fn as_size(self) -> Size {
        Size {
            width: self.width,
            height: self.height,
        }
    }
}

impl From<(Position, Size)> for Rect {
    fn from((position, size): (Position, Size)) -> Self {
        Rect {
            x: position.x,
            y: position.y,
            width: size.width,
            height: size.height,
        }
    }
}

impl From<Size> for Rect {
    fn from(size: Size) -> Self {
        Rect {
            x: 0,
            y: 0,
            width: size.width,
            height: size.height,
        }
    }
}