mod event_args;
mod input_handler;

pub use event_args::*;
pub use input_handler::*;
pub use {
    crossterm::event::poll, crossterm::event::read, crossterm::event::Event,
    crossterm::event::KeyCode, crossterm::event::KeyEvent, crossterm::event::KeyEventKind,
    crossterm::event::KeyEventState, crossterm::event::KeyModifiers,
    crossterm::event::KeyboardEnhancementFlags, crossterm::event::MediaKeyCode,
    crossterm::event::ModifierKeyCode, crossterm::event::MouseButton, crossterm::event::MouseEvent,
    crossterm::event::MouseEventKind,
};
