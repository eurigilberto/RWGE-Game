use rwge::{
    color::RGBA,
    gui::rect_ui::{
        element::{create_new_rect_element, ColoringType, MaskType},
        event::UIEvent,
        BorderRadius, ExtraBufferData, RectMask,
    },
    Engine, glam::UVec2,
};

use crate::public_data::{PublicData, EngineData};

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
        size: UVec2, position: UVec2
    ) {
        //test
        match event {
            UIEvent::Render {
                gui_rects,
            } => {
                let mask = MaskType::Rect { border: None };
                let color = ColoringType::Color(ExtraBufferData::NewData(RGBA {
                    r: 0.2,
                    g: self.value,
                    b: 0.75,
                    a: 1.0,
                }));
                let rect_mask = ExtraBufferData::NewData(RectMask {
                    position: position,
                    size: size,
                });
                let element = create_new_rect_element(
                    gui_rects,
                    public_data.collection.get::<EngineData>().unwrap().screen_size,
                    position,
                    size,
                    0.0,
					rect_mask,
					&mask,
					&color
                );
            }
            _ => {}
        }
    }
}
