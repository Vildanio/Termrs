pub use crossterm::style::{Attribute, Attributes, Color};

pub struct Style {
    pub background: Option<Color>,
    pub foreground: Option<Color>,
    pub underline_color: Option<Color>,
    pub attributes: Attributes,
}

impl Default for Style {
    fn default() -> Self {
        Self {
            background: Default::default(),
            foreground: Default::default(),
            underline_color: Default::default(),
            attributes: Default::default(),
        }
    }
}

impl Style {
    pub fn new(
        background: Option<Color>,
        foreground: Option<Color>,
        underline_color: Option<Color>,
        attributes: Attributes,
    ) -> Self {
        Self {
            background,
            foreground,
            underline_color,
            attributes,
        }
    }

    pub fn background(mut self, color: Color) -> Self {
        self.background = Some(color);
        self
    }

    pub fn foreground(mut self, color: Color) -> Self {
        self.foreground = Some(color);
        self
    }

    pub fn underline_color(mut self, color: Color) -> Self {
        self.underline_color = Some(color);
        self
    }

    pub fn attributes(mut self, attributes: Attributes) -> Self {
        self.attributes = attributes;
        self
    }

    pub fn reset() -> Style {
        Self {
            background: Some(Color::Reset),
            foreground: Some(Color::Reset),
            underline_color: Some(Color::Reset),
            attributes: Default::default(),
        }
    }
}

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
    fn set_style<S: Into<Style>>(self, style: S) -> Self::Item;
}

pub trait Stylize<'a, T>: Sized {
    fn background(self, color: Color) -> T;
    fn foreground<S: Into<Color>>(self, color: S) -> T;
    fn reset(self) -> T;
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

    fn foreground<S: Into<Color>>(self, color: S) -> T {
        let style = self.style().foreground(color.into());
        self.set_style(style)
    }

    fn reset(self) -> T {
        self.set_style(Style::reset())
    }

    fn attributes(self, attributes: Attributes) -> T {
        let style = self.style().attributes(attributes);

        self.set_style(style)
    }
}
