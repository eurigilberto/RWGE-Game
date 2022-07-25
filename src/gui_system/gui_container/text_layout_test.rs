use rwge::{
    color::*,
    font::{
        font_layout::{create_multi_line, create_single_line, FontElement, WordRect},
        font_load_gpu::FontCollection,
    },
    glam::{vec2, Vec2},
    gui::rect_ui::{
        element::{builder::ElementBuilder, push_rect_mask, LinearGradient},
        event::UIEvent,
        BorderRadius, Rect, RectBounds,
    },
    math_utils::lerp_f32,
    uuid::Uuid,
};

use crate::{
    gui_system::{
        control::slider,
        gui_container::text_animation::{TextAnimationData, WordAnimData, WordAnimation},
        ContainerInfo,
    },
    runtime_data::{utils::get_time, PublicData},
};

use super::{render_container_background, GUIContainer};

pub struct TextLayoutTest {
    pub text: String,
    pub last_update_width: f32,
    pub text_height: f32,
    pub first_line_height: f32,
    pub font_elements: Option<Vec<FontElement>>,
    pub word_rects: Option<Vec<WordRect>>,

    //Font Selector
    update_font: bool,
    pub font_index: usize,
    pub font_param: f32,
    pub font_param_corrected: f32,
    pub font_active_id: Option<Uuid>,
    pub hovered_word: Option<WordRect>,

    //Scroll controls
    pub scroll_offset: f32,
    pub mouse_prev_pos: Vec2,
    pub scroll_active_id: Option<Uuid>,
}

impl TextLayoutTest {
    pub fn new() -> Self {
        Self {
            text: include_str!("../../../res/lorem_ipsum.txt").to_owned(),
            last_update_width: 0.0,
            text_height: 0.0,
            font_elements: None,
            word_rects: None,
            first_line_height: 0.0,

            //Font Selector
            update_font: false,
            font_index: 0,
            font_param: 0.0,
            font_param_corrected: 0.0,
            font_active_id: None,
            hovered_word: None,

            //Scroll controls
            scroll_offset: 0.0,
            mouse_prev_pos: Vec2::ZERO,
            scroll_active_id: None,
        }
    }
}

const TOP_MARGIN: f32 = 14.0;
const LEFT_MARGIN: f32 = 16.0;
const RIGHT_MARGIN: f32 = 24.0;
const TEXT_START_OFFSET: f32 = 18.0;
const SCROLL_RIGHT_MARGIN: f32 = 8.0;

const WORD_RECT_PAD_MIN: (f32, f32) = (14.0, 8.0);
const WORD_RECT_PAD_MAX: (f32, f32) = (100.0, 80.0);

fn get_padding(font_param: f32) -> Vec2 {
    vec2(
        lerp_f32(WORD_RECT_PAD_MIN.0, WORD_RECT_PAD_MAX.1, font_param),
        lerp_f32(WORD_RECT_PAD_MIN.1, WORD_RECT_PAD_MAX.1, font_param),
    )
}

const FONT_SIZE_MIN_MAX: (f32, f32) = (14.0, 1024.0);
const LINE_HEIGHT_MIN_MAX: (f32, f32) = (FONT_SIZE_MIN_MAX.0, FONT_SIZE_MIN_MAX.1);
const PARA_SEP_MIN_MAX: (f32, f32) = (LINE_HEIGHT_MIN_MAX.0 + 2.0, LINE_HEIGHT_MIN_MAX.1 + 2.0);

impl GUIContainer for TextLayoutTest {
    fn get_name(&self) -> &str {
        //rwge::glam::Mat4::perspective_lh(fov_y_radians, aspect_ratio, z_near, z_far)
        //rwge::glam::Mat4::perspective_lh(0.4, 0.5, 0.1, 1000.0).inverse();
        "Text Layout"
    }

    fn handle_event(
        &mut self,
        event: &mut rwge::gui::rect_ui::event::UIEvent,
        public_data: &PublicData,
        container_info: crate::gui_system::ContainerInfo,
        control_state: &mut crate::gui_system::control::ControlState,
    ) {
        let scroll_control_id = control_state.get_id();

        if let UIEvent::Render { gui_rects, .. } = event {
            render_container_background(gui_rects, &container_info);
        }

        let cont_rect = container_info
            .rect
            .offset_size(vec2(0.0, -50.0))
            .offset_position(vec2(0.0, -25.0));

        const TOP_RECT_HEIGHT: f32 = 50.0;

        let top_cont_rect = Rect {
            position: container_info.rect.position
                + vec2(
                    0.0,
                    container_info.rect.size.y * 0.5 - TOP_RECT_HEIGHT * 0.5,
                ),
            size: vec2(container_info.rect.size.x, TOP_RECT_HEIGHT),
        };

        const BTN_WIDTH: f32 = 50.0;
        const BTN_HEIGHT: f32 = 20.0;
        const BTN_MARGIN: f32 = 14.0;
        const BTN_GAP: f32 = 2.0;

        let slider_rect = Rect {
            position: top_cont_rect.position
                + vec2(BTN_WIDTH * 3.0 + BTN_MARGIN + BTN_GAP * 2.0, 0.0) * 0.5,
            size: top_cont_rect.size
                - vec2(BTN_WIDTH * 3.0 + BTN_MARGIN + BTN_GAP * 2.0 + 28.0, 0.0),
        };
        self.font_param = slider::slider(
            slider_rect,
            container_info.rect,
            self.font_param,
            0.0,
            1.0,
            &mut self.font_active_id,
            event,
            control_state,
        );
        self.font_param_corrected = f32::powf(self.font_param, 3.5);

        match event {
            UIEvent::MouseMove { .. } | UIEvent::MouseButton(..) => {
                if self.font_active_id.is_some() {
                    self.update_font = true;
                }
            }
            _ => {}
        }

        const SELECTED_BTN: RGBA = RGBA::rgb(0.15, 0.4, 0.8);
        const HOVERED_BTN: RGBA = RGBA::rrr1(0.5);
        const INACTIVE_BTN: RGBA = RGBA::rrr1(0.3);

        let top_cont_rect = top_cont_rect.offset_position(vec2(BTN_MARGIN, -BTN_MARGIN));

        let font_selectors = vec![
            (
                Rect {
                    position: top_cont_rect.top_left_position()
                        + vec2(BTN_WIDTH, -BTN_HEIGHT) * 0.5,
                    size: vec2(BTN_WIDTH, BTN_HEIGHT),
                },
                Some(BorderRadius::ForLeftRight {
                    left: BTN_HEIGHT * 0.5,
                    right: 0.0,
                }),
                control_state.get_id(),
                0.05,
            ),
            (
                Rect {
                    position: top_cont_rect.top_left_position()
                        + vec2(BTN_WIDTH * 1.5, -BTN_HEIGHT * 0.5)
                        + vec2(BTN_GAP, 0.0),
                    size: vec2(BTN_WIDTH, BTN_HEIGHT),
                },
                None,
                control_state.get_id(),
                0.02,
            ),
            (
                Rect {
                    position: top_cont_rect.top_left_position()
                        + vec2(BTN_WIDTH * 2.5, -BTN_HEIGHT * 0.5)
                        + vec2(BTN_GAP * 2.0, 0.0),
                    size: vec2(BTN_WIDTH, BTN_HEIGHT),
                },
                Some(BorderRadius::ForLeftRight {
                    left: 0.0,
                    right: BTN_HEIGHT * 0.5,
                }),
                control_state.get_id(),
                0.1,
            ),
        ];

        {
            // TOP RECT

            if let UIEvent::Update = event {
                for (s_rect, _, control_id, ..) in font_selectors.iter() {
                    let interactable_rect = s_rect.combine_rects(&container_info.rect);
                    if let Some(rect) = interactable_rect {
                        control_state.set_hot_with_rect(*control_id, &rect);
                    }
                }
            }

            if let UIEvent::MouseButton(mouse_input) = event {
                if mouse_input.is_left_pressed() {
                    for (index, (_, _, control_id, ..)) in font_selectors.iter().enumerate() {
                        if control_state.is_hovered(*control_id) {
                            self.font_index = index;
                            self.update_font = true;
                        }
                    }
                }
            }

            if let UIEvent::Render { gui_rects, .. } = event {
                for (index, (s_rect, s_border, control_id, ..)) in font_selectors.iter().enumerate()
                {
                    let mut elem_builder = ElementBuilder::new_with_rect(*s_rect);
                    elem_builder = {
                        if index == self.font_index {
                            elem_builder.set_color(SELECTED_BTN.into())
                        } else if control_state.is_hovered(*control_id) {
                            elem_builder.set_color(HOVERED_BTN.into())
                        } else {
                            elem_builder.set_color(INACTIVE_BTN.into())
                        }
                    };
                    elem_builder = {
                        if let Some(border) = *s_border {
                            elem_builder.set_round_rect(border.into())
                        }else{
                            elem_builder
                        }
                    };
                    elem_builder
                        .set_rect_mask(container_info.rect.into())
                        .build(gui_rects);
                }

                let top_pos = cont_rect.position + vec2(0.0, cont_rect.size.y * 0.5 + 2.0);
                ElementBuilder::new_with_rect(Rect {
                    position: top_pos,
                    size: vec2(cont_rect.size.x, 4.0),
                })
                .set_color(RGBA::rrr1(0.15).into())
                .build(gui_rects);
            }
        }

        {
            // Word interaction
            if let Some(ref w_rects) = self.word_rects {
                let mut control_ids = Vec::new();
                for _ in 0..w_rects.len() {
                    control_ids.push(control_state.get_id());
                }
                if control_state.last_cursor_position.is_some() {
                    if cont_rect.inside_rect(control_state.last_cursor_position.unwrap()) {
                        let text_render_offset = get_text_render_object(
                            cont_rect.height(),
                            self.text_height,
                            self.scroll_offset,
                            self.first_line_height
                        ) + cont_rect.top_left_position();

                        if let UIEvent::Update = event {
                            let mut hovering_word = false;
                            for (rect, control_id) in w_rects.iter().zip(control_ids.iter()) {
                                control_state.set_hot_with_rect(
                                    *control_id,
                                    &(rect.rect.offset_position(text_render_offset)),
                                );
                                if control_state.is_hovered(*control_id) {
                                    self.hovered_word = Some(*rect);
                                    hovering_word = true;
                                }
                            }
                            if !hovering_word {
                                self.hovered_word = None;
                            }
                        }

                        if let UIEvent::MouseButton(mouse_input) = event {
                            if mouse_input.is_left_pressed() && self.hovered_word.is_some() {
                                let w_rect = self.hovered_word.unwrap();
                                public_data.get::<TextAnimationData>()
                                    .unwrap()
                                    .push_anim_data(WordAnimData::new(
                                        w_rect.rect,
                                        self.font_elements.as_ref().unwrap()
                                            [w_rect.index..w_rect.index + w_rect.len]
                                            .to_vec(),
                                        text_render_offset,
                                        get_time(public_data).time,
                                    ))
                            }
                        }
                    } else {
                        self.hovered_word = None;
                    }
                }
                if let UIEvent::Render { gui_rects, .. } = event {
                    let text_render_offset = get_text_render_object(
                        cont_rect.height(),
                        self.text_height,
                        self.scroll_offset,
                        self.first_line_height
                    ) + cont_rect.top_left_position();

                    if let Some(hovered) = self.hovered_word {
                        let w_rect = hovered
                            .rect
                            .offset_position(text_render_offset)
                            .offset_size(get_padding(self.font_param_corrected));
                        ElementBuilder::new_with_rect(w_rect)
                            .set_linear_gradient(
                                LinearGradient {
                                    colors: [RGBA::rgb(0.1, 0.1, 0.6), RGBA::rgb(0.15, 0.4, 0.85)],
                                    start_position: vec2(-w_rect.width() * 0.5, 0.0),
                                    end_position: vec2(w_rect.width() * 0.5, 0.0),
                                }
                                .into(),
                            )
                            .set_round_rect(
                                BorderRadius::ForAll(w_rect.size.min_element() * 0.5).into(),
                            )
                            .set_rect_mask(cont_rect.into())
                            .build(gui_rects);
                    }
                }
            }
        }

        {
            // TEXT LAYOUT RECT
            if let UIEvent::Update = event {
                if f32::abs(self.last_update_width - cont_rect.width()) > 0.5 || self.update_font {
                    let font_collection =
                        &public_data.get::<Vec<FontCollection>>().unwrap()[0];
                    let (font_elems, word_rects, text_height, first_line_height) = create_multi_line(
                        &self.text,
                        lerp_f32(FONT_SIZE_MIN_MAX.0, FONT_SIZE_MIN_MAX.1, self.font_param_corrected),
                        font_collection,
                        self.font_index,
                        font_selectors[self.font_index].3,
                        cont_rect.width() - LEFT_MARGIN - RIGHT_MARGIN,
                        lerp_f32(
                            LINE_HEIGHT_MIN_MAX.0,
                            LINE_HEIGHT_MIN_MAX.1,
                            self.font_param_corrected,
                        ),
                        lerp_f32(PARA_SEP_MIN_MAX.0, PARA_SEP_MIN_MAX.1, self.font_param_corrected),
                    );

                    self.first_line_height = first_line_height;

                    self.font_elements = Some(font_elems);
                    self.word_rects = Some(word_rects);
                    self.text_height = text_height;
                    self.last_update_width = cont_rect.width();

                    self.update_font = false;
                }

                if let Some(active_id) = self.scroll_active_id {
                    if !control_state.hold_active_state(active_id) {
                        self.scroll_active_id = None;
                    }
                } else {
                    let (_, scroll_rect, ..) =
                        scroll_control(cont_rect, self.text_height, self.scroll_offset);
                    control_state.set_hot_with_rect(
                        scroll_control_id,
                        &(scroll_rect.offset_size(vec2(6.0, 6.0))),
                    );
                }
            }

            if let UIEvent::MouseButton(mouse_input) = event {
                if mouse_input.is_left_pressed() {
                    self.scroll_active_id = control_state.set_active(scroll_control_id);
                    if self.scroll_active_id.is_some() {
                        self.mouse_prev_pos = control_state.last_cursor_position.unwrap();
                    }
                }

                if mouse_input.is_left_released() {
                    self.scroll_active_id = None;
                }
            }

            if let UIEvent::MouseMove { corrected, .. } = event {
                if control_state.is_active(self.scroll_active_id) {
                    let move_delta = *corrected - self.mouse_prev_pos;

                    let (_, scroll_rect, height_limit) =
                        scroll_control(cont_rect, self.text_height, self.scroll_offset);

                    let missing_height = height_limit - scroll_rect.height();
                    self.scroll_offset -= move_delta.y / missing_height;
                    self.scroll_offset = self.scroll_offset.max(0.0).min(1.0);
                    self.mouse_prev_pos = *corrected;
                }
            }

            if let UIEvent::Render { gui_rects, .. } = event {
                let (side_bar_rect, scroll_rect, height_limit) =
                    scroll_control(cont_rect, self.text_height, self.scroll_offset);

                let text_render_offset = get_text_render_object(
                    cont_rect.height(),
                    self.text_height,
                    self.scroll_offset,
                    self.first_line_height
                );

                let rect_mask_index = push_rect_mask(cont_rect, gui_rects) as u16;
                if let Some(ref font_elems) = self.font_elements {
                    let text_start_position = cont_rect.top_left_position() + text_render_offset;

                    let mut out_of_bounds_count = 0;
                    let mut removed = false;
                    for (index, elem) in font_elems.iter().enumerate() {
                        let elem_rect = elem.rect.offset_position(text_start_position);

                        if !removed
                            && elem_rect.position.y < cont_rect.position.y
                            && !elem_rect
                                .intersecting_rect(&(cont_rect.offset_size(Vec2::splat(30.0))))
                        {
                            out_of_bounds_count += 1;
                            if out_of_bounds_count > 10 {
                                removed = true;
                                //break;
                            }
                        }

                        let mut lin_grad = None;
                        if let Some(w_rect) = self.hovered_word {
                            if index >= w_rect.index && index < w_rect.index + w_rect.len {
                                lin_grad = Some(LinearGradient {
                                    colors: [RGBA::rgb(0.9, 0.4, 0.2), RGBA::rgb(1.0, 0.6, 0.4)],
                                    start_position: vec2(0.0, -elem.rect.height() * 0.5),
                                    end_position: vec2(0.0, elem.rect.height() * 0.5),
                                });
                            }
                        }
                        let elem_builder = ElementBuilder::new_with_rect(elem_rect)
                            .set_sdffont(elem.tx_slice.into())
                            .set_rect_mask(rect_mask_index.into());
                        {
                            if let Some(lin_grad) = lin_grad {
                                elem_builder.set_linear_gradient(lin_grad.into())
                            } else if removed {
                                elem_builder.set_color(RGBA::RED.into())
                            } else {
                                elem_builder
                            }
                        }
                        .build(gui_rects)
                    }
                }

                ElementBuilder::new_with_rect(side_bar_rect)
                    .set_color(RGBA::rrr1(0.1).into())
                    .set_round_rect(BorderRadius::ForAll(6.0).into())
                    .build(gui_rects);

                let color = if control_state.is_hovered(scroll_control_id) {
                    RGBA::rrr1(0.8)
                } else if control_state.is_active(self.scroll_active_id) {
                    RGBA::rgb(0.2, 0.85, 0.1)
                } else {
                    RGBA::rrr1(0.6)
                };
                ElementBuilder::new_with_rect(scroll_rect)
                    .set_color(color.into())
                    .set_round_rect(BorderRadius::ForAll(4.0).into())
                    .build(gui_rects);
            }
        }
    }
}

fn get_text_render_object(
    container_height: f32,
    text_height: f32,
    scroll_offset: f32,
    first_line: f32,
) -> Vec2 {
    let max_height = container_height - TEXT_START_OFFSET - TOP_MARGIN;
    let missing_height = text_height - max_height;
    let text_vertical_offset = missing_height * scroll_offset + - first_line - TOP_MARGIN;
    vec2(LEFT_MARGIN, text_vertical_offset)
}

pub fn scroll_control(
    container_rect: Rect,
    elem_height: f32,
    scroll_offset: f32,
) -> (Rect, Rect, f32) {
    const SCROLL_RIGHT_MARGIN: f32 = 8.0;
    const TOP_MARGIN: f32 = 14.0;

    let cont_rect = container_rect;
    let side_bar_rect = Rect {
        position: (cont_rect.position + vec2(cont_rect.size.x * 0.5 - SCROLL_RIGHT_MARGIN, 0.0))
            .round(),
        size: vec2(12.0, cont_rect.size.y - 8.0),
    };

    let view_proportion = (cont_rect.size.y - 18.0 - TOP_MARGIN) / elem_height;
    let bar_height_limit = side_bar_rect.height() - 8.0;
    let scroll_heigth = bar_height_limit * (view_proportion.min(1.0));
    let scroll_heigth = scroll_heigth.max(12.0);

    let missing_height = bar_height_limit - scroll_heigth;
    let position_offset = missing_height * scroll_offset;

    let scroll_rect = Rect {
        position: side_bar_rect.position
            + vec2(
                0.0,
                side_bar_rect.height() * 0.5 - 4.0 - scroll_heigth * 0.5 - position_offset,
            ),
        size: vec2(6.0, scroll_heigth),
    };

    (side_bar_rect, scroll_rect, bar_height_limit)
}
