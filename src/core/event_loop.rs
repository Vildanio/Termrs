use std::{
    process::ExitCode,
    time::{Duration, Instant},
};

use crate::{input, EventHandler};

pub struct EventLoop {
    tick_rate: Duration,
}

impl Default for EventLoop {
    fn default() -> Self {
        Self {
            tick_rate: Duration::from_millis(250),
        }
    }
}

impl EventLoop {
    pub fn new(tick_rate: Duration) -> Self {
        Self { tick_rate }
    }

    pub fn run(&self, app: &mut impl EventHandler) -> ExitCode {
        app.on_start();

        let mut last_tick = Instant::now();

        loop {
            let timeout = self.tick_rate.saturating_sub(last_tick.elapsed());

            if input::poll(timeout).unwrap() {
                let event = input::read().unwrap();

                let exit_code = app.on_event(&event);

                // if exit requested
                if let Some(exit_code) = exit_code {
                    app.on_exit();

                    // stop the event loop.
                    break exit_code;
                }
            }

            if last_tick.elapsed() >= self.tick_rate {
                last_tick = Instant::now();
            }
        }
    }
}
