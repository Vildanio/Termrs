use std::{
    io::{self, Write},
    process::ExitCode,
};

use crate::{
    buffer::{Terminal, VirtualBuffer},
    input::{
        Event, KeyEventArgs, KeyEventKind, MouseButtonEventArgs, MouseEventArgs, MouseEventKind,
        MouseWheelEventArgs, PasteEventArgs,
    },
    visual::{MutableContextAction, RetainedMutableContext, Visual},
    EventLoop, Size,
};

/// Runs [`Visual`] using the given visual as a root visual
/// and created [`Terminal`] from stdout.
pub fn run_app(visual: impl Visual) -> ExitCode {
    let terminal = Terminal::<io::Stdout>::from_stdout();

    let mut app = VisualApp::new(visual, terminal);

    EventLoop::default().run(&mut app)
}

/// Provides event loop and root mutable context.
pub struct VisualApp<V, W>
where
    W: Write,
    V: Visual,
{
    visual: V,
    terminal: Terminal<W>,
    /// Is the root visual focused or not
    is_focused: bool,
}

/// Processes events from [`EventLoop`].
pub trait EventHandler {
    /// Processes event loop start.
    fn on_start(&mut self);

    /// Processes the given event and returns value indicating
    /// whether event loop should exit or not.
    fn on_event(&mut self, event: &Event) -> Option<ExitCode>;

    /// Processes event loop exit.
    fn on_exit(&mut self);
}

impl<V, W> EventHandler for VisualApp<V, W>
where
    W: Write,
    V: Visual,
{
    fn on_start(&mut self) {
        crossterm::terminal::enable_raw_mode().unwrap();

        // draw for the first time
        self.redraw();
    }

    fn on_event(&mut self, event: &Event) -> Option<ExitCode> {
        let mut actions = vec![];
        let context = &mut RetainedMutableContext::new(&mut actions);

        match event {
            Event::FocusGained => self.visual.on_got_focus(context),
            Event::FocusLost => self.visual.on_lost_focus(context),
            Event::Key(key_event) => match key_event.kind {
                KeyEventKind::Press | KeyEventKind::Repeat => {
                    self.visual
                        .on_key_press(&KeyEventArgs::from_event(key_event), context);
                }
                KeyEventKind::Release => {
                    self.visual
                        .on_key_release(&KeyEventArgs::from_event(key_event), context);
                }
            },
            Event::Mouse(mouse_event) => match mouse_event.kind {
                MouseEventKind::Down(mouse_button) => {
                    self.visual.on_mouse_down(
                        &MouseButtonEventArgs::from_event(mouse_event, mouse_button),
                        context,
                    );
                }
                MouseEventKind::Up(mouse_button) => {
                    self.visual.on_mouse_up(
                        &MouseButtonEventArgs::from_event(mouse_event, mouse_button),
                        context,
                    );
                }
                MouseEventKind::Drag(_) => {
                    self.visual
                        .on_mouse_move(&MouseEventArgs::from_event(mouse_event), context);
                }
                MouseEventKind::Moved => {
                    self.visual
                        .on_mouse_move(&MouseEventArgs::from_event(mouse_event), context);
                }
                MouseEventKind::ScrollDown => {
                    self.visual.on_mouse_wheel(
                        &MouseWheelEventArgs::from_event(mouse_event, 1, true),
                        context,
                    );
                }
                MouseEventKind::ScrollUp => {
                    self.visual.on_mouse_wheel(
                        &MouseWheelEventArgs::from_event(mouse_event, -1, true),
                        context,
                    );
                }
                MouseEventKind::ScrollLeft => {
                    self.visual.on_mouse_wheel(
                        &MouseWheelEventArgs::from_event(mouse_event, -1, false),
                        context,
                    );
                }
                MouseEventKind::ScrollRight => {
                    self.visual.on_mouse_wheel(
                        &MouseWheelEventArgs::from_event(mouse_event, 1, false),
                        context,
                    );
                }
            },
            Event::Paste(str) => {
                self.visual.on_paste(&PasteEventArgs::new(str), context);
            }
            Event::Resize(column, row) => {
                let desired_size = self
                    .visual
                    .measure(Size::new(column.to_owned(), row.to_owned()));

                let size = desired_size.clip(self.terminal.size());

                let mut virtual_buffer = VirtualBuffer::new(&mut self.terminal, size.into());

                self.visual.draw(&mut virtual_buffer, size);
            }
        }

        let mut exit_code = None;
        let mut redraw = false;
        let mut is_focused = self.is_focused;

        for action in actions {
            match action {
                MutableContextAction::Redraw => redraw = true,
                MutableContextAction::SetFocus(focus) => is_focused = focus,
                MutableContextAction::Terminate(exit_code_val) => exit_code = Some(exit_code_val),
            }
        }

        if redraw {
            self.redraw();
        }

        self.is_focused = is_focused;
        exit_code
    }

    fn on_exit(&mut self) {
        crossterm::terminal::disable_raw_mode().unwrap();
    }
}

impl<V, W> VisualApp<V, W>
where
    W: Write,
    V: Visual,
{
    pub fn new(visual: V, terminal: Terminal<W>) -> Self {
        Self {
            visual,
            terminal,
            is_focused: false,
        }
    }

    pub fn redraw(&mut self) {
        let available_size = self.terminal.size();

        let desired_size = self.visual.measure(available_size);

        let draw_size = desired_size.clip(available_size);

        self.visual.draw(&mut self.terminal, draw_size);

        self.terminal.flush().unwrap();
    }
}
