use std::process::ExitCode;

use crate::Rect;

/// Provides mutable access to the visual tree.
pub trait MutableContext {
    /// Forces visual to be fully redrawed.
    /// This method is needed because visual should not know its own size,
    /// because this is responsibility of parent.
    fn redraw(&mut self);

    /// Forces the given region of visual to be redrawen
    fn redraw_region(&mut self, region: Rect);

    /// Forces the parent visual to remeasure the visual and be fully redrawed
    /// if the visual bounds changed.
    fn remeasure(&mut self);

    // Focus set should be done in VisualContext
    // but not in visual itself, because
    // focus chagning should raise on_got_focus and on_lost_focus events.

    /// Sets focus for the visual.
    fn set_focus(&mut self, value: bool);

    /// The "proper" way to terminate the process.
    fn terminate_app(&mut self, exit_code: ExitCode);
}

/// Represents an action on [`VisualContext`]
pub enum VisualContextAction {
    RedrawVisual,
    RedrawRegion(Rect),
    Remeasure,
    SetFocus(bool),
    Terminate(ExitCode),
}

impl VisualContextAction {
    pub fn apply(&self, visual_context: &mut dyn MutableContext) {
        match self {
            VisualContextAction::RedrawVisual => visual_context.redraw(),
            VisualContextAction::RedrawRegion(region) => visual_context.redraw_region(*region),
            VisualContextAction::Remeasure => visual_context.remeasure(),
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
        self.actions.push(VisualContextAction::RedrawVisual);
    }

    fn redraw_region(&mut self, region: Rect) {
        self.actions.push(VisualContextAction::RedrawRegion(region));
    }

    fn remeasure(&mut self) {
        self.actions.push(VisualContextAction::Remeasure);
    }

    fn terminate_app(&mut self, exit_code: ExitCode) {
        self.actions.push(VisualContextAction::Terminate(exit_code));
    }
}
