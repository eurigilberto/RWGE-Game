use std::{collections::HashMap, string};

use rwge::{
    color::{HSLA, RGBA},
    font::font_layout::create_single_line,
    glam::{vec2, Vec2},
    gui::rect_ui::{
        element::{builder::ElementBuilder, Border, LinearGradient, RadialGradient},
        event::UIEvent,
        BorderRadius, Rect,
    },
    math_utils::{lerp_f32, lerp_vec2},
    uuid::Uuid,
};

use crate::{
    gui_system::{
        control,
        control::{slider, ControlId, ControlState, State, Uiid},
        window_layout::depth_offset,
        ContainerInfo,
    },
    public_data::{
        utils::{get_engine_data, get_font_collections, get_time},
        PublicData,
    },
};

use super::{render_container_background, GUIContainer};

struct BoxData {
    pub box_size: f32,
    pub box_positions: Vec<Vec2>,
    pub box_color: Vec<RGBA>,
}

impl BoxData {
    pub fn new(count: usize, position: Vec2, color: RGBA) -> Self {
        Self {
            box_size: 0.0,
            box_positions: vec![position; count as usize],
            box_color: vec![color; count as usize],
        }
    }
}

struct AnimationData {
    pub select_hover_boxes: Vec<bool>,
    pub current_values: BoxData,
    pub target_values: BoxData,
    pub box_indices: Vec<usize>,
    pub multi_select_active_id: Option<Uuid>,
    pub start_position: Vec2,
    pub end_position: Vec2,
}

impl AnimationData {
    pub fn new(count: usize, position: Vec2, color: RGBA) -> Self {
        Self {
            select_hover_boxes: vec![false; count as usize],
            current_values: BoxData::new(count, position, color),
            target_values: BoxData::new(count, position, color),
            multi_select_active_id: None,
            start_position: Vec2::ZERO,
            end_position: Vec2::ZERO,
            box_indices: (0..count as usize).collect(),
        }
    }
}

pub struct ContainerOne {
    pub name: String,
    pub value: f32,
    pub color: RGBA,
    pub count: usize,

    slider_instance: Uuid,
    slider_active_id: Option<Uuid>,
    anim_data: AnimationData,

    pub instance_id: Uuid,
}

impl ContainerOne {
    pub fn new(name: String, value: f32, color: RGBA, count: usize) -> Self {
        Self {
            name,
            value,
            color,
            count,
            slider_instance: Uuid::nil(),
            slider_active_id: None,
            anim_data: AnimationData::new(count, Vec2::ZERO, color),
            instance_id: Uuid::new_v4(),
        }
    }
}

fn selection_rect(start_pos: Vec2, end_pos: Vec2, container_rect: &Rect) -> Rect {
    let position = lerp_vec2(start_pos, end_pos, vec2(0.5, 0.5));
    let size = Vec2::abs((start_pos - position) * 2.0);

    Rect { position, size }
        .combine_rects(&container_rect.offset_size(-vec2(4.0, 4.0)))
        .unwrap_or(Rect {
            position: Vec2::ZERO,
            size: Vec2::ZERO,
        })
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
        const CONTAINER_MARGIN: f32 = 10.0;

        let container_size = container_info.rect.size;
        let mut position_offset = vec2(0.0, 0.0);
        let mut top_left_position = container_info.top_left_position();

        match event {
            UIEvent::Render { gui_rects, .. } => {
                render_container_background(gui_rects, &container_info);
            }
            _ => {}
        }

        {
            {
                //BACKGROUND SELECTION BOX
                let background_control = control_state.get_id();
                if let UIEvent::Update = event {
                    let data = &mut self.anim_data;
                    if data.multi_select_active_id.is_some() {
                        control_state.hold_active_state(data.multi_select_active_id.unwrap());
                    } else {
                        control_state.set_hot_with_rect(background_control, &container_info.rect);
                    }
                }

                if let UIEvent::MouseButton(mouse_input) = event {
                    if mouse_input.is_left_pressed() {
                        let data = &mut self.anim_data;
                        data.multi_select_active_id = control_state.set_active(background_control);
                        if data.multi_select_active_id.is_some() {
                            data.start_position = control_state.last_cursor_position.unwrap();
                            data.end_position = data.start_position;
                        }
                    }
                    if mouse_input.is_left_released() {
                        let data = &mut self.anim_data;
                        if control_state.is_active(data.multi_select_active_id) {
                            let random_color = HSLA {
                                h: rwge::rand::random::<f32>() * 360.0,
                                s: 0.6,
                                l: 0.5,
                                a: 1.0,
                            }
                            .into();
                            for (box_hover, box_color) in data
                                .select_hover_boxes
                                .iter_mut()
                                .zip(data.target_values.box_color.iter_mut())
                            {
                                if *box_hover {
                                    *box_color = random_color;
                                }
                                *box_hover = false;
                            }

                            data.multi_select_active_id = None;
                        }
                    }
                }

                if let UIEvent::MouseMove { corrected, .. } = event {
                    let data = &mut self.anim_data;
                    if control_state.is_active(data.multi_select_active_id) {
                        let box_size =
                            vec2(data.current_values.box_size, data.current_values.box_size);
                        let select_rect = selection_rect(
                            data.start_position,
                            data.end_position,
                            &container_info.rect,
                        );
                        for (box_pos, box_hover) in data
                            .current_values
                            .box_positions
                            .iter()
                            .zip(data.select_hover_boxes.iter_mut())
                        {
                            let box_rect = Rect {
                                position: *box_pos,
                                size: box_size,
                            };

                            if select_rect.intersecting_rect(&box_rect) {
                                *box_hover = true;
                            } else {
                                *box_hover = false;
                            }
                        }
                        data.end_position = *corrected;
                    }
                }
            }

            {
                //SLIDER AND BUTTON
                const TEXT_WIDTH: f32 = 200.0;
                const TEXT_HEIGHT: f32 = 25.0;

                const SLIDER_WIDTH_MIN: f32 = 100.0;
                const SLIDER_WIDTH_MAX: f32 = 500.0;

                position_offset += vec2(CONTAINER_MARGIN, -CONTAINER_MARGIN);

                //render slider
                if let UIEvent::Render {
                    gui_rects,
                    extra_render_steps,
                } = event
                {
                    let font_collection = &get_font_collections(public_data)[0];

                    let (font_elem, font_rect) =
                        create_single_line("Side Margin", 18.0, font_collection, 0, 0.0);

                    let height_offset = -TEXT_HEIGHT * 0.5 - font_rect.height() * 0.5;

                    for element in font_elem {
                        ElementBuilder::new_with_rect(element.rect.offset_position(
                            top_left_position + position_offset + vec2(0.0, height_offset),
                        ))
                        .set_sdffont(element.tx_slice.into())
                        .set_rect_mask(container_info.rect.into())
                        .build(gui_rects);
                    }

                    let label_width = font_rect.width();

                    let side_margin = (self.value * 100.0).round() / 100.0;
                    let (font_elem, font_rect) = create_single_line(
                        format!("{}", side_margin).as_str(),
                        18.0,
                        font_collection,
                        0,
                        0.0,
                    );

                    let height_offset = -TEXT_HEIGHT * 0.5 - font_rect.height() * 0.5;

                    let pos_offset = top_left_position
                        + position_offset
                        + vec2(label_width + 20.0, height_offset);

                    let bg_rect = font_rect;
                    let bg_rect = bg_rect
                        .offset_size(vec2(20.0, 10.0))
                        .offset_position(pos_offset);

                    ElementBuilder::new_with_rect(bg_rect)
                        .set_color(RGBA::rrr1(0.1).into())
                        .set_round_rect(BorderRadius::ForAll(bg_rect.height() * 0.5).into())
                        .set_rect_mask(container_info.rect.into())
                        .build(gui_rects);

                    for element in font_elem {
                        ElementBuilder::new_with_rect(element.rect.offset_position(pos_offset))
                            .set_sdffont(element.tx_slice.into())
                            .set_rect_mask(container_info.rect.into())
                            .build(gui_rects);
                    }
                }

                position_offset.x += TEXT_WIDTH;

                let available_size = container_size.x - position_offset.x;
                let slider_width =
                    available_size.max(SLIDER_WIDTH_MIN).min(SLIDER_WIDTH_MAX) - CONTAINER_MARGIN;

                let slider_rect = Rect {
                    position: vec2(
                        top_left_position.x + slider_width * 0.5,
                        top_left_position.y - TEXT_HEIGHT * 0.5,
                    ) + position_offset,
                    size: vec2(slider_width, TEXT_HEIGHT),
                };

                let mut slider_active_id = if self.slider_active_id.is_some()
                    && self.slider_instance == self.instance_id
                {
                    self.slider_active_id
                } else {
                    None
                };

                self.value = slider::slider(
                    slider_rect,
                    container_info.rect,
                    self.value,
                    0.0,
                    600.0,
                    &mut slider_active_id,
                    event,
                    control_state,
                );
                if let UIEvent::MouseButton(..) = event {
                    if self.slider_active_id.is_some() && self.slider_instance == self.instance_id {
                        if slider_active_id.is_none() {
                            self.slider_active_id = None
                        }
                    } else if self.slider_active_id.is_none() {
                        if slider_active_id.is_some() {
                            self.slider_active_id = slider_active_id;
                            self.slider_instance = self.instance_id;
                        }
                    }
                }

                top_left_position -= vec2(0.0, TEXT_HEIGHT + CONTAINER_MARGIN);

                const BUTTON_WIDTH: f32 = 150.0;
                const BUTTON_HEIGHT: f32 = 40.0;
                let font_collection = &get_font_collections(public_data)[0];

                let color_1: RGBA = RGBA::rgb(0.5, 0.2, 0.2);
                let c_param = get_time(public_data).sin_time(2.0) * 0.5 + 0.5;
                let color_2: RGBA = RGBA::rgb(lerp_f32(0.8, 1.0, c_param), 0.4, 0.2);

                let b_color_1: RGBA = RGBA::rgb(0.2, 0.2, 0.5);
                let c_param = get_time(public_data).sin_time(2.0) * 0.5 + 0.5;
                let b_color_2: RGBA = RGBA::rgb(0.2, 0.4, lerp_f32(0.8, 1.0, c_param));

                if control::button(
                    Rect {
                        position: (top_left_position
                            + vec2(CONTAINER_MARGIN, -CONTAINER_MARGIN)
                            + vec2(BUTTON_WIDTH, -BUTTON_HEIGHT) * 0.5),
                        size: vec2(BUTTON_WIDTH, BUTTON_HEIGHT),
                    },
                    container_info.rect,
                    "Randomize",
                    event,
                    control_state,
                    BorderRadius::ForAll(BUTTON_HEIGHT * 0.5),
                    20.0,
                    font_collection,
                    0,
                    0.0,
                    [color_1, color_2],
                    [b_color_1, b_color_2],
                ) {
                    let anim_data = &mut self.anim_data;
                    let box_count = anim_data.current_values.box_color.len();
                    for i in 0..box_count {
                        let rand_index = rwge::rand::random::<usize>() % box_count;
                        anim_data.current_values.box_color.swap(i, rand_index);
                        anim_data.current_values.box_positions.swap(i, rand_index);
                        anim_data.target_values.box_color.swap(i, rand_index);
                        anim_data.box_indices.swap(i, rand_index);
                    }
                }

                top_left_position -= vec2(0.0, BUTTON_HEIGHT + CONTAINER_MARGIN);
            }

            // Grid component?
            const GRID_RECT_SIZE: f32 = 30.0;
            const GRID_RECT_PADDING: f32 = 10.0;
            const GRID_MARGIN: f32 = CONTAINER_MARGIN;

            let mut controls = vec![Uiid::default(); self.count as usize];

            for control in controls.iter_mut() {
                *control = control_state.get_id();
            }

            if let UIEvent::MouseButton(mouse_input) = event {
                if mouse_input.is_left_pressed() {
                    for (index, control) in controls.iter().enumerate() {
                        let state = control_state.get_control_state((*control).into());
                        if let State::Hovered = state {
                            self.anim_data.target_values.box_color[index] = RGBA::rgb(
                                rwge::rand::random(),
                                rwge::rand::random(),
                                rwge::rand::random(),
                            );
                        }
                    }
                }
            }

            //Update animation values
            if let UIEvent::Update = event {
                let anim_data = &mut self.anim_data;
                //Update animation values
                // Generate rectangle positions
                let allowed_horizontal_size = container_size.x - (GRID_MARGIN * 2.0) - self.value;

                let horizontal_rect_count = (allowed_horizontal_size
                    / (GRID_RECT_SIZE + GRID_RECT_PADDING))
                    .floor()
                    .max(1.0);
                let required_rect_size = (allowed_horizontal_size / horizontal_rect_count)
                    .max(GRID_RECT_SIZE + GRID_RECT_PADDING);

                let v_scaler = ((container_info.rect.size.x - 2.0 * GRID_MARGIN - GRID_RECT_SIZE)
                    / (self.value.max(0.1)))
                .min(1.0);
                let start_position = top_left_position
                    + vec2(GRID_MARGIN + self.value * 0.5 * v_scaler, -GRID_MARGIN);

                let horizontal_rect_count = horizontal_rect_count as u32;
                let size_padded = required_rect_size;
                let size_elem = required_rect_size - GRID_RECT_PADDING;

                anim_data.target_values.box_size = size_elem;
                for (index, target) in anim_data.target_values.box_positions.iter_mut().enumerate()
                {
                    let h_index = index as u32 % horizontal_rect_count;
                    let v_index = index as u32 / horizontal_rect_count;

                    *target = start_position
                        + vec2(size_padded * 0.5, -size_padded * 0.5)
                        + vec2(size_padded * h_index as f32, -size_padded * v_index as f32);
                }

                //println!("Current delta time {}", (get_time(public_data).delta_time_millis));
                let anim_scaler = (get_time(public_data).delta_time_millis) / 11.0;
                //Update current values
                anim_data.current_values.box_size = lerp_f32(
                    anim_data.current_values.box_size,
                    anim_data.target_values.box_size,
                    0.1 * anim_scaler,
                );
                for (current, target) in anim_data
                    .current_values
                    .box_positions
                    .iter_mut()
                    .zip(anim_data.target_values.box_positions.iter())
                {
                    *current = Vec2::lerp(*current, *target, 0.1 * anim_scaler);
                }

                for (current, target) in anim_data
                    .current_values
                    .box_color
                    .iter_mut()
                    .zip(anim_data.target_values.box_color.iter())
                {
                    *current = current.lerp_rgba(target, 0.05 * anim_scaler);
                }
            }

            if let UIEvent::Update = event {
                let anim_data = &mut self.anim_data.current_values;
                for (index, position) in anim_data.box_positions.iter().enumerate() {
                    let control_id = controls[index];

                    let i = index as u32;
                    let scaler_param = get_engine_data(public_data)
                        .time
                        .sin_time_phase(6.0, (i as f32) / 2.0)
                        * 0.5
                        + 0.5;
                    let size_elem_anim =
                        lerp_f32(anim_data.box_size * 0.5, anim_data.box_size, scaler_param);

                    let rect_size = if control_state.is_hovered(control_id) {
                        vec2(anim_data.box_size, anim_data.box_size)
                    } else {
                        vec2(size_elem_anim, size_elem_anim)
                    };
                    let control_rect = Rect {
                        position: *position,
                        size: rect_size,
                    };

                    if let Some(combined_rect) = control_rect.combine_rects(&container_info.rect) {
                        control_state.set_hot_with_rect(control_id, &combined_rect);
                    }
                }
            }

            if let UIEvent::Render { gui_rects, .. } = event {
                let instance_data = &mut self.anim_data;
                let ref anim_data = instance_data.current_values;
                let box_size = anim_data.box_size;
                for (control_index, (((position, color), select_hover), index)) in anim_data
                    .box_positions
                    .iter()
                    .zip(&anim_data.box_color)
                    .zip(instance_data.select_hover_boxes.iter())
                    .zip(instance_data.box_indices.iter())
                    .enumerate()
                {
                    let index = *index;
                    let control_id = controls[control_index];

                    let i = index as u32;

                    let (rect_size, roundness, box_color) = if control_state.is_hovered(control_id)
                    {
                        (
                            vec2(anim_data.box_size, anim_data.box_size),
                            5.0,
                            *color * 2.0,
                        )
                    } else if *select_hover {
                        let hover_scale_param = get_time(public_data).sin_time(4.0) * 0.5 + 0.5;
                        let hover_size =
                            lerp_f32(box_size * 0.75, box_size * 1.15, hover_scale_param);
                        (vec2(hover_size, hover_size), 5.0, *color)
                    } else {
                        let s_phase = (i as f32) / 2.0;
                        let scaler_param =
                            get_time(public_data).sin_time_phase(6.0, s_phase) * 0.5 + 0.5;
                        let size_elem_anim = lerp_f32(box_size * 0.5, box_size, scaler_param);
                        (vec2(size_elem_anim, size_elem_anim), 5.0, *color)
                    };

                    let box_rect = Rect {
                        position: *position,
                        size: rect_size,
                    };

                    if box_rect.position.y < container_info.rect.position.y {
                        if !box_rect
                            .offset_size(vec2(10.0, 10.0))
                            .intersecting_rect(&container_info.rect)
                        {
                            continue;
                        }
                    }

                    let mut element_builder = ElementBuilder::new_with_rect(box_rect)
                        .set_color(box_color.into())
                        .set_rect_mask(container_info.rect.into())
                        .set_round_rect(BorderRadius::ForAll(roundness).into());

                    element_builder =
                        if i % 2 == 0 && !control_state.is_hovered(control_id) && !select_hover {
                            let rot =
                                get_engine_data(public_data).time.time * (2.0 + (i % 7) as f32);
                            element_builder.set_rotation(rot)
                        } else {
                            element_builder
                        };

                    if i % 4 == 0 {
                        let linear_gradient = LinearGradient {
                            colors: [box_color, box_color * 0.5],
                            start_position: vec2(0.0, rect_size.y * 0.5),
                            end_position: vec2(0.0, -rect_size.y * 0.5),
                        };
                        element_builder.set_linear_gradient(linear_gradient.into())
                    } else if i % 5 == 0 || i % 5 == 4 {
                        let radial_gradient = RadialGradient {
                            colors: [box_color, box_color * 0.5],
                            center_position: Vec2::ZERO,
                            end_radius: rect_size.x * 0.702,
                            start_radius: 0.0,
                        };
                        element_builder.set_radial_gradient(radial_gradient.into())
                    } else {
                        element_builder
                    }
                    .build(gui_rects);
                }
            }

            if let UIEvent::Render {
                gui_rects,
                extra_render_steps,
            } = event
            {
                let data = &mut self.anim_data;
                if data.multi_select_active_id.is_some() {
                    let state = control_state.get_control_state(ControlId::Active(
                        data.multi_select_active_id.unwrap(),
                    ));
                    if let State::Active = state {
                        let select_rect = selection_rect(
                            data.start_position,
                            data.end_position,
                            &container_info.rect,
                        );

                        ElementBuilder::new_with_rect(select_rect)
                            .set_color(RGBA::GREEN.set_alpha(0.25).into())
                            .set_border(Some(Border {
                                size: 2,
                                color: RGBA::GREEN.set_alpha(0.5).into(),
                            }))
                            .set_rect_mask(container_info.rect.into())
                            .build(gui_rects);

                        let hover_count =
                            data.select_hover_boxes.iter().fold(0, |acc, hover| {
                                if *hover {
                                    acc + 1
                                } else {
                                    acc
                                }
                            });

                        let font_collection = &get_font_collections(public_data)[0];
                        let text = format!("count {hover_count}");
                        let (font_elems, font_rect) =
                            create_single_line(text.as_str(), 16.0, font_collection, 0, 0.0);
                        const TEXT_PADDING_H: f32 = 12.0;
                        const TEXT_PADDING_V: f32 = 6.0;

                        let mut label_box_elements = Vec::new();
                        label_box_elements.push(
                            ElementBuilder::new_with_rect(Rect {
                                position: select_rect.position,
                                size: font_rect.size
                                    + vec2(TEXT_PADDING_H * 2.0, TEXT_PADDING_V * 2.0),
                            })
                            .set_color(RGBA::rrr1(0.15).into())
                            .set_round_rect(
                                BorderRadius::ForAll(TEXT_PADDING_V + font_rect.size.y * 0.5)
                                    .into(),
                            )
                            .set_border(Some(Border {
                                size: 2,
                                color: RGBA::WHITE.into(),
                            })),
                        );

                        for elem in font_elems {
                            label_box_elements.push(
                                ElementBuilder::new_with_rect(elem.rect.offset_position(
                                    select_rect.position - font_rect.size * 0.5,
                                ))
                                .set_sdffont(elem.tx_slice.into())
                            );
                        }

                        extra_render_steps.push(
                            Box::new(move |gui_rects| {
                                for elem in label_box_elements{
                                    elem.build(gui_rects);
                                }
                            }),
                            container_info.depth_range.0 + depth_offset::SELECT_COUNT,
                        )
                    }
                }
            }
        }
        if let UIEvent::Render { .. } = event {
            //End of frame as far as the container is aware
        }
    }
}
