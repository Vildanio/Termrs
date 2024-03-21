mod border;
mod button;
mod hstack;
mod list;
mod text_block;
mod text_box;
mod tree;
mod vstack;

pub use text_block::TextBlock;

use crate::{layout::VStackLayout, visual::TreeVisual};

pub type VStack<'a, I> = TreeVisual<'a, VStackLayout, I>;

#[macro_export]
macro_rules! vstack {
    ($($x:expr),*) => {{
        let mut children = termrs::visuals![$($x),*];

        VStack::new(termrs::layout::VStackLayout, termrs::input::EmptyVisualLeafInput, children)
    }};
}

#[macro_export]
macro_rules! visuals {
    ($($x:expr),*) => {{
        // TODO(opt): Create vector with capacity.

        let mut children: Vec<Box<dyn termrs::visual::Visual>> = vec![];

        $(
            children.push(Box::new($x));
        )*

        children
    }};
}
