use std::{
    io::{self, Write},
    process::ExitCode,
    time::{Duration, Instant},
};

use crossterm::event::{self, Event, KeyEventKind, MouseEventKind};

use crate::{
    buffer::{Terminal, VirtualBuffer},
    input::{
        KeyEventArgs, MouseButtonEventArgs, MouseEventArgs, MouseWheelEventArgs, PasteEventArgs,
    },
    visual::{RetainedVisualContext, Visual, VisualContextAction},
    Size,
};

const TICK_DURATION: Duration = Duration::from_millis(250);

/// Runs App using the given visual as a root visual.
pub fn run_app(visual: impl Visual) -> ExitCode {
    let terminal = Terminal::<io::Stdout>::from_stdout();

    App::new(visual, terminal).start(TICK_DURATION)
}

/// Provides event loop and root mutable context.
pub struct App<V, W>
where
    W: Write,
    V: Visual,
{
    visual: V,
    terminal: Terminal<W>,
    /// # Returns
    /// None — if termination not requested
    /// Some — if termination was requested
    exit_code: Option<ExitCode>,
    /// Is the root visual focused or not
    is_focused: bool,
}

impl<V, W> App<V, W>
where
    W: Write,
    V: Visual,
{
    pub fn new(visual: V, terminal: Terminal<W>) -> Self {
        Self {
            visual,
            terminal,
            is_focused: false,
            exit_code: None,
        }
    }

    /// Starts loop of input events processing.
    pub fn start(&mut self, tick_rate: Duration) -> ExitCode {
        let mut last_tick = Instant::now();

        crossterm::terminal::enable_raw_mode().expect("Cannot enable raw mode");

        // draw for the first time
        self.redraw();

        loop {
            // exit if requested
            if let Some(exit_code) = self.exit_code {
                crossterm::terminal::disable_raw_mode().expect("Cannot disable raw mode");

                break exit_code;
            }

            let timeout = tick_rate.saturating_sub(last_tick.elapsed());
            if crossterm::event::poll(timeout).unwrap() {
                let event = event::read().unwrap();

                self.process_event(&event);
            }

            if last_tick.elapsed() >= tick_rate {
                last_tick = Instant::now();
            }
        }
    }

    pub fn process_event(&mut self, event: &Event) {
        let mut actions = vec![];
        let context = &mut RetainedVisualContext::new(&mut actions);

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

                let mut virtual_buffer =
                    VirtualBuffer::new(Box::new(&mut self.terminal), size.into());

                self.visual.draw(&mut virtual_buffer, size);
            }
        }

        for action in actions {
            match action {
                VisualContextAction::Redraw => self.redraw(),
                VisualContextAction::SetFocus(focus) => self.is_focused = focus,
                VisualContextAction::Terminate(exit_code) => self.exit_code = Some(exit_code),
            }
        }
    }

    pub fn redraw(&mut self) {
        let available_size = self.terminal.size();

        let desired_size = self.visual.measure(available_size);

        let draw_size = desired_size.clip(available_size);

        self.visual.draw(&mut self.terminal, draw_size);

        self.terminal.flush().expect("Cannot flush terminal");
    }
}
