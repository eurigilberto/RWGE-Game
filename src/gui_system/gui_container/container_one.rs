use rwge::{
    color::RGBA,
    glam::{vec2, UVec2, Vec2},
    gui::rect_ui::{
        element::builder::ElementBuilder, event::UIEvent, BorderRadius, ExtraBufferData, Rect,
    },
    math_utils::lerp_f32,
    Engine,
};

use crate::{
    gui_system::{control::ControlState, window_layout::GUI_ACTIVE_COLOR, ContainerInfo},
    public_data::{self, utils::get_engine_data, EngineData, PublicData},
};

use super::GUIContainer;

pub struct ContainerOne {
    pub name: String,
    pub value: f32,
    pub color: RGBA,
    pub count: u32,
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
        let container_mask = Rect { position, size };
        match event {
            UIEvent::Render { gui_rects, .. } => {
                ElementBuilder::new(position, size)
                    .set_color(GUI_ACTIVE_COLOR.into())
                    .set_rect_mask(
                        Rect {
                            position: position,
                            size: size,
                        }
                        .into(),
                    )
                    .build(gui_rects);
            }
            _ => {}
        }

        {
            let position = position + size * vec2(-0.5, 0.5);
            // Grid component?
            const GRID_RECT_SIZE: f32 = 30.0;
            const GRID_RECT_PADDING: f32 = 10.0;
            const GRID_MARGIN: f32 = 8.0;

            // Generate rectangle positions
            let containers = Vec::<(Vec2, Vec2)>::with_capacity(self.count as usize);

            let allowed_horizontal_size = size.x - (GRID_MARGIN * 2.0);
            let rect_count_horizontal_fit =
                allowed_horizontal_size / (GRID_RECT_SIZE + GRID_RECT_PADDING);
            let horizontal_rect_count = rect_count_horizontal_fit.floor().max(1.0);
            let required_rect_size = allowed_horizontal_size / horizontal_rect_count; //with padding
            let required_rect_size = required_rect_size.max(GRID_RECT_SIZE + GRID_RECT_PADDING);
            let start_position = position + vec2(GRID_MARGIN, -GRID_MARGIN);

            let horizontal_rect_count = horizontal_rect_count as u32;
            let size_padded = required_rect_size;
            let size_elem = required_rect_size - GRID_RECT_PADDING;
            //let mut rect_position = start_position;
            for i in 0..self.count {
                let h_index = i % horizontal_rect_count;
                let v_index = i / horizontal_rect_count;

                match event {
                    UIEvent::Render { gui_rects, .. } => {
                        let scaler_param = get_engine_data(public_data)
                            .time
                            .sin_time_phase(6.0, (i as f32) / 2.0)
                            * 0.5
                            + 0.5;
                        let size_elem_anim = lerp_f32(size_elem * 0.5, size_elem, scaler_param);

                        let rect_position = start_position
                            + vec2(size_padded * 0.5, -size_padded * 0.5)
                            + vec2(size_padded * h_index as f32, -size_padded * v_index as f32);
                        let rect_size = vec2(size_elem_anim, size_elem_anim);
                        let element_builder = ElementBuilder::new(rect_position, rect_size)
                            .set_color(self.color.into())
                            .set_rect_mask(container_mask.into());
                        if i % 2 == 0 {
                            element_builder.set_rotation(
                                get_engine_data(public_data).time.time * (2.0 + (i % 7) as f32),
                            )
                        } else {
                            element_builder
                        }
                        .build(gui_rects);
                    }
                    _ => {}
                }
            }
        }

        /* ElementBuilder::new(screen_size, vec2(10.0, 10.0), vec2(10.0, 10.0))
        .set_color(RGBA::GREEN.into())
        .build(gui_rects); */
    }
}
