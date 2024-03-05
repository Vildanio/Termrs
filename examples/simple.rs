use termrs::{
    run_app,
    visual::{TextBlock, TreeVisual},
};

fn main() {
    let mut visual = TreeVisual::vstack();
    visual.set_children(vec![
        Box::new(TextBlock::new("first column")),
        Box::new(TextBlock::new("second column")),
        Box::new(TextBlock::new("third column")),
    ]);

    run_app(visual);
}
