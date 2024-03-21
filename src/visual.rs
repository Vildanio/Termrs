mod content_visual;
mod mutable_context;
mod tree_visual;
mod visuals;

pub use content_visual::*;
pub use mutable_context::*;
pub use tree_visual::*;
pub use visuals::*;

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
