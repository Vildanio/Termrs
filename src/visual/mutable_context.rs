use std::process::ExitCode;

/// Provides mutable access to application state.
pub trait MutableContext {
    /// Forces a complete redraw of the visual. This is necessary as a visual element should not
    /// be aware of its own size - this is the responsibility of the parent element.
    fn redraw(&mut self);

    /// Sets the focus for the visual.
    fn set_focus(&mut self, value: bool);

    /// Provides a proper way to terminate the process. The exit code is specified by the
    /// `ExitCode` parameter.
    fn terminate_app(&mut self, exit_code: ExitCode);
}

/// Represents an action on [`MutableContext`]
pub enum MutableContextAction {
    Redraw,
    SetFocus(bool),
    Terminate(ExitCode),
}

impl MutableContextAction {
    pub fn apply(&self, visual_context: &mut dyn MutableContext) {
        match self {
            MutableContextAction::Redraw => visual_context.redraw(),
            MutableContextAction::SetFocus(value) => visual_context.set_focus(*value),
            MutableContextAction::Terminate(exit_code) => visual_context.terminate_app(*exit_code),
        }
    }
}

/// Saves all calls as [`MutableContextAction`]
pub struct RetainedMutableContext<'a> {
    actions: &'a mut Vec<MutableContextAction>,
}

impl<'a> RetainedMutableContext<'a> {
    pub fn new(actions: &'a mut Vec<MutableContextAction>) -> Self {
        Self { actions }
    }
}

impl<'a> MutableContext for RetainedMutableContext<'a> {
    fn set_focus(&mut self, value: bool) {
        self.actions.push(MutableContextAction::SetFocus(value));
    }

    fn redraw(&mut self) {
        self.actions.push(MutableContextAction::Redraw);
    }

    fn terminate_app(&mut self, exit_code: ExitCode) {
        self.actions
            .push(MutableContextAction::Terminate(exit_code));
    }
}
