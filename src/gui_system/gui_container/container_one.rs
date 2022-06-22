use rwge::{
    color::RGBA,
    glam::{UVec2, Vec2},
    gui::rect_ui::{
        element::builder::ElementBuilder, event::UIEvent, BorderRadius, ExtraBufferData, RectMask,
    },
    Engine,
};

use crate::{
    gui_system::{control::ControlState, ContainerInfo},
    public_data::{self, EngineData, PublicData},
};

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
        container_info: ContainerInfo,
        control_state: &mut ControlState,
    ) {
        //test
        let position = container_info.position;
        let size = container_info.size;
        match event {
            UIEvent::Render { gui_rects, .. } => {
                ElementBuilder::new( position, size)
                    .set_color(RGBA::rrr1(0.55).into())
                    .set_rect_mask(
                        RectMask {
                            position: position,
                            size: size,
                        }
                        .into(),
                    )
                    .build(gui_rects);

                // Grid component?

                /* ElementBuilder::new(screen_size, vec2(10.0, 10.0), vec2(10.0, 10.0))
                .set_color(RGBA::GREEN.into())
                .build(gui_rects); */
            }
            _ => {}
        }
    }
}
