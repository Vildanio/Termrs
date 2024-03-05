use crate::{buffer::WriteBuffer, Size};

use super::{
    input::{
        EmptyVisualLeafInput, KeyEventArgs, MouseButtonEventArgs, MouseEventArgs,
        MouseWheelEventArgs, PasteEventArgs, VisualInput, VisualLeafInput,
    }, Draw, Layout, MutableContext, RetainedVisualContext, VStackLayout, Visual, VisualContextAction
};

/// Composes other visuals to implement the [`Visual`].
pub struct TreeVisual<'a> {
    /// The drawing and measuring strategy.
    layout: Box<dyn Layout>,

    /// Visual which handled input events.
    ///
    /// Dont forget to validate this value when chldren changed!
    focused: Option<&'a mut Box<dyn Visual>>,

    /// Object which handles input before children.
    input_handler: Box<dyn VisualLeafInput>,

    /// Visuals which used by layout
    children: Vec<Box<dyn Visual>>,
}

impl<'a> TreeVisual<'a> {
    pub fn new(
        layout: Box<dyn Layout>,
        input_handler: Box<dyn VisualLeafInput>,
        children: Vec<Box<dyn Visual>>,
    ) -> Self {
        Self {
            layout,
            children,
            input_handler,
            focused: None,
        }
    }

    pub fn vstack() -> TreeVisual<'a> {
        TreeVisual::new(
            Box::new(VStackLayout),
            Box::new(EmptyVisualLeafInput),
            vec![],
        )
    }

    // i think it is not neccessary to declare set_layout method
    // because there is no much cases when it is needed.

    pub fn set_children(&mut self, children: Vec<Box<dyn Visual>>) {
        self.children = children;
    }

    pub fn set_input_handler(&mut self, input_handler: Box<dyn VisualLeafInput>) {
        self.input_handler = input_handler;
    }

    // TODO: Replace with abstract iterator
    pub fn children(&self) -> &Vec<Box<dyn Visual>> {
        &self.children
    }

    fn apply_actions(actions: &Vec<VisualContextAction>, visual_context: &mut dyn MutableContext) {
        // Apply the actions to the mutable context
        for action in actions {
            action.apply(visual_context);
        }
    }
}

impl<'a> VisualInput for TreeVisual<'a> {
    fn on_paste(&mut self, args: &PasteEventArgs, visual_context: &mut dyn MutableContext) -> bool {
        // All actions that this visual will perform on the given context.
        // It consist of actions that apply:
        // 1. input_handler
        // 2. focused child
        let mut actions: Vec<VisualContextAction> = vec![];

        let mut bubble_handled = false;
        let tunnel_handled = self
            .input_handler
            .tunnel_paste(args, &mut RetainedVisualContext::new(&mut actions));

        if !tunnel_handled {
            if let Some(ref mut focused) = self.focused {
                bubble_handled = focused.on_paste(args, visual_context);
            }
        }

        if !bubble_handled {
            bubble_handled = self
                .input_handler
                .bubble_paste(args, &mut RetainedVisualContext::new(&mut actions));
        }

        // Apply the actions to the mutable context
        Self::apply_actions(&actions, visual_context);

        bubble_handled
    }

    fn on_got_focus(&mut self, visual_context: &mut dyn MutableContext) {
        let mut actions: Vec<VisualContextAction> = vec![];

        self.input_handler
            .on_got_focus(&mut RetainedVisualContext::new(&mut actions));

        for child in self.children.iter_mut() {
            child.on_got_focus(&mut RetainedVisualContext::new(&mut actions));
        }

        Self::apply_actions(&actions, visual_context);
    }

    fn on_lost_focus(&mut self, visual_context: &mut dyn MutableContext) {
        let mut actions: Vec<VisualContextAction> = vec![];

        self.input_handler
            .on_lost_focus(&mut RetainedVisualContext::new(&mut actions));

        for child in self.children.iter_mut() {
            child.on_lost_focus(&mut RetainedVisualContext::new(&mut actions));
        }

        Self::apply_actions(&actions, visual_context);
    }

    fn on_key_press(
        &mut self,
        args: &KeyEventArgs,
        visual_context: &mut dyn MutableContext,
    ) -> bool {
        let mut actions: Vec<VisualContextAction> = vec![];

        let mut bubble_handled = false;
        let tunnel_handled = self
            .input_handler
            .tunnel_key_press(args, &mut RetainedVisualContext::new(&mut actions));

        if !tunnel_handled {
            if let Some(ref mut focused) = self.focused {
                bubble_handled = focused.on_key_press(args, visual_context);
            }
        }

        if !bubble_handled {
            bubble_handled = self
                .input_handler
                .bubble_key_press(args, &mut RetainedVisualContext::new(&mut actions));
        }

        Self::apply_actions(&actions, visual_context);

        bubble_handled
    }

    fn on_key_release(
        &mut self,
        args: &KeyEventArgs,
        visual_context: &mut dyn MutableContext,
    ) -> bool {
        let mut actions: Vec<VisualContextAction> = vec![];

        let mut bubble_handled = false;
        let tunnel_handled = self
            .input_handler
            .tunnel_key_release(args, &mut RetainedVisualContext::new(&mut actions));

        if !tunnel_handled {
            if let Some(ref mut focused) = self.focused {
                bubble_handled = focused.on_key_release(args, visual_context);
            }
        }

        if !bubble_handled {
            bubble_handled = self
                .input_handler
                .bubble_key_release(args, &mut RetainedVisualContext::new(&mut actions));
        }

        Self::apply_actions(&actions, visual_context);

        bubble_handled
    }

    fn on_mouse_move(
        &mut self,
        args: &MouseEventArgs,
        visual_context: &mut dyn MutableContext,
    ) -> bool {
        let mut actions: Vec<VisualContextAction> = vec![];

        let mut bubble_handled = false;
        let tunnel_handled = self
            .input_handler
            .tunnel_mouse_move(args, &mut RetainedVisualContext::new(&mut actions));

        if !tunnel_handled {
            if let Some(ref mut focused) = self.focused {
                bubble_handled = focused.on_mouse_move(args, visual_context);
            }
        }

        if !bubble_handled {
            bubble_handled = self
                .input_handler
                .bubble_mouse_move(args, &mut RetainedVisualContext::new(&mut actions));
        }

        Self::apply_actions(&actions, visual_context);

        bubble_handled
    }

    fn on_mouse_wheel(
        &mut self,
        args: &MouseWheelEventArgs,
        visual_context: &mut dyn MutableContext,
    ) -> bool {
        let mut actions: Vec<VisualContextAction> = vec![];

        let mut bubble_handled = false;
        let tunnel_handled = self
            .input_handler
            .tunnel_mouse_wheel(args, &mut RetainedVisualContext::new(&mut actions));

        if !tunnel_handled {
            if let Some(ref mut focused) = self.focused {
                bubble_handled = focused.on_mouse_wheel(args, visual_context);
            }
        }

        if !bubble_handled {
            bubble_handled = self
                .input_handler
                .bubble_mouse_wheel(args, &mut RetainedVisualContext::new(&mut actions));
        }

        Self::apply_actions(&actions, visual_context);

        bubble_handled
    }

    fn on_mouse_up(
        &mut self,
        args: &MouseButtonEventArgs,
        visual_context: &mut dyn MutableContext,
    ) -> bool {
        let mut actions: Vec<VisualContextAction> = vec![];

        let mut bubble_handled = false;
        let tunnel_handled = self
            .input_handler
            .tunnel_mouse_up(args, &mut RetainedVisualContext::new(&mut actions));

        if !tunnel_handled {
            if let Some(ref mut focused) = self.focused {
                bubble_handled = focused.on_mouse_up(args, visual_context);
            }
        }

        if !bubble_handled {
            bubble_handled = self
                .input_handler
                .bubble_mouse_up(args, &mut RetainedVisualContext::new(&mut actions));
        }

        Self::apply_actions(&actions, visual_context);

        bubble_handled
    }

    fn on_mouse_down(
        &mut self,
        args: &MouseButtonEventArgs,
        visual_context: &mut dyn MutableContext,
    ) -> bool {
        let mut actions: Vec<VisualContextAction> = vec![];

        let mut bubble_handled = false;
        let tunnel_handled = self
            .input_handler
            .tunnel_mouse_down(args, &mut RetainedVisualContext::new(&mut actions));

        if !tunnel_handled {
            if let Some(ref mut focused) = self.focused {
                bubble_handled = focused.on_mouse_down(args, visual_context);
            }
        }

        if !bubble_handled {
            bubble_handled = self
                .input_handler
                .bubble_mouse_down(args, &mut RetainedVisualContext::new(&mut actions));
        }

        Self::apply_actions(&actions, visual_context);

        bubble_handled
    }
}

impl<'a> Draw for TreeVisual<'a> {
    fn draw(&self, buffer: &mut dyn WriteBuffer, available_size: Size) -> Size {
        let arrange = self.layout.draw(&self.children, buffer, available_size);

        arrange.size()
    }

    fn measure(&self, constraints: Size) -> Size {
        self.layout.measure(&self.children, constraints)
    }
}

impl<'a> Visual for TreeVisual<'a> {}