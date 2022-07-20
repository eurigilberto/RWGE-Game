use rwge::{
    color::RGBA,
    glam::{ivec2, vec2, Vec2},
    gui::rect_ui::{element::builder::ElementBuilder, event::UIEvent, Rect},
    uuid::Uuid,
    winit::dpi::PhysicalPosition,
};

use crate::runtime_data::{utils::get_window, PublicData, RuntimeData};

use super::{drag_element::DragElement, ControlId, ControlState, State};

pub fn main_window_top_bar(
    position: Vec2,
    size: Vec2,
    event: &mut UIEvent,
    public_data: &PublicData,
    control_state: &mut ControlState,
    active_id: &mut Option<Uuid>,
    drag_element: &mut DragElement,
) {
    let control_id = control_state.get_id();

    let get_control_id = || {
        if active_id.is_some() {
            ControlId::Active(active_id.unwrap())
        } else {
            ControlId::Control(control_id)
        }
    };

    match event {
        UIEvent::Render { gui_rects, .. } => {
            let current_control_id = get_control_id();
            let state = control_state.get_control_state(current_control_id);
            let color: RGBA = if let State::Active = state {
                RGBA::GREEN
            } else if let State::Hovered = state {
                RGBA::rrr1(0.45)
            } else {
                RGBA::rrr1(0.25)
            };

            ElementBuilder::new(position, size)
                .set_color(color.into())
                .build(gui_rects);
        }
        UIEvent::Update => {
            control_state.set_hot_with_rect(control_id, &Rect { position, size });
            match control_state.get_control_state(get_control_id()) {
                State::Active => {
                    let window_pos = drag_element.compute_element_position();
                    public_data.push_mut(Box::new(move |public_data| {
                        let window = public_data
                            .get_mut::<rwge::winit::window::Window>()
                            .unwrap();

                        let new_position = PhysicalPosition::new(window_pos.x, window_pos.y);

                        window.set_outer_position(new_position);
                    }));
                    control_state.hold_active_state(active_id.unwrap());
                }
                _ => { /* No Op */ }
            }
        }
        UIEvent::MouseMove { raw, .. } => {
            control_state.set_hot_with_rect(control_id, &Rect { position, size });

            let outer_pos = get_window(public_data).outer_position().unwrap();
            let outer_pos = (ivec2(outer_pos.x, outer_pos.y)).as_vec2();
            drag_element.update_position(outer_pos + *raw);
        }
        UIEvent::MouseButton(input) => match (input.button, input.state) {
            (rwge::winit::event::MouseButton::Left, rwge::winit::event::ElementState::Pressed) => {
                *active_id = control_state.set_active(control_id);
                if let Some(_) = active_id {
                    let outer_pos = public_data
                        .get::<rwge::winit::window::Window>()
                        .unwrap()
                        .outer_position()
                        .unwrap();

                    drag_element.start_dragging(vec2(outer_pos.x as f32, outer_pos.y as f32));
                }
            }
            (rwge::winit::event::MouseButton::Left, rwge::winit::event::ElementState::Released) => {
                if let State::Active = control_state.get_control_state(get_control_id()) {
                    match control_state.remove_active(active_id.unwrap()) {
                        Ok(_) => {
                            drag_element.stop_dragging();
                            *active_id = None;
                        }
                        Err(_) => {
                            *active_id = None;
                        }
                    }
                }
            }
            _ => {}
        },
        _ => {}
    }
}
