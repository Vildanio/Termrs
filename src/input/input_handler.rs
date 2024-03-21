#![allow(unused)]

use crate::{
    input::{
        KeyEventArgs, MouseButtonEventArgs, MouseEventArgs, MouseWheelEventArgs, PasteEventArgs,
    },
    visual::MutableContext,
};

pub struct EmptyVisualLeafInput;

impl VisualLeafInput for EmptyVisualLeafInput {}

pub trait VisualLeafInput {
    fn tunnel_paste(
        &mut self,
        args: &PasteEventArgs,
        visual_context: &mut dyn MutableContext,
    ) -> bool {
        false
    }

    fn bubble_paste(
        &mut self,
        args: &PasteEventArgs,
        visual_context: &mut dyn MutableContext,
    ) -> bool {
        false
    }

    // These methods are not routed, because... i don't know.
    fn on_got_focus(&mut self, visual_context: &mut dyn MutableContext) {}
    fn on_lost_focus(&mut self, visual_context: &mut dyn MutableContext) {}

    fn tunnel_key_press(
        &mut self,
        args: &KeyEventArgs,
        visual_context: &mut dyn MutableContext,
    ) -> bool {
        false
    }

    fn tunnel_key_release(
        &mut self,
        args: &KeyEventArgs,
        visual_context: &mut dyn MutableContext,
    ) -> bool {
        false
    }

    fn bubble_key_press(
        &mut self,
        args: &KeyEventArgs,
        visual_context: &mut dyn MutableContext,
    ) -> bool {
        false
    }
    fn bubble_key_release(
        &mut self,
        args: &KeyEventArgs,
        visual_context: &mut dyn MutableContext,
    ) -> bool {
        false
    }

    fn tunnel_mouse_move(
        &mut self,
        args: &MouseEventArgs,
        visual_context: &mut dyn MutableContext,
    ) -> bool {
        false
    }

    fn tunnel_mouse_wheel(
        &mut self,
        args: &MouseWheelEventArgs,
        visual_context: &mut dyn MutableContext,
    ) -> bool {
        false
    }

    fn tunnel_mouse_up(
        &mut self,
        args: &MouseButtonEventArgs,
        visual_context: &mut dyn MutableContext,
    ) -> bool {
        false
    }

    fn tunnel_mouse_down(
        &mut self,
        args: &MouseButtonEventArgs,
        visual_context: &mut dyn MutableContext,
    ) -> bool {
        false
    }

    fn bubble_mouse_move(
        &mut self,
        args: &MouseEventArgs,
        visual_context: &mut dyn MutableContext,
    ) -> bool {
        false
    }
    fn bubble_mouse_wheel(
        &mut self,
        args: &MouseWheelEventArgs,
        visual_context: &mut dyn MutableContext,
    ) -> bool {
        false
    }
    fn bubble_mouse_up(
        &mut self,
        args: &MouseButtonEventArgs,
        visual_context: &mut dyn MutableContext,
    ) -> bool {
        false
    }
    fn bubble_mouse_down(
        &mut self,
        args: &MouseButtonEventArgs,
        visual_context: &mut dyn MutableContext,
    ) -> bool {
        false
    }
}

/// Provides handlers for common events.
pub trait VisualInput {
    fn on_paste(&mut self, args: &PasteEventArgs, visual_context: &mut dyn MutableContext) -> bool {
        false
    }

    // These methods called even for not-focused visuals
    fn on_got_focus(&mut self, visual_context: &mut dyn MutableContext) {}

    fn on_lost_focus(&mut self, visual_context: &mut dyn MutableContext) {}

    fn on_key_press(
        &mut self,
        args: &KeyEventArgs,
        visual_context: &mut dyn MutableContext,
    ) -> bool {
        false
    }

    fn on_key_release(
        &mut self,
        args: &KeyEventArgs,
        visual_context: &mut dyn MutableContext,
    ) -> bool {
        false
    }

    fn on_mouse_move(
        &mut self,
        args: &MouseEventArgs,
        visual_context: &mut dyn MutableContext,
    ) -> bool {
        false
    }

    fn on_mouse_wheel(
        &mut self,
        args: &MouseWheelEventArgs,
        visual_context: &mut dyn MutableContext,
    ) -> bool {
        false
    }

    fn on_mouse_up(
        &mut self,
        args: &MouseButtonEventArgs,
        visual_context: &mut dyn MutableContext,
    ) -> bool {
        false
    }

    fn on_mouse_down(
        &mut self,
        args: &MouseButtonEventArgs,
        visual_context: &mut dyn MutableContext,
    ) -> bool {
        false
    }
}
