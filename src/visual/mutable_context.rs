use std::process::ExitCode;

/// `MutableContext` provides mutable access to a visual.
pub trait MutableContext {
    /// Forces a complete redraw of the visual. This is necessary as a visual element should not
    /// be aware of its own size - this is the responsibility of the parent element.
    fn redraw(&mut self);

    /// Sets the focus for the visual. This should be done in the `VisualContext` and not in the
    /// visual itself, as changing focus should trigger `on_got_focus` and `on_lost_focus` events.
    fn set_focus(&mut self, value: bool);

    /// Provides a proper way to terminate the process. The exit code is specified by the
    /// `ExitCode` parameter.
    fn terminate_app(&mut self, exit_code: ExitCode);
}

/// Represents an action on [`VisualContext`]
pub enum VisualContextAction {
    Redraw,
    SetFocus(bool),
    Terminate(ExitCode),
}

impl VisualContextAction {
    pub fn apply(&self, visual_context: &mut dyn MutableContext) {
        match self {
            VisualContextAction::Redraw => visual_context.redraw(),
            VisualContextAction::SetFocus(value) => visual_context.set_focus(*value),
            VisualContextAction::Terminate(exit_code) => visual_context.terminate_app(*exit_code),
        }
    }
}

/// Saves all calls as [`VisualContextAction`] vect
/// with ability to apply them to another visual context later.
pub struct RetainedVisualContext<'a> {
    actions: &'a mut Vec<VisualContextAction>,
}

// TODO: Should i implement MutableContext directly on Vec<VisualContextAction>?

impl<'a> RetainedVisualContext<'a> {
    pub fn new(actions: &'a mut Vec<VisualContextAction>) -> Self {
        Self { actions }
    }
}

impl<'a> MutableContext for RetainedVisualContext<'a> {
    fn set_focus(&mut self, value: bool) {
        self.actions.push(VisualContextAction::SetFocus(value));
    }

    fn redraw(&mut self) {
        self.actions.push(VisualContextAction::Redraw);
    }

    fn terminate_app(&mut self, exit_code: ExitCode) {
        self.actions.push(VisualContextAction::Terminate(exit_code));
    }
}
