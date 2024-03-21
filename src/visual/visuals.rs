mod border;
mod button;
mod hstack;
mod list;
mod text_block;
mod text_box;
mod tree;
mod vstack;

pub use text_block::TextBlock;

use crate::{input::EmptyVisualLeafInput, layout::VStackLayout};

use super::{TreeVisual, Visual};

/// Creates a vertical stack visual.
pub fn vstack<'a>(
    children: Vec<Box<dyn Visual>>,
) -> TreeVisual<'a, VStackLayout, EmptyVisualLeafInput> {
    TreeVisual::new(VStackLayout, EmptyVisualLeafInput, children)
}
