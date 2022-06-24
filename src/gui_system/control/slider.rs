use rwge::{
    gui::rect_ui::{event::UIEvent, Rect, element::{builder::ElementBuilder, LinearGradient}},
    uuid::Uuid,
};

use super::ControlState;

pub fn slider(
    rect: Rect,
    mask: Rect,
    value: f32,
    min: f32,
    max: f32,
    active_id: Option<Uuid>,
    event: &mut UIEvent,
    control_state: &mut ControlState,
) -> f32 {

	let new_value = value.max(min).min(max);

    /*match event {
        UIEvent::Update => {},
        UIEvent::Render {
            gui_rects,
            extra_render_steps,
        } => {
			let bg_position = rect.position;
			let bg_size = rect.size;

			let lin_grad = LinearGradient{
				colors: [RGBA::],
				start_position: todo!(),
				end_position: todo!(),
			}

			ElementBuilder::new(bg_position, bg_size).set_linear_gradient(gradient)
		},
        _ => {},
    }*/

    new_value
}
