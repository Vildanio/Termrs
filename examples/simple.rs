use termrs::{
    run_app,
    style::Stylize,
    visual::{TextBlock, TreeVisual},
    Attribute, Color,
};

fn main() {
    let visual = TreeVisual::vstack().with_children(vec![
        Box::new(TextBlock::new("first column").background(Color::Red)),
        Box::new(TextBlock::new("second column").foreground(Color::Cyan)),
        Box::new(
            TextBlock::new("third column")
                .attributes(Attribute::Underlined.into())
                .underline_color(Color::Blue),
        ),
    ]);

    run_app(visual);
}
