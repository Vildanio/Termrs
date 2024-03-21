use super::{Attributes, Color, Style};

/// A trait for objects that have a `Style`.
///
/// This trait enables generic code to be written that can interact with any object that has a
/// `Style`. This is used by the `Stylize` trait to allow generic code to be written that can
/// interact with any object that can be styled.
pub trait Styled {
    type Item;

    /// Returns the style of the object.
    fn style(&self) -> Style;

    /// Sets the style of the object.
    ///
    /// `style` accepts any type that is convertible to [`Style`] (e.g. [`Style`], [`Color`], or
    /// your own type that implements [`Into<Style>`]).
    fn set_style(self, style: Style) -> Self::Item;
}

pub trait Stylize<'a, T>: Sized {
    fn reset(self) -> T;
    fn background(self, color: Color) -> T;
    fn foreground(self, color: Color) -> T;
    fn underline_color(self, color: Color) -> T;
    fn attributes(self, attributes: Attributes) -> T;
}

impl<'a, T, U> Stylize<'a, T> for U
where
    U: Styled<Item = T>,
{
    fn background(self, color: Color) -> T {
        let style = self.style().background(color);
        self.set_style(style)
    }

    fn foreground(self, color: Color) -> T {
        let style = self.style().foreground(color);
        self.set_style(style)
    }

    fn reset(self) -> T {
        self.set_style(Style::reset())
    }

    fn attributes(self, attributes: Attributes) -> T {
        let style = self.style().attributes(attributes);

        self.set_style(style)
    }

    fn underline_color(self, color: Color) -> T {
        let style = self.style().underline_color(color);
        self.set_style(style)
    }
}
