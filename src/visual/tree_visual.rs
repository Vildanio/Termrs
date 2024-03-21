use crate::{
    buffer::WriteBuffer,
    input::{
        KeyEventArgs, MouseButtonEventArgs, MouseEventArgs, MouseWheelEventArgs, PasteEventArgs,
        VisualInput, VisualLeafInput,
    },
    layout::Layout,
    visual::{Draw, MutableContext, Visual},
    Size,
};

/// Composes other visuals to implement the [`Visual`].
pub struct TreeVisual<'a, L, I>
where
    L: Layout,
    I: VisualLeafInput,
{
    /// The drawing and measuring strategy.
    layout: L,

    /// Visual which handled input events.
    ///
    /// Dont forget to validate this value when chldren changed!
    focused: Option<&'a mut Box<dyn Visual>>,

    /// Object which handles input before children.
    input_handler: I,

    /// Visuals which used by layout
    children: Vec<Box<dyn Visual>>,
}

impl<'a, L, I> TreeVisual<'a, L, I>
where
    L: Layout,
    I: VisualLeafInput,
{
    pub fn new(layout: L, input_handler: I, children: Vec<Box<dyn Visual>>) -> Self {
        Self {
            layout,
            children,
            input_handler,
            focused: None,
        }
    }

    pub fn with_children(mut self, children: Vec<Box<dyn Visual>>) -> Self {
        self.children = children;
        self
    }

    pub fn with_input_handler(mut self, input_handler: I) -> Self {
        self.input_handler = input_handler;
        self
    }

    // TODO: Replace with abstract iterator
    pub fn children(&self) -> &Vec<Box<dyn Visual>> {
        &self.children
    }
}

impl<'a, L, I> VisualInput for TreeVisual<'a, L, I>
where
    L: Layout,
    I: VisualLeafInput,
{
    fn on_paste(&mut self, args: &PasteEventArgs, visual_context: &mut dyn MutableContext) -> bool {
        let mut bubble_handled = false;
        let tunnel_handled = self.input_handler.tunnel_paste(args, visual_context);

        if !tunnel_handled {
            if let Some(ref mut focused) = self.focused {
                bubble_handled = focused.on_paste(args, visual_context);
            }
        }

        if !bubble_handled {
            bubble_handled = self.input_handler.bubble_paste(args, visual_context);
        }

        bubble_handled
    }

    fn on_got_focus(&mut self, visual_context: &mut dyn MutableContext) {
        self.input_handler.on_got_focus(visual_context);

        for child in self.children.iter_mut() {
            child.on_got_focus(visual_context);
        }
    }

    fn on_lost_focus(&mut self, visual_context: &mut dyn MutableContext) {
        self.input_handler.on_lost_focus(visual_context);

        for child in self.children.iter_mut() {
            child.on_lost_focus(visual_context);
        }
    }

    fn on_key_press(
        &mut self,
        args: &KeyEventArgs,
        visual_context: &mut dyn MutableContext,
    ) -> bool {
        let mut bubble_handled = false;
        let tunnel_handled = self.input_handler.tunnel_key_press(args, visual_context);

        if !tunnel_handled {
            if let Some(ref mut focused) = self.focused {
                bubble_handled = focused.on_key_press(args, visual_context);
            }
        }

        if !bubble_handled {
            bubble_handled = self.input_handler.bubble_key_press(args, visual_context);
        }

        bubble_handled
    }

    fn on_key_release(
        &mut self,
        args: &KeyEventArgs,
        visual_context: &mut dyn MutableContext,
    ) -> bool {
        let mut bubble_handled = false;
        let tunnel_handled = self.input_handler.tunnel_key_release(args, visual_context);

        if !tunnel_handled {
            if let Some(ref mut focused) = self.focused {
                bubble_handled = focused.on_key_release(args, visual_context);
            }
        }

        if !bubble_handled {
            bubble_handled = self.input_handler.bubble_key_release(args, visual_context);
        }

        bubble_handled
    }

    fn on_mouse_move(
        &mut self,
        args: &MouseEventArgs,
        visual_context: &mut dyn MutableContext,
    ) -> bool {
        let mut bubble_handled = false;
        let tunnel_handled = self.input_handler.tunnel_mouse_move(args, visual_context);

        if !tunnel_handled {
            if let Some(ref mut focused) = self.focused {
                bubble_handled = focused.on_mouse_move(args, visual_context);
            }
        }

        if !bubble_handled {
            bubble_handled = self.input_handler.bubble_mouse_move(args, visual_context);
        }

        bubble_handled
    }

    fn on_mouse_wheel(
        &mut self,
        args: &MouseWheelEventArgs,
        visual_context: &mut dyn MutableContext,
    ) -> bool {
        let mut bubble_handled = false;
        let tunnel_handled = self.input_handler.tunnel_mouse_wheel(args, visual_context);

        if !tunnel_handled {
            if let Some(ref mut focused) = self.focused {
                bubble_handled = focused.on_mouse_wheel(args, visual_context);
            }
        }

        if !bubble_handled {
            bubble_handled = self.input_handler.bubble_mouse_wheel(args, visual_context);
        }

        bubble_handled
    }

    fn on_mouse_up(
        &mut self,
        args: &MouseButtonEventArgs,
        visual_context: &mut dyn MutableContext,
    ) -> bool {
        let mut bubble_handled = false;
        let tunnel_handled = self.input_handler.tunnel_mouse_up(args, visual_context);

        if !tunnel_handled {
            if let Some(ref mut focused) = self.focused {
                bubble_handled = focused.on_mouse_up(args, visual_context);
            }
        }

        if !bubble_handled {
            bubble_handled = self.input_handler.bubble_mouse_up(args, visual_context);
        }

        bubble_handled
    }

    fn on_mouse_down(
        &mut self,
        args: &MouseButtonEventArgs,
        visual_context: &mut dyn MutableContext,
    ) -> bool {
        let mut bubble_handled = false;
        let tunnel_handled = self.input_handler.tunnel_mouse_down(args, visual_context);

        if !tunnel_handled {
            if let Some(ref mut focused) = self.focused {
                bubble_handled = focused.on_mouse_down(args, visual_context);
            }
        }

        if !bubble_handled {
            bubble_handled = self.input_handler.bubble_mouse_down(args, visual_context);
        }

        bubble_handled
    }
}

impl<'a, L, I> Draw for TreeVisual<'a, L, I>
where
    L: Layout,
    I: VisualLeafInput,
{
    fn draw(&self, buffer: &mut dyn WriteBuffer, available_size: Size) -> Size {
        let arrange = self.layout.draw(&self.children, buffer, available_size);

        arrange.size()
    }

    fn measure(&self, constraints: Size) -> Size {
        self.layout.measure(&self.children, constraints)
    }
}

impl<'a, L, I> Visual for TreeVisual<'a, L, I>
where
    L: Layout,
    I: VisualLeafInput,
{
}
