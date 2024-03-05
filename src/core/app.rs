use std::{
    io::{self, Write},
    process::ExitCode,
    time::{Duration, Instant},
};

use crossterm::event::{self, Event, KeyEventKind, MouseEventKind};

use crate::{
    buffer::{Terminal, VirtualBuffer},
    visual::{
        input::{
            KeyEventArgs, MouseButtonEventArgs, MouseEventArgs, MouseWheelEventArgs,
            PasteEventArgs, VisualInput,
        },
        MutableContext, RetainedVisualContext, Visual,
    },
    Rect, Size,
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
        let mut context = RetainedVisualContext::new(&mut actions);

        match event {
            Event::FocusGained => self.visual.on_got_focus(&mut context),
            Event::FocusLost => self.visual.on_lost_focus(&mut context),
            Event::Key(key_event) => match key_event.kind {
                KeyEventKind::Press | KeyEventKind::Repeat => {
                    self.visual
                        .on_key_press(&KeyEventArgs::from_event(key_event), &mut context);
                }
                KeyEventKind::Release => {
                    self.visual
                        .on_key_release(&KeyEventArgs::from_event(key_event), &mut context);
                }
            },
            Event::Mouse(mouse_event) => match mouse_event.kind {
                MouseEventKind::Down(mouse_button) => {
                    self.visual.on_mouse_down(
                        &MouseButtonEventArgs::from_event(mouse_event, mouse_button),
                        &mut context,
                    );
                }
                MouseEventKind::Up(mouse_button) => {
                    self.visual.on_mouse_up(
                        &MouseButtonEventArgs::from_event(mouse_event, mouse_button),
                        &mut context,
                    );
                }
                MouseEventKind::Drag(_) => {
                    self.visual
                        .on_mouse_move(&MouseEventArgs::from_event(mouse_event), &mut context);
                }
                MouseEventKind::Moved => {
                    self.visual
                        .on_mouse_move(&MouseEventArgs::from_event(mouse_event), &mut context);
                }
                MouseEventKind::ScrollDown => {
                    self.visual.on_mouse_wheel(
                        &MouseWheelEventArgs::from_event(mouse_event, 1, true),
                        &mut context,
                    );
                }
                MouseEventKind::ScrollUp => {
                    self.visual.on_mouse_wheel(
                        &MouseWheelEventArgs::from_event(mouse_event, -1, true),
                        &mut context,
                    );
                }
                MouseEventKind::ScrollLeft => {
                    self.visual.on_mouse_wheel(
                        &MouseWheelEventArgs::from_event(mouse_event, -1, false),
                        &mut context,
                    );
                }
                MouseEventKind::ScrollRight => {
                    self.visual.on_mouse_wheel(
                        &MouseWheelEventArgs::from_event(mouse_event, 1, false),
                        &mut context,
                    );
                }
            },
            Event::Paste(str) => {
                self.visual
                    .on_paste(&PasteEventArgs::new(str), &mut context);
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
    }

    pub fn redraw(&mut self) {
        let available_size = self.terminal.size();

        let desired_size = self.visual.measure(available_size);

        let draw_size = desired_size.clip(available_size);

        self.visual.draw(&mut self.terminal, draw_size);

        self.terminal.flush();
    }

    pub fn redraw_region(&mut self, region: Rect) {
        let mut constrained_buffer = VirtualBuffer::new(Box::new(&mut self.terminal), region);

        self.visual.redraw(&mut constrained_buffer, region);

        self.terminal.flush();
    }
}

pub struct AppContext<'a, V, W>
where
    W: Write,
    V: Visual,
{
    /// Mutable reference to the terminal
    /// and the root visual.
    app: &'a mut App<V, W>,
}

impl<'a, V, W> VisualInput for AppContext<'a, V, W>
where
    W: Write,
    V: Visual,
{
    fn on_paste(&mut self, args: &PasteEventArgs, visual_context: &mut dyn MutableContext) -> bool {
        self.app.visual.on_paste(args, visual_context)
    }

    fn on_got_focus(&mut self, visual_context: &mut dyn MutableContext) {
        self.app.visual.on_got_focus(visual_context)
    }

    fn on_lost_focus(&mut self, visual_context: &mut dyn MutableContext) {
        self.app.visual.on_lost_focus(visual_context)
    }

    fn on_key_press(
        &mut self,
        args: &KeyEventArgs,
        visual_context: &mut dyn MutableContext,
    ) -> bool {
        self.app.visual.on_key_press(args, visual_context)
    }

    fn on_key_release(
        &mut self,
        args: &KeyEventArgs,
        visual_context: &mut dyn MutableContext,
    ) -> bool {
        self.app.visual.on_key_release(args, visual_context)
    }

    fn on_mouse_move(
        &mut self,
        args: &MouseEventArgs,
        visual_context: &mut dyn MutableContext,
    ) -> bool {
        self.app.visual.on_mouse_move(args, visual_context)
    }

    fn on_mouse_wheel(
        &mut self,
        args: &MouseWheelEventArgs,
        visual_context: &mut dyn MutableContext,
    ) -> bool {
        self.app.visual.on_mouse_wheel(args, visual_context)
    }

    fn on_mouse_up(
        &mut self,
        args: &MouseButtonEventArgs,
        visual_context: &mut dyn MutableContext,
    ) -> bool {
        self.app.visual.on_mouse_up(args, visual_context)
    }

    fn on_mouse_down(
        &mut self,
        args: &MouseButtonEventArgs,
        visual_context: &mut dyn MutableContext,
    ) -> bool {
        self.app.visual.on_mouse_down(args, visual_context)
    }
}

impl<'a, V, W> MutableContext for AppContext<'a, V, W>
where
    W: Write,
    V: Visual,
{
    fn set_focus(&mut self, value: bool) {
        self.app.is_focused = value;
    }

    fn redraw(&mut self) {
        self.app.redraw();
    }

    fn redraw_region(&mut self, region: Rect) {
        self.app.redraw_region(region);
    }

    fn remeasure(&mut self) {
        self.app.redraw();
    }

    fn terminate_app(&mut self, exit_code: ExitCode) {
        self.app.exit_code = Some(exit_code);
    }
}
