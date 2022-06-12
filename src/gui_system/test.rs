use rwge::{gui::rect_ui::{system::GUIRects, graphic::RectGraphic}, wgpu, render_system::RenderSystem, entity_component::PublicDataCollection, font::font_atlas::FontAtlas, glam::{UVec2, uvec2}};

use crate::{DataTypeKey, DataType};

pub fn render_char(
	gui_rects: &mut GUIRects,
	font_atlas_collection: &Vec<FontAtlas>,
	collection_index: usize,
	component_index: u32,
	glyph_char: char,
	position: UVec2,
	rotation: u32,
) {
	let e_char = font_atlas_collection[collection_index]
		.font_glyphs
		.iter()
		.find(|elem| elem.character == glyph_char)
		.expect(format!("Glyph {} not found", glyph_char).as_str());

	gui_rects
		.rect_collection
		.color
		.cpu_vector
		.push([0.5, 0.5, 0.2, 1.0]);

	let char_size = e_char.get_padded_size();
	let packed_char_size = (char_size.x & 0x0000ffff) << 16 | (char_size.y & 0x0000ffff);

	//There needs to be a place in the font atlas where it specifies the texture slice
	let packed_texture_selection = (0) << 4 | component_index;

	let tx_pos_index = gui_rects.rect_collection.texture_position.cpu_vector.len();
	gui_rects.rect_collection.texture_position.cpu_vector.push([
		e_char.tex_coord.x,
		e_char.tex_coord.y,
		packed_char_size,
		packed_texture_selection,
	]);

	let texture_mask_val: u32 = 0;
	let _type: u32 = 1;

	let dv13 = texture_mask_val << 8 | _type;

	let test_rect = RectGraphic {
		position_size: [
			position.x,
			position.y,
			((char_size.x as f32) * 2.0) as u32,
			((char_size.y as f32) * 2.0) as u32,
		],
		data_vector_0: [0, tx_pos_index as u32 + 1, 0, 2],
		data_vector_1: [0, rotation, 0, dv13],
	};
	gui_rects
		.rect_collection
		.rect_graphic
		.cpu_vector
		.push(test_rect);
}

pub fn test_screen(system_time: &rwge::engine_time::EngineTime,
	gui_rects: &mut GUIRects,
	font_atlas_collection: &Vec<FontAtlas>) {
    let texture_mask_val: u32 = 3;
    let element_type: u32 = 0;

    let dv13 = texture_mask_val << 8 | element_type;

    let rotation = (system_time.time_data.time * 8190.0) as u32;

    let test_rect = RectGraphic {
        position_size: [10, 10, 10, 10],
        data_vector_0: [0, 0, 0, 1],
        data_vector_1: [0, 0, 0, dv13],
    };
    gui_rects
        .rect_collection
        .rect_graphic
        .cpu_vector
        .push(test_rect);

    let test_rect = RectGraphic {
        position_size: [100, 100, 70, 70],
        data_vector_0: [0, 0, 1, 1],
        data_vector_1: [0, rotation, 2, dv13],
    };
    gui_rects
        .rect_collection
        .rect_graphic
        .cpu_vector
        .push(test_rect);

    let c_dv13 = texture_mask_val << 8 | 2;
    let test_rect = RectGraphic {
        position_size: [100, 300, 70, 70],
        data_vector_0: [0, 0, 1, 1],
        data_vector_1: [0, 0, 2, c_dv13],
    };
    gui_rects
        .rect_collection
        .rect_graphic
        .cpu_vector
        .push(test_rect);

    let test_rect = RectGraphic {
        position_size: [200, 300, 100, 70],
        data_vector_0: [0, 0, 1, 1],
        data_vector_1: [0, 0, 2, c_dv13],
    };
    gui_rects
        .rect_collection
        .rect_graphic
        .cpu_vector
        .push(test_rect);

    let size_interp = f32::sin(system_time.time_data.time) * 0.5 + 0.5;
    let rect_width = (140.0 - 70.0) * size_interp + 70.0;
    let rect_height = (70.0 - 140.0) * size_interp + 140.0;
    let test_rect = RectGraphic {
        position_size: [200, 500, rect_width as u32, rect_height as u32],
        data_vector_0: [0, 0, 1, 1],
        data_vector_1: [0, 0, 2, c_dv13],
    };
    gui_rects
        .rect_collection
        .rect_graphic
        .cpu_vector
        .push(test_rect);

    let test_rect = RectGraphic {
        position_size: [400, 500, rect_width as u32, rect_height as u32],
        data_vector_0: [0, 0, 1, 1],
        data_vector_1: [0, rotation * 4, 2, c_dv13],
    };
    gui_rects
        .rect_collection
        .rect_graphic
        .cpu_vector
        .push(test_rect);

    let test_rect = RectGraphic {
        position_size: [250, 100, 70, 70],
        data_vector_0: [0, 0, 1, 1],
        data_vector_1: [0, rotation * 4, 1, dv13],
    };
    gui_rects
        .rect_collection
        .rect_graphic
        .cpu_vector
        .push(test_rect);

    gui_rects
        .rect_collection
        .color
        .cpu_vector
        .push([0.5, 0.5, 0.5, 1.0]);
    gui_rects
        .rect_collection
        .rect_mask
        .cpu_vector
        .push([10.0, 20.0, 30.0, 40.0]);
    gui_rects
        .rect_collection
        .texture_position
        .cpu_vector
        .push([1, 2, 3, 4]);
    gui_rects
        .rect_collection
        .border_radius
        .cpu_vector
        .push([11.0, 11.0, 0.0, 11.0]);

    // FONT RENDER TESTING
    render_char(
        gui_rects,
        font_atlas_collection,
        0,
        0,
        'E',
        uvec2(450, 100),
        0,
    );
    render_char(
        gui_rects,
        font_atlas_collection,
        0,
        0,
        'U',
        uvec2(510, 100),
        rotation,
    );
    render_char(
        gui_rects,
        font_atlas_collection,
        0,
        0,
        'R',
        uvec2(570, 100),
        0,
    );
    render_char(
        gui_rects,
        font_atlas_collection,
        0,
        0,
        'I',
        uvec2(620, 100),
        0,
    );

    render_char(
        gui_rects,
        font_atlas_collection,
        1,
        1,
        'E',
        uvec2(450, 200),
        rotation,
    );
    render_char(
        gui_rects,
        font_atlas_collection,
        1,
        1,
        'U',
        uvec2(510, 200),
        rotation,
    );
    render_char(
        gui_rects,
        font_atlas_collection,
        1,
        1,
        'R',
        uvec2(570, 200),
        0,
    );
    render_char(
        gui_rects,
        font_atlas_collection,
        1,
        1,
        'I',
        uvec2(620, 200),
        0,
    );

    render_char(
        gui_rects,
        font_atlas_collection,
        2,
        2,
        'E',
        uvec2(450, 300),
        0,
    );
    render_char(
        gui_rects,
        font_atlas_collection,
        2,
        2,
        'U',
        uvec2(490, 300),
        rotation,
    );
    render_char(
        gui_rects,
        font_atlas_collection,
        2,
        2,
        'R',
        uvec2(530, 300),
        0,
    );
    render_char(
        gui_rects,
        font_atlas_collection,
        2,
        2,
        'I',
        uvec2(560, 300),
        0,
    );
}
