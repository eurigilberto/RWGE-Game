use std::collections::HashMap;

use rwge::{
    color::RGBA,
    glam::{vec2, UVec2, Vec2},
    gui::rect_ui::{
        element::{builder::ElementBuilder, LinearGradient, RadialGradient, push_radial_gradient}, event::UIEvent, BorderRadius, ExtraBufferData, Rect,
    },
    math_utils::{lerp_f32, lerp_vec2},
    uuid::Uuid,
    Engine,
};

use crate::{
    gui_system::{
        control::{slider, ControlState, State, Uiid},
        window_layout::GUI_ACTIVE_COLOR,
        ContainerInfo,
    },
    public_data::{self, utils::get_engine_data, EngineData, PublicData},
};

use super::{render_container_background, GUIContainer};

struct BoxData {
    pub box_size: f32,
    pub boxes_data: Vec<Vec2>,
}

struct InstanceAnimData {
    pub current_values: BoxData,
    pub target_values: BoxData,
}

pub struct ContainerOne {
    pub name: String,
    pub value: f32,
    pub color: RGBA,
    pub count: u32,

    slider_instance: usize,
    slider_active_id: Option<Uuid>,
    instance_anim_data: HashMap<usize, InstanceAnimData>,
}

impl ContainerOne {
    pub fn new(name: String, value: f32, color: RGBA, count: u32) -> Self {
        Self {
            name,
            value,
            color,
            count,
            slider_instance: 0,
            slider_active_id: None,
            instance_anim_data: HashMap::new(),
        }
    }
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
        instance_index: usize,
    ) {
        const CONTAINER_MARGIN: f32 = 10.0;

        let size = container_info.rect.size;
        match event {
            UIEvent::Render { gui_rects, .. } => {
                render_container_background(gui_rects, &container_info);
            }
            _ => {}
        }

        {
            let mut position = container_info.get_top_left_position();

            {
                //render slider
                const MIN_SLIDER_WIDTH: f32 = 50.0;
                let slider_size = vec2(
                    (container_info.rect.size.x - (CONTAINER_MARGIN * 2.0)).max(MIN_SLIDER_WIDTH),
                    10.0,
                );
                let slider_position = position
                    + vec2(CONTAINER_MARGIN, -CONTAINER_MARGIN)
                    + slider_size * vec2(0.5, -0.5);
                let mut slider_active_id =
                    if self.slider_active_id.is_some() && self.slider_instance == instance_index {
                        self.slider_active_id
                    } else {
                        None
                    };

                if let UIEvent::MouseButton(..) = event {
                    //println!("---- Setting up slider for instance {instance_index}");
                }
                self.value = slider::slider(
                    Rect {
                        position: slider_position,
                        size: slider_size,
                    },
                    container_info.rect,
                    self.value,
                    0.0,
                    200.0,
                    &mut slider_active_id,
                    event,
                    control_state,
                );
                if let UIEvent::MouseButton(..) = event {
                    if self.slider_active_id.is_some() && self.slider_instance == instance_index {
                        if slider_active_id.is_none() {
                            self.slider_active_id = None
                        }
                    }else if self.slider_active_id.is_none(){
                        if slider_active_id.is_some() {
                            self.slider_active_id = slider_active_id;
                            self.slider_instance = instance_index;
                        }
                    }
                    //println!("---- Setting up slider for instance {instance_index} -- active status {}", slider_active_id.is_some());
                }

                

                position -= vec2(0.0, slider_size.y + CONTAINER_MARGIN);
            }

            // Grid component?
            const GRID_RECT_SIZE: f32 = 30.0;
            const GRID_RECT_PADDING: f32 = 10.0;
            const GRID_MARGIN: f32 = CONTAINER_MARGIN;

            let mut controls = vec![Uiid::default(); self.count as usize];

            for control in controls.iter_mut() {
                *control = control_state.get_id();
            }

            if let UIEvent::Update = event {
                if let Some(anim_data) = self.instance_anim_data.get_mut(&instance_index) {
                    //Update animation values
                    // Generate rectangle positions
                    let allowed_horizontal_size = size.x - (GRID_MARGIN * 2.0) - self.value;

                    let horizontal_rect_count = (allowed_horizontal_size
                        / (GRID_RECT_SIZE + GRID_RECT_PADDING))
                        .floor()
                        .max(1.0);
                    let required_rect_size = (allowed_horizontal_size / horizontal_rect_count)
                        .max(GRID_RECT_SIZE + GRID_RECT_PADDING);

                    let v_scaler = ((container_info.rect.size.x - 2.0 * GRID_MARGIN - GRID_RECT_SIZE) / (self.value.max(0.1))).min(1.0);
                    let start_position = position + vec2(GRID_MARGIN + self.value*0.5 * v_scaler, -GRID_MARGIN);

                    let horizontal_rect_count = horizontal_rect_count as u32;
                    let size_padded = required_rect_size;
                    let size_elem = required_rect_size - GRID_RECT_PADDING;

                    anim_data.target_values.box_size = size_elem;
                    for (index, target) in anim_data.target_values.boxes_data.iter_mut().enumerate()
                    {
                        let h_index = index as u32 % horizontal_rect_count;
                        let v_index = index as u32 / horizontal_rect_count;

                        *target = start_position
                            + vec2(size_padded * 0.5, -size_padded * 0.5)
                            + vec2(size_padded * h_index as f32, -size_padded * v_index as f32);
                    }

                    //Update current values
                    anim_data.current_values.box_size = lerp_f32(
                        anim_data.current_values.box_size,
                        anim_data.target_values.box_size,
                        0.1,
                    );
                    for (current, target) in anim_data
                        .current_values
                        .boxes_data
                        .iter_mut()
                        .zip(anim_data.target_values.boxes_data.iter())
                    {
                        *current = Vec2::lerp(*current, *target, 0.1);
                    }
                } else {
                    //Create animation values if there are none for this instance
                    let current = BoxData {
                        box_size: 0.0,
                        boxes_data: vec![position; self.count as usize],
                    };

                    let target = BoxData {
                        box_size: 0.0,
                        boxes_data: vec![position; self.count as usize],
                    };

                    self.instance_anim_data.insert(
                        instance_index,
                        InstanceAnimData {
                            current_values: current,
                            target_values: target,
                        },
                    );
                }
            }

            if let UIEvent::Update = event {
                let ref anim_data = self
                    .instance_anim_data
                    .get(&instance_index)
                    .unwrap()
                    .current_values;
                for (index, position) in anim_data.boxes_data.iter().enumerate() {
                    let control_id = controls[index];

                    let i = index as u32;
                    let scaler_param = get_engine_data(public_data)
                        .time
                        .sin_time_phase(6.0, (i as f32) / 2.0)
                        * 0.5
                        + 0.5;
                    let size_elem_anim =
                        lerp_f32(anim_data.box_size * 0.5, anim_data.box_size, scaler_param);

                    let rect_size = vec2(size_elem_anim, size_elem_anim);
                    let control_rect = Rect {
                        position: *position,
                        size: rect_size,
                    };

                    if let Some(combined_rect) = control_rect.combine_rects(&container_info.rect){
                        control_state.set_hot_with_rect(
                            control_id,
                            &combined_rect,
                        );
                    }
                    
                }
            }

            if let UIEvent::Render {
                gui_rects,
                extra_render_steps,
            } = event
            {
                let ref anim_data = self
                    .instance_anim_data
                    .get(&instance_index)
                    .unwrap()
                    .current_values;
                for (index, position) in anim_data.boxes_data.iter().enumerate() {
                    let control_id = controls[index];

                    let i = index as u32;
                    let scaler_param = get_engine_data(public_data)
                        .time
                        .sin_time_phase(6.0, (i as f32) / 2.0)
                        * 0.5
                        + 0.5;
                    let size_elem_anim =
                        lerp_f32(anim_data.box_size * 0.5, anim_data.box_size, scaler_param);
                    let roundness = if i % 10 == 0 {lerp_f32(1.0, 15.0, scaler_param)} else {5.0};
                    let box_color = if let State::Hovered =
                        control_state.get_control_state(control_id.into())
                    {
                        RGBA::WHITE
                    } else {
                        self.color
                    };

                    let rect_size = vec2(size_elem_anim, size_elem_anim);
                    let mut element_builder = ElementBuilder::new(*position, rect_size)
                        .set_color(box_color.into())
                        .set_rect_mask(container_info.rect.into())
                        .set_round_rect(BorderRadius::ForAll(roundness).into());
                    
                    let linear_gradient = LinearGradient{
                        colors: [box_color, box_color * 0.5],
                        start_position: vec2(0.0, rect_size.y * 0.5),
                        end_position: vec2(0.0, -rect_size.y * 0.5),
                    };

                    let radial_gradient = RadialGradient{
                        colors: [box_color, box_color * 0.5],
                        center_position: Vec2::ZERO,
                        end_radius: rect_size.x * 0.702,
                        start_radius: 0.0,
                    };
                    
                    if i % 2 == 0 {
                        element_builder = element_builder.set_rotation(
                            get_engine_data(public_data).time.time * (2.0 + (i % 7) as f32),
                        );
                    }

                    if i % 4 == 0 {
                        element_builder = element_builder.set_linear_gradient(linear_gradient.into());
                    }
                    if i % 5 == 0 || i % 5 == 4 {
                        element_builder = element_builder.set_radial_gradient(radial_gradient.into());
                    }

                    element_builder.build(gui_rects);
                }
            }
        }
        if let UIEvent::Render { .. } = event {
            //End of frame as far as the container is aware
        }
    }
}
