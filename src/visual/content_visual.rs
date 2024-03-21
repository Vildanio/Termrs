use crate::{
    buffer::WriteBuffer,
    input::{
        KeyEventArgs, MouseButtonEventArgs, MouseEventArgs, MouseWheelEventArgs, PasteEventArgs,
        VisualInput, VisualLeafInput,
    },
    visual::{Draw, MutableContext, Visual},
    Size,
};

pub trait ContentLayout {
    fn draw(&self, child: &dyn Visual, buffer: &mut dyn WriteBuffer, available_size: Size) -> Size;

    fn measure(&self, child: &dyn Visual, constraints: Size) -> Size;
}

/// Visual which can have a single child visual.
pub struct ContentVisual {
    child: Box<dyn Visual>,
    is_child_focused: bool,
    layout: Box<dyn ContentLayout>,
    input_handler: Box<dyn VisualLeafInput>,
}

impl VisualInput for ContentVisual {
    fn on_paste(&mut self, args: &PasteEventArgs, visual_context: &mut dyn MutableContext) -> bool {
        let mut bubble_handled = false;
        let tunnel_handled = self.input_handler.tunnel_paste(args, visual_context);

        if !tunnel_handled {
            if self.is_child_focused {
                bubble_handled = self.child.on_paste(args, visual_context);
            }
        }

        if !bubble_handled {
            bubble_handled = self.input_handler.bubble_paste(args, visual_context);
        }

        bubble_handled
    }

    fn on_got_focus(&mut self, visual_context: &mut dyn MutableContext) {
        self.child.on_got_focus(visual_context)
    }

    fn on_lost_focus(&mut self, visual_context: &mut dyn MutableContext) {
        self.child.on_lost_focus(visual_context)
    }

    fn on_key_press(
        &mut self,
        args: &KeyEventArgs,
        visual_context: &mut dyn MutableContext,
    ) -> bool {
        let mut bubble_handled = false;
        let tunnel_handled = self.input_handler.tunnel_key_press(args, visual_context);

        if !tunnel_handled {
            if self.is_child_focused {
                bubble_handled = self.child.on_key_press(args, visual_context);
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
            if self.is_child_focused {
                bubble_handled = self.child.on_key_release(args, visual_context);
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
            if self.is_child_focused {
                bubble_handled = self.child.on_mouse_move(args, visual_context);
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
            if self.is_child_focused {
                bubble_handled = self.child.on_mouse_wheel(args, visual_context);
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
            if self.is_child_focused {
                bubble_handled = self.child.on_mouse_up(args, visual_context);
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
            if self.is_child_focused {
                bubble_handled = self.child.on_mouse_down(args, visual_context);
            }
        }

        if !bubble_handled {
            bubble_handled = self.input_handler.bubble_mouse_down(args, visual_context);
        }

        bubble_handled
    }
}

impl Draw for ContentVisual {
    fn draw(&self, buffer: &mut dyn WriteBuffer, available_size: Size) -> Size {
        self.layout.draw(&*self.child, buffer, available_size)
    }

    fn measure(&self, constraints: Size) -> Size {
        self.layout.measure(&*self.child, constraints)
    }
}

impl Visual for ContentVisual {}
