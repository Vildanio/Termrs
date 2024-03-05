use crate::{
    buffer::WriteBuffer,
    input::{VisualInput, VisualLeafInput},
    Size,
};

pub trait Draw {
    /// Render itself to the given buffer
    /// and returns size of consumed region.
    fn draw(&self, buffer: &mut dyn WriteBuffer, available_size: Size) -> Size;

    /// Measures size of the visual.
    fn measure(&self, constraints: Size) -> Size;
}

pub trait Visual: Draw + VisualInput {}

/// [`Visual`] which has no chlidren.
pub trait VisualLeaf: Draw + VisualLeafInput {}
