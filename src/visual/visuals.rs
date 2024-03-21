mod text_block;
mod vstack;

pub use text_block::TextBlock;

/// Creates a vertical stack visual with the provided visuals as children.
#[macro_export]
macro_rules! vstack {
    ($($x:expr),*) => {{
        let mut children = termrs::visuals![$($x),*];

        termrs::visual::TreeVisual::new(termrs::layout::VStackLayout, termrs::input::EmptyVisualLeafInput, children)
    }};
}

/// Creates a [`TextBlock`].
#[macro_export]
macro_rules! text_block {
    ($text:expr) => {
        TextBlock::new($text)
    };
}

/// Creates a [`Vec`] of boxed visuals.
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
