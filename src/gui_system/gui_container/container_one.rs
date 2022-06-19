use rwge::{
    color::RGBA,
    gui::rect_ui::{
        element::{builder::ElementBuilder},
        event::UIEvent,
        BorderRadius, ExtraBufferData, RectMask,
    },
    Engine, glam::{UVec2, Vec2},
};

use crate::{public_data::{PublicData, EngineData, self}, gui_system::ContainerInfo};

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
        public_data: &PublicData,
        container_info: ContainerInfo
    ) {
        //test
        let position = container_info.position;
        let size = container_info.size;
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

                // Grid component?



                /* ElementBuilder::new(screen_size, vec2(10.0, 10.0), vec2(10.0, 10.0))
        .set_color(RGBA::GREEN.into())
        .build(gui_rects); */

            }
            _ => {}
        }
    }
}
