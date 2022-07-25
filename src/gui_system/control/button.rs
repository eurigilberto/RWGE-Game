use rwge::{
    color::*,
    font::{font_layout::create_single_line, font_load_gpu::FontCollection},
    glam::vec2,
    gui::rect_ui::{
        element::{builder::ElementBuilder, push_rect_mask, LinearGradient},
        event::UIEvent,
        BorderRadius, Rect,
    },
};

use super::ControlState;

pub fn button(
    rect: Rect,
    mask: Rect,
    label: &str,
    event: &mut UIEvent,
    control_state: &mut ControlState,
    border_radius: BorderRadius,

    font_size: f32,
    font_collection: &FontCollection,
    collection_index: usize,
    char_spacing: f32,

    inactive_colors: [RGBA; 2],
    hover_colors: [RGBA; 2],
) -> bool {
    let control_id = control_state.get_id();

	if let UIEvent::Update = event {
		let interact_rect = rect.combine_rects(&mask);
		if let Some(interact_rect) = interact_rect {
			control_state.set_hot_with_rect(control_id, &interact_rect);
		}
	}

	if let UIEvent::MouseButton(mouse_input) = event{
		if mouse_input.is_left_pressed() && control_state.is_hovered(control_id) {
			return true;
		}
	}

    if let UIEvent::Render { gui_rects, .. } = event {
        let rect_mask_index = push_rect_mask(mask, gui_rects) as u16;

        ElementBuilder::new_with_rect(rect)
            .set_linear_gradient(
                LinearGradient {
                    colors: if control_state.is_hovered(control_id) {
                        hover_colors
                    } else {
                        inactive_colors
                    },
                    start_position: vec2(-rect.width() * 0.5, 0.0),
                    end_position: vec2(rect.width() * 0.5, 0.0),
                }
                .into(),
            )
            .set_rect_mask(rect_mask_index.into())
            .set_round_rect(border_radius.into())
            .build(gui_rects);

        let (font_elems, font_rect) = create_single_line(
            label,
            font_size,
            font_collection,
            collection_index,
            char_spacing,
        );
        let label_offset = rect.position - font_rect.size * 0.5;
        for elements in font_elems {
            let font_rect = elements.rect.offset_position(label_offset);
            ElementBuilder::new_with_rect(font_rect)
                .set_sdffont(elements.tx_slice.into())
                .set_rect_mask(rect_mask_index.into())
                .build(gui_rects);
        }
    }
    return false;
}
