mod margin;
mod offset;

pub use {margin::*, offset::*};

use std::vec;

use crate::{
    buffer::{VirtualBuffer, WriteBuffer},
    visual::Visual,
    Position, Rect, Size,
};

/// Provides strategy for drawing and measuring collection of [`Visual`]
pub trait Layout {
    // TODO: Take abstract iterator instead of concrete vector
    /// Draws the children to the given buffer
    fn draw(
        &self,
        children: &[Box<dyn Visual>],
        buffer: &mut dyn WriteBuffer,
        available_size: Size,
    ) -> Box<dyn VisualArrangement>;

    /// Measures extent of the children
    fn measure(&self, children: &[Box<dyn Visual>], constraints: Size) -> Size;
}

pub trait VisualArrangement {
    fn size(&self) -> Size;
    fn size_visual(&self, index: usize) -> Size;
    fn visual_hit(&self, position: Position) -> Option<usize>;
}

/// Defines vertical stack layout.
pub struct VStackLayout;

impl Layout for VStackLayout {
    fn draw(
        &self,
        children: &[Box<dyn Visual>],
        buffer: &mut dyn WriteBuffer,
        available_size: Size,
    ) -> Box<dyn VisualArrangement> {
        let mut rects: Vec<Rect> = vec![];
        let mut draw_size = available_size;
        let mut max_width = 0;
        let mut y = 0;

        for child in children {
            if draw_size.height == 0 {
                break;
            }

            let mut virtual_buffer =
                VirtualBuffer::new(buffer, Rect::new(0, y, draw_size.width, draw_size.height));

            let child_size = child.draw(&mut virtual_buffer, draw_size);

            y += child_size.height;
            draw_size.height -= child_size.height;
            max_width += max_width.max(child_size.width);
            rects.push(Rect::new(0, y, child_size.width, child_size.height));
        }

        Box::new(RectArrangement {
            size: Size::new(max_width, available_size.height - draw_size.height),
            rects,
        })
    }

    fn measure(&self, children: &[Box<dyn Visual>], constraints: Size) -> Size {
        let mut available_size = constraints;
        let mut desired_size = Size::default();

        for child in children {
            if desired_size.width > constraints.width && desired_size.height > constraints.height {
                break;
            }

            let child_size = child.measure(available_size);

            available_size.height -= child_size.height;
            desired_size.height += child_size.height;
            desired_size.width = desired_size.width.max(child_size.width);
        }

        desired_size
    }
}

struct RectArrangement {
    size: Size,
    rects: Vec<Rect>,
}

impl VisualArrangement for RectArrangement {
    fn size(&self) -> Size {
        self.size
    }

    fn size_visual(&self, index: usize) -> Size {
        let rect = self.rects[index];

        rect.as_size()
    }
    fn visual_hit(&self, position: Position) -> Option<usize> {
        for (index, rect) in self.rects.iter().enumerate() {
            if rect.contains(position) {
                return Some(index);
            }
        }

        None
    }
}
