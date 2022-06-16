use std::any::Any;

use rwge::gui::rect_ui::event::UIEvent;

use crate::{as_any::AsAny, public_data::PublicData};

pub trait GUIContainer:AsAny {
    fn get_name(&self)->&str;
    fn handle_event(&self, event: &mut UIEvent, public_data: &mut PublicData);
}

impl <T:GUIContainer + 'static> AsAny for T{
    fn as_any(&self)->&dyn Any {
        self
    }

    fn as_any_mut(&mut self)-> &mut dyn Any {
        self
    }
}