use std::process::ExitCode;

use termrs::{
    run_app,
    style::Stylize,
    visual::{TextBlock, VStack},
    vstack, Attribute, Color,
};

fn main() -> ExitCode {
    let visual = vstack![
        TextBlock::new("first column").background(Color::Red),
        TextBlock::new("second column").foreground(Color::Cyan),
        TextBlock::new("third column")
            .attributes(Attribute::Underlined.into())
            .underline_color(Color::Blue),
        vstack![
            TextBlock::new("forth column").background(Color::White),
            TextBlock::new("fifth column").foreground(Color::Blue)
        ]
    ];

    run_app(visual)
}
