use rwge::{
    color::*,
    glam::{vec2, Vec2},
    gui::rect_ui::{
        element::{builder::ElementBuilder, LinearGradient, Border},
        event::UIEvent,
        BorderRadius, Rect,
    },
    math_utils::lerp_f32,
    uuid::Uuid,
};

use super::{get_current_control_id, ControlId, ControlState, State};

pub fn inv_lerp(a: f32, b: f32, t: f32) -> f32 {
    (t - a) / (b - a)
}

const SLIDER_CONTROL_PIN_WIDTH: f32 = 22.0;
const SLIDER_CONTROL_PIN_HEIGHT: f32 = 22.0;
const SLIDER_CONTROL_BG_HEIGHT: f32 = 10.0;

pub fn slider(
    rect: Rect,
    mask: Rect,
    value: f32,
    min: f32,
    max: f32,
    active_id: &mut Option<Uuid>,
    event: &mut UIEvent,
    control_state: &mut ControlState,
) -> f32 {
    let mut new_value = value.max(min).min(max);

    let control_id = control_state.get_id();

    let compute_new_value = |mouse_position: Vec2| {
        let rel_pos = mouse_position - rect.position;
        let horizontal_pos_norm =
            (rel_pos.x + rect.size.x * 0.5).max(0.0).min(rect.size.x) / rect.size.x;
        lerp_f32(min, max, horizontal_pos_norm)
    };

    match event {
        UIEvent::MouseButton(mouse_input) => {
            if mouse_input.is_left_pressed() {
                *active_id = control_state.set_active(control_id);
                if active_id.is_some() {
                    new_value = compute_new_value(control_state.last_cursor_position.unwrap());
                }
                //println!("Setting active - status {}", active_id.is_some());
            }

            if mouse_input.is_left_released() {
                *active_id = None;
            }
        }
        UIEvent::MouseMove { corrected, .. } => {
            if active_id.is_some() {
                let state = control_state.get_control_state(ControlId::Active(active_id.unwrap()));
                if let State::Active = state {
                    new_value = compute_new_value(*corrected);
                }
            }
        }
        UIEvent::Update => {
            let control_rect = Rect {
                position: rect.position,
                size: vec2(rect.size.x, SLIDER_CONTROL_PIN_HEIGHT),
            };
            let control_rect = control_rect.combine_rects(&mask);
            
            if active_id.is_some() {
                control_state.hold_active_state(active_id.unwrap());
            } else {
                if let Some(control_rect) = control_rect{
                    control_state.set_hot_with_rect(control_id, &control_rect);
                }
            }
        }
        UIEvent::Render { gui_rects, .. } => {
            let bg_position = rect.position;
            let bg_size = vec2(rect.size.x, SLIDER_CONTROL_BG_HEIGHT);

            let norm_pos = inv_lerp(min, max, value);

            let filled_section = Rect {
                position: vec2(
                    rect.top_left_position().x + rect.size.x * norm_pos * 0.5,
                    rect.position.y,
                ),
                size: vec2(rect.size.x * norm_pos, rect.size.y),
            };

            let unfilled_section = Rect {
                position: vec2(
                    rect.top_left_position().x
                        + rect.size.x * norm_pos
                        + rect.size.x * (1.0 - norm_pos) * 0.5,
                    rect.position.y,
                ),
                size: vec2(rect.size.x * (1.0 - norm_pos), rect.size.y),
            };

            let gradient_start = rect.size.x * norm_pos - rect.size.x * 0.5;
            let lin_grad = LinearGradient {
                colors: [RGBA::WHITE, RGBA::rgb(0.0, 0.25, 0.75)],
                start_position: vec2(gradient_start, 0.0),
                end_position: vec2(-rect.size.x * 0.5, 0.0),
            };

            let slider_pin_size = vec2(SLIDER_CONTROL_PIN_WIDTH, SLIDER_CONTROL_PIN_HEIGHT);
            let slider_pin_position = vec2(gradient_start + rect.position.x, rect.position.y);

            if let Some(filled_w_mask) = filled_section.combine_rects(&mask) {
                ElementBuilder::new(bg_position, bg_size)
                    .set_linear_gradient(lin_grad.into())
                    .set_rect_mask(filled_w_mask.into())
                    .set_round_rect(BorderRadius::ForAll(SLIDER_CONTROL_BG_HEIGHT * 0.5).into())
                    .build(gui_rects);
            }

            if let Some(unfilled_w_mask) = mask.combine_rects(&unfilled_section) {
                let state =
                    control_state.get_control_state(get_current_control_id(control_id, active_id));
                ElementBuilder::new(bg_position, bg_size)
                    .set_color(match state {
                        State::Inactive => RGBA::rrr1(0.15).into(),
                        State::Hovered | State::Active => RGBA::rrr1(0.25).into(),
                    })
                    .set_rect_mask(unfilled_w_mask.into())
                    .set_round_rect(BorderRadius::ForAll(SLIDER_CONTROL_BG_HEIGHT * 0.5).into())
                    .build(gui_rects);
            }

            let state =
                    control_state.get_control_state(get_current_control_id(control_id, active_id));
            let (border_size, border_color) = match state {
                State::Inactive => {(4, RGBA::rrr1(0.5))},
                State::Hovered => {(4,RGBA::rrr1(0.75))},
                State::Active => {(2,RGBA::rrr1(0.75))},
            };
            ElementBuilder::new(slider_pin_position.round(), slider_pin_size)
                .set_circle()
                .set_color(RGBA::BLACK.into())
                .set_border(Some(Border{
                    size: border_size,
                    color: border_color.into(),
                }))
                .set_rect_mask(mask.into())
                .build(gui_rects);
        }
        _ => {}
    }

    new_value
}
