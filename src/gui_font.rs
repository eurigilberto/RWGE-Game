use std::num::NonZeroU32;

use rwge::{half, Engine, gui::rect_ui::system::GUIRects, font::font_atlas::{FontAtlas, FontCharLimit}, glam::uvec2};

pub fn write_font_to_gpu(engine: &Engine, gui_rects: &GUIRects) -> Vec<FontAtlas> {
    let font_data_1 = include_bytes!("../res/fonts/NeoSans/NeoSansStd-MediumItalic.otf");
    let font_data_2 = include_bytes!("../res/fonts/Lato/Lato-Bold.ttf");
    let font_data_3 = include_bytes!("../res/fonts/Lobster-Regular.ttf");

    let font_atlas_1 = FontAtlas::new(
        font_data_1,
        uvec2(1024, 1024),
        48.0,
        FontCharLimit::All,
    )
    .expect("Font Atlas could not be created");
    let font_atlas_2 = FontAtlas::new(
        font_data_2,
        uvec2(1024, 1024),
        48.0,
        FontCharLimit::All,
    )
    .expect("Font Atlas could not be created");
    let font_atlas_3 = FontAtlas::new(
        font_data_3,
        uvec2(1024, 1024),
        22.0,
        FontCharLimit::All,
    )
    .expect("Font Atlas could not be created");
    

    let fonts_texture_size = font_atlas_1.font_sdf_texture.len() * 4;
    let atlas_lenght = font_atlas_1.font_sdf_texture.len();

    let font_data_slice_1 = half::slice::HalfFloatSliceExt::reinterpret_cast(
        font_atlas_1.font_sdf_texture.as_slice(),
    );
    let font_data_slice_2 = half::slice::HalfFloatSliceExt::reinterpret_cast(
        font_atlas_2.font_sdf_texture.as_slice(),
    );
    let font_data_slice_3 = half::slice::HalfFloatSliceExt::reinterpret_cast(
        font_atlas_3.font_sdf_texture.as_slice(),
    );

    let mut fonts_texture = vec![0 as u16; fonts_texture_size];

    for index in 0..atlas_lenght {
        let pixel_index = index * 4 as usize;
        fonts_texture[pixel_index] = font_data_slice_1[index];
        fonts_texture[pixel_index + 1] = font_data_slice_2[index];
        fonts_texture[pixel_index + 2] = font_data_slice_3[index];
    }

    let tx_block_size = (rwge::wgpu::TextureFormat::Rgba16Float)
        .describe()
        .block_size;
    let bytes_per_row = tx_block_size as u32 * 1024;

    engine.render_system.render_window.queue.write_texture(
        gui_rects.texture_atlas.texture.as_image_copy(),
        rwge::bytemuck::cast_slice(fonts_texture.as_slice()),
        rwge::wgpu::ImageDataLayout {
            offset: 0,
            bytes_per_row: NonZeroU32::new(bytes_per_row),
            rows_per_image: NonZeroU32::new(1024),
        },
        rwge::wgpu::Extent3d {
            width: 1024,
            height: 1024,
            depth_or_array_layers: 1,
        },
    );

    vec![font_atlas_1, font_atlas_2, font_atlas_3]
}