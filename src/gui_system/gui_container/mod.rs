pub mod container_one;

use std::any::Any;

use rwge::{glam::{UVec2, Vec2}, gui::rect_ui::event::UIEvent, Engine};

use crate::{as_any::AsAny, public_data::PublicData};

pub trait GUIContainer: AsAny {
    fn get_name(&self) -> &str;
    fn handle_event(
        &mut self,
        event: &mut UIEvent,
        public_data_changes: &Option<&mut Vec<Box<dyn FnMut(&mut PublicData) -> ()>>>,
        public_data: &PublicData,
        size: Vec2,
        position: Vec2
    );
}

impl<T: GUIContainer + 'static> AsAny for T {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
