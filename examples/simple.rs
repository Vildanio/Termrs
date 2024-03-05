use termrs::{run_app, visual::{TextBlock, VisualTree}};

fn main() {
    let mut visual = VisualTree::vstack();
    visual.set_children(vec![
       Box::new(TextBlock::new("first column")), 
       Box::new(TextBlock::new("second column")), 
       Box::new(TextBlock::new("third column")), 
    ]);
    
    run_app(visual);
}