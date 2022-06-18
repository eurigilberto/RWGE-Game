use rwge::{
    color::RGBA,
    gui::rect_ui::{
        element::{create_new_rect_element, ColoringType, MaskType, builder::ElementBuilder},
        event::UIEvent,
        BorderRadius, ExtraBufferData, RectMask,
    },
    Engine, glam::{UVec2, Vec2},
};

use crate::public_data::{PublicData, EngineData, self};

use super::GUIContainer;

pub struct ContainerOne {
    pub name: String,
    pub value: f32,
    pub cound: u32,
}

impl GUIContainer for ContainerOne {
    fn get_name(&self) -> &str {
        self.name.as_str()
    }

    fn handle_event(
        &mut self,
        event: &mut UIEvent,
        public_data_changes: &Option<&mut Vec<Box<dyn FnMut(&mut PublicData) -> ()>>>,
        public_data: &PublicData,
        size: Vec2, position: Vec2
    ) {
        //test
        match event {
            UIEvent::Render {
                gui_rects,
            } => {
                let screen_size = public_data::utils::get_engine_data(public_data).screen_size;
                ElementBuilder::new(screen_size, position, size).set_color(RGBA {
                    r: 0.2,
                    g: self.value,
                    b: 0.75,
                    a: 1.0,
                }.into()).set_rect_mask(RectMask {
                    position: position,
                    size: size,
                }.into()).build(gui_rects);
            }
            _ => {}
        }
    }
}
