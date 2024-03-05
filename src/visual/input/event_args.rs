use crossterm::event::{KeyEvent, MouseEvent};

use super::{KeyCode, KeyEventState, KeyModifiers, MouseButton};
use crate::Position;

pub struct KeyEventArgs {
    /// The key itself.
    pub code: KeyCode,

    /// Additional key modifiers.
    pub modifiers: KeyModifiers,
    /// Keyboard state.
    ///
    /// Only set if [`crate::input::KeyboardEnhancementFlags::DISAMBIGUATE_ESCAPE_CODES`] has been enabled.
    pub state: KeyEventState,
}

impl KeyEventArgs {
    pub fn new(code: KeyCode, modifiers: KeyModifiers, state: KeyEventState) -> Self {
        Self {
            code,
            modifiers,
            state,
        }
    }
    
    pub fn from_event(event: &KeyEvent) -> KeyEventArgs {
        Self {
            code: event.code,
            modifiers: event.modifiers,
            state: event.state
        }
    }
}

pub struct MouseEventArgs {
    /// Position of the cell that the event occurred on.
    pub position: Position,
    /// The key modifiers active when the event occurred.
    pub modifiers: KeyModifiers,
}

impl MouseEventArgs {
    pub fn new(position: Position, modifiers: KeyModifiers) -> Self {
        Self {
            position,
            modifiers,
        }
    }

    pub fn from_event(event: &MouseEvent) -> MouseEventArgs {
        Self {
            position: Position::new(event.column, event.row),
            modifiers: event.modifiers,
        }
    }
}

pub struct MouseWheelEventArgs {
    /// Position of the cell that the event occurred on.
    pub position: Position,
    /// The key modifiers active when the event occurred.
    pub modifiers: KeyModifiers,
    pub delta: i16,
    pub is_vertical: bool,
}

impl MouseWheelEventArgs {
    pub fn new(position: Position, modifiers: KeyModifiers, delta: i16, is_vertical: bool) -> Self {
        Self {
            position,
            modifiers,
            delta,
            is_vertical,
        }
    }

    pub fn from_event(event: &MouseEvent, delta: i16, is_vertical: bool) -> MouseWheelEventArgs {
        Self {
            position: Position::new(event.column, event.row),
            modifiers: event.modifiers,
            delta,
            is_vertical,
        }
    }
}

pub struct MouseButtonEventArgs {
    /// Position of the cell that the event occurred on.
    pub position: Position,
    /// The key modifiers active when the event occurred.
    pub modifiers: KeyModifiers,
    /// The mouse button which caused the event.
    pub button: MouseButton,
}

impl MouseButtonEventArgs {
    pub fn new(position: Position, modifiers: KeyModifiers, buttons: MouseButton) -> Self {
        Self { position, modifiers, button: buttons }
    }

    pub fn from_event(event: &MouseEvent, button: MouseButton) -> MouseButtonEventArgs {
        Self {
            position: Position::new(event.column, event.row),
            modifiers: event.modifiers,
            button
        }
    }
}


pub struct PasteEventArgs<'a> {
    text: &'a String,
}

impl<'a> PasteEventArgs<'a> {
    pub fn new(text: &'a String) -> Self {
        Self { text }
    }

    pub fn text(&self) -> &str {
        &self.text
    }
}
