use rwge::{
    color::RGBA,
    font::font_atlas::FontAtlas,
    glam::{uvec2, vec2, UVec2},
    gui::rect_ui::{
        element::{
            builder::ElementBuilder, create_new_rect_element, Border, ColoringType, LinearGradient,
            MaskType, RadialGradient, TextureSlice,
        },
        BorderRadius, ExtraBufferData, GUIRects, Rect,
    },
    math_utils::lerp_f32,
};

use crate::runtime_data::EngineTimeData;

pub fn test_screen(time: &EngineTimeData, gui_rects: &mut GUIRects, screen_size: UVec2) {
    ElementBuilder::new(vec2(10.0, 10.0), vec2(10.0, 10.0))
        .set_color(RGBA::GREEN.into())
        .build(gui_rects);

    ElementBuilder::new(vec2(100.0, 100.0), vec2(50.0, 50.0))
        .set_color(RGBA::RED.into())
        .build(gui_rects);

    ElementBuilder::new(vec2(100.0, 200.0), vec2(50.0, 50.0))
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

    {
        ElementBuilder::new(vec2(100.0, 300.0), vec2(70.0, 70.0))
            .set_color(RGBA::BLUE.into())
            .set_round_rect(BorderRadius::ForAll(10.0).into())
            .build(gui_rects);

        {
            ElementBuilder::new(vec2(600.0, 120.0), vec2(60.0, 60.0))
                .set_color(RGBA::RED.into())
                .set_round_rect(BorderRadius::ForAll(10.0).into())
                .set_border(Some(Border {
                    size: 2,
                    color: RGBA::GREEN.into(),
                }))
                .build(gui_rects);

            ElementBuilder::new(vec2(675.0, 120.0), vec2(60.0, 60.0))
                .set_color(RGBA::RED.into())
                .set_circle()
                .set_border(Some(Border {
                    size: 2,
                    color: RGBA::GREEN.into(),
                }))
                .build(gui_rects);

            ElementBuilder::new(vec2(750.0, 120.0), vec2(60.0, 60.0))
                .set_color(RGBA::RED.into())
                .set_border(Some(Border {
                    size: 2,
                    color: RGBA::GREEN.into(),
                }))
                .build(gui_rects);
        }

        {
            let border_size = lerp_f32(5.0, 20.0, time.sin_time(4.0) * 0.5 + 0.5) as u32;

            let border_ = BorderRadius::ForCorners {
                top_right: lerp_f32(5.0, 20.0, time.sin_time(4.0) * 0.5 + 0.5),
                bottom_right: lerp_f32(10.0, 15.0, time.sin_time(2.0) * 0.5 + 0.5),
                top_left: lerp_f32(10.0, 20.0, time.sin_time(3.0) * 0.5 + 0.5),
                bottom_left: lerp_f32(0.0, 30.0, time.sin_time(5.0) * 0.5 + 0.5),
            };

            ElementBuilder::new(vec2(600.0, 50.0), vec2(60.0, 60.0))
                .set_color(RGBA::RED.into())
                .set_round_rect(border_.into())
                .set_border(Some(Border {
                    size: 3,
                    color: RGBA::GREEN.into(),
                }))
                .set_rotation(time.time * 2.5)
                .build(gui_rects);

            ElementBuilder::new(vec2(675.0, 50.0), vec2(60.0, 60.0))
                .set_color(RGBA::RED.into())
                .set_circle()
                .set_border(Some(Border {
                    size: border_size,
                    color: RGBA::GREEN.into(),
                }))
                .set_rotation(time.time * 2.5)
                .build(gui_rects);

            ElementBuilder::new(vec2(750.0, 50.0), vec2(60.0, 60.0))
                .set_color(RGBA::RED.into())
                .set_border(Some(Border {
                    size: border_size,
                    color: RGBA::GREEN.into(),
                }))
                .set_rotation(time.time * 2.5)
                .build(gui_rects);
        }

        let size_x = lerp_f32(60.0, 120.0, time.sin_time(2.0) * 0.5 + 0.5);
        let size_y = lerp_f32(60.0, 120.0, time.cos_time(2.0) * 0.5 + 0.5);
        let circle_rad = lerp_f32(5.0, 20.0, time.sin_time(4.0) * 0.5 + 0.5);
        ElementBuilder::new(vec2(850.0, 100.0), vec2(size_x, size_y))
            .set_color(RGBA::RED.into())
            .set_circle()
            .set_border(Some(Border {
                size: circle_rad as u32,
                color: RGBA::new(0.0, 0.0, 0.5, 1.0).into(),
            }))
            .build(gui_rects);

        ElementBuilder::new(vec2(850.0, 240.0), vec2(70.0, 140.0))
            .set_color(RGBA::RED.into())
            .set_circle()
            .set_border(Some(Border {
                size: 10,
                color: RGBA::new(0.0, 0.0, 0.5, 1.0).into(),
            }))
            .build(gui_rects);

        ElementBuilder::new(vec2(850.0, 360.0), vec2(140.0, 70.0))
            .set_color(RGBA::RED.into())
            .set_circle()
            .set_border(Some(Border {
                size: 10,
                color: RGBA::new(0.0, 0.0, 0.5, 1.0).into(),
            }))
            .build(gui_rects);
    }

    ElementBuilder::new(vec2(100.0, 400.0), vec2(70.0, 70.0))
        .set_color(RGBA::GREY.into())
        .set_circle()
        .build(gui_rects);

    let size_interp = ((f32::sin(time.time) * 50.0) + 100.0);
    ElementBuilder::new(vec2(100.0, 500.0), vec2(size_interp, 70.0))
        .set_circle()
        .set_color(RGBA::GREY.into())
        .build(gui_rects);

    let size_interp = ((f32::sin(time.time) * 50.0) + 100.0);
    ElementBuilder::new(vec2(240.0, 100.0), vec2(size_interp, 70.0))
        .set_circle()
        .set_color(RGBA::RED.into())
        .set_rotation(time.time * 2.0)
        .build(gui_rects);

    let mask = MaskType::Rect { border: None };
    let color = ColoringType::RadialGradient(ExtraBufferData::NewData(RadialGradient {
        colors: [RGBA::GREY, RGBA::BLUE],
        center_position: vec2(0.0, 0.0),
        end_radius: 30.0,
        start_radius: 0.0,
    }));
    create_new_rect_element(
        gui_rects,
        uvec2(240, 400).as_vec2(),
        uvec2(70, 70).as_vec2(),
        time.time * 2.0,
        ExtraBufferData::NewData(Rect {
            position: (screen_size.as_vec2() * 0.5),
            size: screen_size.as_vec2(),
        }),
        &mask,
        &color,
        0
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
        uvec2(240, 500).as_vec2(),
        uvec2(70, 70).as_vec2(),
        time.time * 2.0,
        ExtraBufferData::NewData(Rect {
            position: (screen_size.as_vec2() * 0.5),
            size: screen_size.as_vec2(),
        }),
        &mask,
        &color,
        0
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
        uvec2(310, 100).as_vec2(),
        uvec2(70, 70).as_vec2(),
        time.time * 2.0,
        ExtraBufferData::NewData(Rect {
            position: (screen_size.as_vec2() * 0.5),
            size: screen_size.as_vec2(),
        }),
        &mask,
        &color,
        0
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
        uvec2(310, 200).as_vec2(),
        uvec2(70, 70).as_vec2(),
        time.time * 2.0,
        ExtraBufferData::NewData(Rect {
            position: (screen_size.as_vec2() * 0.5),
            size: screen_size.as_vec2(),
        }),
        &mask,
        &color,
        0
    );

    let mask = MaskType::Circle { border: None };
    let color = ColoringType::LinearGradient(ExtraBufferData::NewData(LinearGradient {
        colors: [RGBA::RED, RGBA::WHITE],
        start_position: vec2(f32::sin(time.time) * 20.0, 0.0),
        end_position: vec2(40.0, 0.0),
    }));
    create_new_rect_element(
        gui_rects,
        uvec2(330, 500).as_vec2(),
        uvec2(70, 70).as_vec2(),
        0.0,
        ExtraBufferData::NewData(Rect {
            position: (screen_size.as_vec2() * 0.5),
            size: screen_size.as_vec2(),
        }),
        &mask,
        &color,
        0
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
        uvec2(450, 100).as_vec2(),
        uvec2(size_interp, 70).as_vec2(),
        0.0,
        ExtraBufferData::NewData(Rect {
            position: (screen_size.as_vec2() * 0.5),
            size: screen_size.as_vec2(),
        }),
        &mask,
        &color,
        0
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
        uvec2(450, 250).as_vec2(),
        uvec2(100, size_interp).as_vec2(),
        0.0,
        ExtraBufferData::NewData(Rect {
            position: (screen_size.as_vec2() * 0.5),
            size: screen_size.as_vec2(),
        }),
        &mask,
        &color,
        0
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
        uvec2(450, 400).as_vec2(),
        uvec2(100, size_interp).as_vec2(),
        sin_time_rot,
        ExtraBufferData::NewData(Rect {
            position: (screen_size.as_vec2() * 0.5),
            size: screen_size.as_vec2(),
        }),
        &mask,
        &color,
        0
    );

    ElementBuilder::new(uvec2(650, 400).as_vec2(), uvec2(300, 150).as_vec2())
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
            Rect {
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
    ElementBuilder::new(position, size)
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
