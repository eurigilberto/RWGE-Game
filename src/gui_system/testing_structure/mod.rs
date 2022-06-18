use rwge::{
    color::RGBA,
    font::font_atlas::FontAtlas,
    glam::{uvec2, vec2, UVec2},
    gui::rect_ui::{
        element::{
            builder::ElementBuilder, create_new_rect_element, ColoringType, LinearGradient,
            MaskType, RadialGradient, TextureSlice,
        },
        BorderRadius, ExtraBufferData, GUIRects, RectMask,
    },
};

use crate::public_data::EngineTimeData;

pub fn test_screen(
    time: &EngineTimeData,
    gui_rects: &mut GUIRects,
    font_atlas_collection: &Vec<FontAtlas>,
    screen_size: UVec2,
) {
    ElementBuilder::new(screen_size, vec2(10.0, 10.0), vec2(10.0, 10.0))
        .set_color(RGBA::GREEN.into())
        .build(gui_rects);

    ElementBuilder::new(screen_size, vec2(100.0, 100.0), vec2(50.0, 50.0))
        .set_color(RGBA::RED.into())
        .build(gui_rects);

    ElementBuilder::new(screen_size, vec2(100.0, 200.0), vec2(50.0, 50.0))
        .set_color(RGBA::BLUE.into())
        .set_round_rect(
            BorderRadius::ForCorners {
                top_right: 5.0,
                bottom_right: 10.0,
                top_left: 15.0,
                bottom_left: 20.0,
            }
            .into(),
        )
        .build(gui_rects);

    ElementBuilder::new(screen_size, vec2(100.0, 300.0), vec2(70.0, 70.0))
        .set_color(RGBA::BLUE.into())
        .set_round_rect(BorderRadius::ForAll(10.0).into())
        .build(gui_rects);

    ElementBuilder::new(screen_size, vec2(100.0, 400.0), vec2(70.0, 70.0))
        .set_color(RGBA::GREY.into())
        .set_circle()
        .build(gui_rects);

    let size_interp = ((f32::sin(time.time) * 50.0) + 100.0);
    ElementBuilder::new(screen_size, vec2(100.0, 500.0), vec2(size_interp, 70.0))
        .set_circle()
        .set_color(RGBA::GREY.into())
        .build(gui_rects);

    let size_interp = ((f32::sin(time.time) * 50.0) + 100.0);
    ElementBuilder::new(screen_size, vec2(240.0, 100.0), vec2(size_interp, 70.0))
        .set_circle()
        .set_color(RGBA::RED.into())
        .set_rotation(time.time * 2.0)
        .build(gui_rects);

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
            vec2(200.0, 200.0),
            e_char.get_padded_size().as_vec2() * 2.0,
            time.time * 2.0,
            ExtraBufferData::NewData(RectMask {
                position: (screen_size.as_vec2() * 0.5),
                size: screen_size.as_vec2(),
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
            vec2(200.0, 300.0),
            e_char_a.get_padded_size().as_vec2(),
            time.time * 2.0,
            ExtraBufferData::NewData(RectMask {
                position: (screen_size.as_vec2() * 0.5),
                size: screen_size.as_vec2(),
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
        uvec2(240, 400).as_vec2(),
        uvec2(70, 70).as_vec2(),
        time.time * 2.0,
        ExtraBufferData::NewData(RectMask {
            position: (screen_size.as_vec2() * 0.5),
            size: screen_size.as_vec2(),
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
        uvec2(240, 500).as_vec2(),
        uvec2(70, 70).as_vec2(),
        time.time * 2.0,
        ExtraBufferData::NewData(RectMask {
            position: (screen_size.as_vec2() * 0.5),
            size: screen_size.as_vec2(),
        }),
        &mask,
        &color,
    );

    let mask = MaskType::RoundRect {
        border_radius: ExtraBufferData::NewData(BorderRadius::ForCorners {
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
        uvec2(310, 100).as_vec2(),
        uvec2(70, 70).as_vec2(),
        time.time * 2.0,
        ExtraBufferData::NewData(RectMask {
            position: (screen_size.as_vec2() * 0.5),
            size: screen_size.as_vec2(),
        }),
        &mask,
        &color,
    );

    let mask = MaskType::RoundRect {
        border_radius: ExtraBufferData::NewData(BorderRadius::ForCorners {
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
        uvec2(310, 200).as_vec2(),
        uvec2(70, 70).as_vec2(),
        time.time * 2.0,
        ExtraBufferData::NewData(RectMask {
            position: (screen_size.as_vec2() * 0.5),
            size: screen_size.as_vec2(),
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
            uvec2(310, 300).as_vec2(),
            e_char_a.get_padded_size().as_vec2() * 2.0,
            time.time * 2.0,
            ExtraBufferData::NewData(RectMask {
                position: (screen_size.as_vec2() * 0.5),
                size: screen_size.as_vec2(),
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
            start_position: vec2(
                e_char_a.get_padded_width() as f32 * -0.45,
                e_char_a.get_padded_height() as f32 * -0.5,
            ),
            end_position: vec2(
                e_char_a.get_padded_width() as f32 * 0.45,
                e_char_a.get_padded_height() as f32 * 0.5,
            ),
        }));
        create_new_rect_element(
            gui_rects,
            screen_size,
            uvec2(330, 400).as_vec2(),
            e_char_a.get_padded_size().as_vec2() * 2.0,
            0.0,
            ExtraBufferData::NewData(RectMask {
                position: (screen_size.as_vec2() * 0.5),
                size: screen_size.as_vec2(),
            }),
            &maska,
            &color,
        );

        let mask = MaskType::Circle { border: None };
        let color = ColoringType::LinearGradient(ExtraBufferData::NewData(LinearGradient {
            colors: [RGBA::RED, RGBA::WHITE],
            start_position: vec2(f32::sin(time.time) * 20.0, 0.0),
            end_position: vec2(40.0, 0.0),
        }));
        create_new_rect_element(
            gui_rects,
            screen_size,
            uvec2(330, 500).as_vec2(),
            uvec2(70, 70).as_vec2(),
            0.0,
            ExtraBufferData::NewData(RectMask {
                position: (screen_size.as_vec2() * 0.5),
                size: screen_size.as_vec2(),
            }),
            &mask,
            &color,
        );

        let mask = MaskType::Circle { border: None };
        let sin_time = f32::sin(time.time * 4.0);
        let color = ColoringType::LinearGradient(ExtraBufferData::NewData(LinearGradient {
            colors: [RGBA::RED, RGBA::WHITE],
            start_position: vec2(sin_time * 20.0, sin_time * 20.0),
            end_position: vec2(40.0, 40.0),
        }));
        let size_interp = ((f32::sin(time.time) * 50.0) as i32 + 100) as u32;
        create_new_rect_element(
            gui_rects,
            screen_size,
            uvec2(450, 100).as_vec2(),
            uvec2(size_interp, 70).as_vec2(),
            0.0,
            ExtraBufferData::NewData(RectMask {
                position: (screen_size.as_vec2() * 0.5),
                size: screen_size.as_vec2(),
            }),
            &mask,
            &color,
        );

        let mask = MaskType::Circle { border: None };
        let color = ColoringType::RadialGradient(ExtraBufferData::NewData(RadialGradient {
            colors: [RGBA::RED, RGBA::WHITE],
            center_position: vec2(0.0, 0.0),
            end_radius: 30.0,
            start_radius: 20.0,
        }));
        let sin_time = f32::sin(time.time * 3.0);
        let size_interp = ((sin_time * 30.0) as i32 + 60) as u32;
        create_new_rect_element(
            gui_rects,
            screen_size,
            uvec2(450, 250).as_vec2(),
            uvec2(100, size_interp).as_vec2(),
            0.0,
            ExtraBufferData::NewData(RectMask {
                position: (screen_size.as_vec2() * 0.5),
                size: screen_size.as_vec2(),
            }),
            &mask,
            &color,
        );

        let mask = MaskType::Circle { border: None };
        let color = ColoringType::RadialGradient(ExtraBufferData::NewData(RadialGradient {
            colors: [RGBA::BLUE, RGBA::RED.set_alpha(0.0)],
            center_position: vec2(0.0, 0.0),
            end_radius: 60.0,
            start_radius: 10.0,
        }));
        let sin_time_rot = f32::sin(time.time * 4.0) * 4.0;
        let sin_time = f32::sin(time.time * 2.0);
        let size_interp = ((sin_time * 30.0) as i32 + 60) as u32;
        create_new_rect_element(
            gui_rects,
            screen_size,
            uvec2(450, 400).as_vec2(),
            uvec2(100, size_interp).as_vec2(),
            sin_time_rot,
            ExtraBufferData::NewData(RectMask {
                position: (screen_size.as_vec2() * 0.5),
                size: screen_size.as_vec2(),
            }),
            &mask,
            &color,
        );

        ElementBuilder::new(screen_size, uvec2(650, 400).as_vec2(), uvec2(300, 150).as_vec2())
            .set_circle()
            .set_radial_gradient(
                RadialGradient {
                    colors: [RGBA::BLUE, RGBA::RED],
                    center_position: vec2(0.0, 0.0),
                    end_radius: 100.0,
                    start_radius: 10.0,
                }
                .into(),
            )
            .set_rect_mask(
                RectMask {
                    position: uvec2(450, 450).as_vec2(),
                    size: uvec2(380, 225).as_vec2(),
                }
                .into(),
            )
            .set_rotation(0.6)
            .build(gui_rects);

        let sin_time = time.sin_time(2.0);
        let cos_time = time.cos_time(1.5);
        let size_interp = sin_time * 50.0 + 100.0;
        let center_position = vec2(sin_time * 40.0, cos_time * 20.0);
        let size = vec2(180.0, size_interp);
        let position = vec2(650.0, 250.0);
        ElementBuilder::new(screen_size, position, size)
            .set_radial_gradient(
                RadialGradient {
                    colors: [RGBA::BLUE, RGBA::WHITE],
                    center_position,
                    end_radius: 70.0,
                    start_radius: 30.0,
                }
                .into(),
            )
            .set_circle()
            .build(gui_rects);
    }
}
