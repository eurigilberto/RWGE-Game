use rwge::{
    color::RGBA,
    font::font_atlas::FontAtlas,
    glam::{uvec2, UVec2, vec2},
    gui::rect_ui::{
        element::{create_new_rect_element, ColoringType, MaskType, RadialGradient, TextureSlice, LinearGradient},
        graphic::RectGraphic,
        system::{BorderRadius, ExtraBufferData, GUIRects, RectMask},
    },
    render_system::RenderSystem,
    wgpu,
};

pub fn test_screen(
    system_time: &rwge::engine_time::EngineTime,
    gui_rects: &mut GUIRects,
    font_atlas_collection: &Vec<FontAtlas>,
    screen_size: UVec2,
) {
    let mask = MaskType::Rect { border: None };
    let color = ColoringType::Color(ExtraBufferData::NewData(RGBA::GREEN));
    create_new_rect_element(
        gui_rects,
        screen_size,
        uvec2(10, 10),
        uvec2(10, 10),
        0.0,
        ExtraBufferData::NewData(RectMask {
            position: (screen_size.as_vec2() * 0.5).as_uvec2(),
            size: screen_size,
        }),
        &mask,
        &color,
    );

    let mask = MaskType::Rect { border: None };
    let color = ColoringType::Color(ExtraBufferData::NewData(RGBA::RED));
    create_new_rect_element(
        gui_rects,
        screen_size,
        uvec2(100, 100),
        uvec2(50, 50),
        0.0,
        ExtraBufferData::NewData(RectMask {
            position: (screen_size.as_vec2() * 0.5).as_uvec2(),
            size: screen_size,
        }),
        &mask,
        &color,
    );

    let mask = MaskType::RoundRect {
        border_radius: ExtraBufferData::NewData(BorderRadius {
            top_right: 5.0,
            bottom_right: 10.0,
            top_left: 15.0,
            bottom_left: 20.0,
        }),
        border: None,
    };
    let color = ColoringType::Color(ExtraBufferData::NewData(RGBA::BLUE));
    create_new_rect_element(
        gui_rects,
        screen_size,
        uvec2(100, 200),
        uvec2(50, 50),
        0.0,
        ExtraBufferData::NewData(RectMask {
            position: (screen_size.as_vec2() * 0.5).as_uvec2(),
            size: screen_size,
        }),
        &mask,
        &color,
    );

    let mask = MaskType::RoundRect {
        border_radius: ExtraBufferData::NewData(BorderRadius {
            top_right: 5.0,
            bottom_right: 10.0,
            top_left: 15.0,
            bottom_left: 20.0,
        }),
        border: None,
    };
    let color = ColoringType::Color(ExtraBufferData::NewData(RGBA::BLUE));
    create_new_rect_element(
        gui_rects,
        screen_size,
        uvec2(100, 300),
        uvec2(70, 70),
        system_time.time_data.time,
        ExtraBufferData::NewData(RectMask {
            position: (screen_size.as_vec2() * 0.5).as_uvec2(),
            size: screen_size,
        }),
        &mask,
        &color,
    );

    let mask = MaskType::Circle { border: None };
    let color = ColoringType::Color(ExtraBufferData::NewData(RGBA::GREY));
    create_new_rect_element(
        gui_rects,
        screen_size,
        uvec2(100, 400),
        uvec2(70, 70),
        0.0,
        ExtraBufferData::NewData(RectMask {
            position: (screen_size.as_vec2() * 0.5).as_uvec2(),
            size: screen_size,
        }),
        &mask,
        &color,
    );

    let mask = MaskType::Circle { border: None };
    let color = ColoringType::Color(ExtraBufferData::NewData(RGBA::GREY));
    let size_interp = ((f32::sin(system_time.time_data.time) * 50.0) as i32 + 100) as u32;
    create_new_rect_element(
        gui_rects,
        screen_size,
        uvec2(100, 500),
        uvec2(size_interp, 70),
        0.0,
        ExtraBufferData::NewData(RectMask {
            position: (screen_size.as_vec2() * 0.5).as_uvec2(),
            size: screen_size,
        }),
        &mask,
        &color,
    );

    let mask = MaskType::Circle { border: None };
    let color = ColoringType::Color(ExtraBufferData::NewData(RGBA::RED));
    let size_interp = ((f32::sin(system_time.time_data.time) * 50.0) as i32 + 100) as u32;
    create_new_rect_element(
        gui_rects,
        screen_size,
        uvec2(240, 100),
        uvec2(size_interp, 70),
        system_time.time_data.time * 2.0,
        ExtraBufferData::NewData(RectMask {
            position: (screen_size.as_vec2() * 0.5).as_uvec2(),
            size: screen_size,
        }),
        &mask,
        &color,
    );

    {
        let glyph_char = 'T';
        let e_char = font_atlas_collection[1]
            .font_glyphs
            .iter()
            .find(|elem| elem.character == glyph_char)
            .expect(format!("Glyph {} not found", glyph_char).as_str());
        let mask = MaskType::SDFFont(ExtraBufferData::NewData(TextureSlice {
            sample_component: 1,
            slice_position: e_char.tex_coord,
            size: e_char.get_padded_size(),
            array_index: 0,
        }));
        let color = ColoringType::Color(ExtraBufferData::NewData(RGBA::GREEN));
        create_new_rect_element(
            gui_rects,
            screen_size,
            uvec2(200, 200),
            e_char.get_padded_size() * 2,
            system_time.time_data.time * 2.0,
            ExtraBufferData::NewData(RectMask {
                position: (screen_size.as_vec2() * 0.5).as_uvec2(),
                size: screen_size,
            }),
            &mask,
            &color,
        );
    }
    {
        let glyph_char = 'S';
        let e_char_a = font_atlas_collection[1]
            .font_glyphs
            .iter()
            .find(|elem| elem.character == glyph_char)
            .expect(format!("Glyph {} not found", glyph_char).as_str());
        let maska = MaskType::SDFFont(ExtraBufferData::NewData(TextureSlice {
            sample_component: 1,
            slice_position: e_char_a.tex_coord,
            size: e_char_a.get_padded_size(),
            array_index: 0,
        }));
        let color = ColoringType::Color(ExtraBufferData::NewData(RGBA::GREEN));
        create_new_rect_element(
            gui_rects,
            screen_size,
            uvec2(200, 300),
            e_char_a.get_padded_size(),
            system_time.time_data.time * 2.0,
            ExtraBufferData::NewData(RectMask {
                position: (screen_size.as_vec2() * 0.5).as_uvec2(),
                size: screen_size,
            }),
            &maska,
            &color,
        );
    }

    let mask = MaskType::Rect { border: None };
    let color = ColoringType::RadialGradient(ExtraBufferData::NewData(RadialGradient {
        colors: [RGBA::GREY, RGBA::BLUE],
        center_position: vec2(0.0, 0.0),
        end_radius: 30.0,
        start_radius: 0.0,
    }));
    create_new_rect_element(
        gui_rects,
        screen_size,
        uvec2(240, 400),
        uvec2(70, 70),
        system_time.time_data.time * 2.0,
        ExtraBufferData::NewData(RectMask {
            position: (screen_size.as_vec2() * 0.5).as_uvec2(),
            size: screen_size,
        }),
        &mask,
        &color,
    );

    let mask = MaskType::Rect { border: None };
    let color = ColoringType::RadialGradient(ExtraBufferData::NewData(RadialGradient {
        colors: [RGBA::GREY, RGBA::BLUE.set_alpha(0.0)],
        center_position: vec2(0.0, 0.0),
        end_radius: 50.0,
        start_radius: 15.0,
    }));
    create_new_rect_element(
        gui_rects,
        screen_size,
        uvec2(240, 500),
        uvec2(70, 70),
        system_time.time_data.time * 2.0,
        ExtraBufferData::NewData(RectMask {
            position: (screen_size.as_vec2() * 0.5).as_uvec2(),
            size: screen_size,
        }),
        &mask,
        &color,
    );

    let mask = MaskType::RoundRect {
        border_radius: ExtraBufferData::NewData(BorderRadius {
            top_right: 5.0,
            bottom_right: 10.0,
            top_left: 15.0,
            bottom_left: 20.0,
        }),
        border: None,
    };
    let color = ColoringType::RadialGradient(ExtraBufferData::NewData(RadialGradient {
        colors: [RGBA::GREY, RGBA::BLUE.set_alpha(0.0)],
        center_position: vec2(0.0, 0.0),
        end_radius: 50.0,
        start_radius: 15.0,
    }));
    create_new_rect_element(
        gui_rects,
        screen_size,
        uvec2(310, 100),
        uvec2(70, 70),
        system_time.time_data.time * 2.0,
        ExtraBufferData::NewData(RectMask {
            position: (screen_size.as_vec2() * 0.5).as_uvec2(),
            size: screen_size,
        }),
        &mask,
        &color,
    );
    
    let mask = MaskType::RoundRect {
        border_radius: ExtraBufferData::NewData(BorderRadius {
            top_right: 0.0,
            bottom_right: 10.0,
            top_left: 15.0,
            bottom_left: 0.0,
        }),
        border: None,
    };
    let color = ColoringType::LinearGradient(ExtraBufferData::NewData(LinearGradient {
        colors: [RGBA::GREY, RGBA::RED],
        start_position: vec2(0.0, 0.0),
        end_position: vec2(0.0, 35.0),
    }));
    create_new_rect_element(
        gui_rects,
        screen_size,
        uvec2(310, 200),
        uvec2(70, 70),
        system_time.time_data.time * 2.0,
        ExtraBufferData::NewData(RectMask {
            position: (screen_size.as_vec2() * 0.5).as_uvec2(),
            size: screen_size,
        }),
        &mask,
        &color,
    );

    {
        let glyph_char = 'E';
        let e_char_a = font_atlas_collection[1]
            .font_glyphs
            .iter()
            .find(|elem| elem.character == glyph_char)
            .expect(format!("Glyph {} not found", glyph_char).as_str());
        let maska = MaskType::SDFFont(ExtraBufferData::NewData(TextureSlice {
            sample_component: 1,
            slice_position: e_char_a.tex_coord,
            size: e_char_a.get_padded_size(),
            array_index: 0,
        }));
        let color = ColoringType::LinearGradient(ExtraBufferData::NewData(LinearGradient {
            colors: [RGBA::GREEN, RGBA::RED],
            start_position: vec2(0.0, e_char_a.get_padded_height() as f32 * -0.5),
            end_position: vec2(0.0, e_char_a.get_padded_height() as f32 * 0.5),
        }));
        create_new_rect_element(
            gui_rects,
            screen_size,
            uvec2(310, 300),
            e_char_a.get_padded_size() * 2,
            system_time.time_data.time * 2.0,
            ExtraBufferData::NewData(RectMask {
                position: (screen_size.as_vec2() * 0.5).as_uvec2(),
                size: screen_size,
            }),
            &maska,
            &color,
        );
    }

    {
        let glyph_char = 'E';
        let e_char_a = font_atlas_collection[1]
            .font_glyphs
            .iter()
            .find(|elem| elem.character == glyph_char)
            .expect(format!("Glyph {} not found", glyph_char).as_str());
        let maska = MaskType::SDFFont(ExtraBufferData::NewData(TextureSlice {
            sample_component: 1,
            slice_position: e_char_a.tex_coord,
            size: e_char_a.get_padded_size(),
            array_index: 0,
        }));
        let color = ColoringType::LinearGradient(ExtraBufferData::NewData(LinearGradient {
            colors: [RGBA::BLUE, RGBA::WHITE],
            start_position: vec2(e_char_a.get_padded_width() as f32 * -0.45, e_char_a.get_padded_height() as f32 * -0.5),
            end_position: vec2(e_char_a.get_padded_width() as f32 * 0.45, e_char_a.get_padded_height() as f32 * 0.5),
        }));
        create_new_rect_element(
            gui_rects,
            screen_size,
            uvec2(330, 400),
            e_char_a.get_padded_size() * 2,
            0.0,
            ExtraBufferData::NewData(RectMask {
                position: (screen_size.as_vec2() * 0.5).as_uvec2(),
                size: screen_size,
            }),
            &maska,
            &color,
        );

        let mask = MaskType::Circle { border: None };
        let color = ColoringType::LinearGradient(ExtraBufferData::NewData(LinearGradient{
            colors: [RGBA::RED, RGBA::WHITE],
            start_position: vec2(f32::sin(system_time.time_data.time) * 20.0,0.0),
            end_position: vec2(40.0, 0.0),
        }));
        create_new_rect_element(
            gui_rects,
            screen_size,
            uvec2(330, 500),
            uvec2(70, 70),
            0.0,
            ExtraBufferData::NewData(RectMask {
                position: (screen_size.as_vec2() * 0.5).as_uvec2(),
                size: screen_size,
            }),
            &mask,
            &color,
        );

        let mask = MaskType::Circle { border: None };
        let sin_time =f32::sin(system_time.time_data.time * 4.0);
        let color = ColoringType::LinearGradient(ExtraBufferData::NewData(LinearGradient{
            colors: [RGBA::RED, RGBA::WHITE],
            start_position: vec2( sin_time* 20.0,sin_time * 20.0),
            end_position: vec2(40.0, 40.0),
        }));
        let size_interp = ((f32::sin(system_time.time_data.time) * 50.0) as i32 + 100) as u32;
        create_new_rect_element(
            gui_rects,
            screen_size,
            uvec2(450, 100),
            uvec2(size_interp, 70),
            0.0,
            ExtraBufferData::NewData(RectMask {
                position: (screen_size.as_vec2() * 0.5).as_uvec2(),
                size: screen_size,
            }),
            &mask,
            &color,
        );

        let mask = MaskType::Circle { border: None };
        let color = ColoringType::RadialGradient(ExtraBufferData::NewData(RadialGradient{
            colors: [RGBA::RED, RGBA::WHITE],
            center_position: vec2(0.0,0.0),
            end_radius: 30.0,
            start_radius: 20.0,
        }));
        let sin_time = f32::sin(system_time.time_data.time * 3.0);
        let size_interp = ((sin_time * 30.0) as i32 + 60) as u32;
        create_new_rect_element(
            gui_rects,
            screen_size,
            uvec2(450, 250),
            uvec2(100, size_interp),
            0.0,
            ExtraBufferData::NewData(RectMask {
                position: (screen_size.as_vec2() * 0.5).as_uvec2(),
                size: screen_size,
            }),
            &mask,
            &color,
        );

        let mask = MaskType::Circle { border: None };
        let color = ColoringType::RadialGradient(ExtraBufferData::NewData(RadialGradient{
            colors: [RGBA::BLUE, RGBA::RED.set_alpha(0.0)],
            center_position: vec2(0.0,0.0),
            end_radius: 60.0,
            start_radius: 10.0,
        }));
        let sin_time_rot = f32::sin(system_time.time_data.time * 4.0) * 4.0;
        let sin_time = f32::sin(system_time.time_data.time * 2.0);
        let size_interp = ((sin_time * 30.0) as i32 + 60) as u32;
        create_new_rect_element(
            gui_rects,
            screen_size,
            uvec2(450, 400),
            uvec2(100, size_interp),
            sin_time_rot,
            ExtraBufferData::NewData(RectMask {
                position: (screen_size.as_vec2() * 0.5).as_uvec2(),
                size: screen_size,
            }),
            &mask,
            &color,
        );

        let mask = MaskType::Circle { border: None };
        let color = ColoringType::RadialGradient(ExtraBufferData::NewData(RadialGradient{
            colors: [RGBA::BLUE, RGBA::RED],
            center_position: vec2(0.0,0.0),
            end_radius: 100.0,
            start_radius: 10.0,
        }));
        let sin_time = f32::sin(system_time.time_data.time * 2.0);
        create_new_rect_element(
            gui_rects,
            screen_size,
            uvec2(650, 400),
            uvec2(300, 150),
            0.6,
            ExtraBufferData::NewData(RectMask {
                position: uvec2(450, 450),
                size: uvec2(380, 225),
            }),
            &mask,
            &color,
        );

        let sin_time = f32::sin(system_time.time_data.time * 2.0);
        let cos_time = f32::cos(system_time.time_data.time * 1.5);
        let mask = MaskType::Circle { border: None };
        let color = ColoringType::RadialGradient(ExtraBufferData::NewData(RadialGradient{
            colors: [RGBA::BLUE, RGBA::WHITE],
            center_position: vec2(sin_time * 40.0,cos_time * 20.0),
            end_radius: 70.0,
            start_radius: 30.0,
        }));
        let sin_time = f32::sin(system_time.time_data.time * 3.0);
        let size_interp = ((sin_time * 50.0) as i32 + 100) as u32;
        create_new_rect_element(
            gui_rects,
            screen_size,
            uvec2(650, 250),
            uvec2(180, size_interp),
            0.0,
            ExtraBufferData::NewData(RectMask {
                position: (screen_size.as_vec2() * 0.5).as_uvec2(),
                size: screen_size,
            }),
            &mask,
            &color,
        );
    }
}
