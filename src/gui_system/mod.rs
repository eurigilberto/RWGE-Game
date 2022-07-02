mod testing_structure;
use testing_structure::test_screen;

use std::{
    any::{Any, TypeId},
    ops::Range,
};

mod control;
pub mod gui_container;
mod window_layout;

#[derive(Copy, Clone)]
pub struct ContainerInfo {
    rect: Rect,
    depth_range: (u32, u32),
    //Maybe more in the future
}

impl ContainerInfo {
    pub fn top_left_position(&self) -> Vec2 {
        self.rect.top_left_position()
    }
}

use rwge::{
    color::RGBA,
    glam::{uvec2, vec2, UVec2, Vec2},
    gui::rect_ui::{
        element::{create_new_rect_element, ColoringType, MaskType},
        event::UIEvent,
        GUIRects, Rect,
    },
    slotmap::slotmap::Slotmap,
    Engine,
};

use crate::{
    gui_system::gui_container::{text_animation::TextAnimation, text_layout_test::TextLayoutTest},
    public_data::{
        utils::{get_engine_data, get_render_texture},
        PublicData,
    },
};

use self::{
    gui_container::{
        container_one::ContainerOne, performance_monitor::PerformanceMonitor, GUIContainer,
    },
    window_layout::{DividedElement, GUIContainerSlotkey, WindowSystem},
};

/// This version of the window system is only going to work with windowed spaces. This needs to be refactored in the future to support docking.
pub struct GUISystem {
    pub window_layouting: WindowSystem,
    pub container_collection: Slotmap<Box<dyn GUIContainer>>,
    pub screen_size: UVec2,
}

impl GUISystem {
    pub fn new(screen_size: UVec2) -> Self {
        let mut container_collection = Slotmap::<Box<dyn GUIContainer>>::new_with_capacity(20);

        let mut window_layouting = WindowSystem::new();

        const ELLIPSIS: char = '…';

        let c1_key = window_layouting
            .push_gui_container(Box::new(ContainerOne::new(
                String::from("Wind C1"),
                0.0,
                RGBA::rgb(0.0, 0.25, 0.75),
                50,
            )))
            .unwrap();

        let c2_key = window_layouting
            .push_gui_container(Box::new(ContainerOne::new(
                String::from("Wind C2"),
                0.0,
                RGBA::rgb(0.75, 0.25, 0.0),
                500,
            )))
            .unwrap();

        let c3_key = window_layouting
            .push_gui_container(Box::new(ContainerOne::new(
                String::from("Wind C3"),
                0.0,
                RGBA::rgb(0.1, 0.75, 0.25),
                100,
            )))
            .unwrap();

        let perf_key = window_layouting
            .push_gui_container(Box::new(PerformanceMonitor::new()))
            .unwrap();

        let text_layout_key = window_layouting
            .push_gui_container(Box::new(TextLayoutTest::new()))
            .unwrap();
        let text_animation_key = window_layouting
            .push_gui_container(Box::new(TextAnimation::new()))
            .unwrap();

        let tab_1 = window_layouting.create_tab(vec![c1_key, c2_key]);
        let tab_2 = window_layouting.create_tab(vec![c2_key, c3_key, c1_key]);
        let tab_3 = window_layouting.create_tab(vec![c3_key, c2_key]);
        let perf_tab = window_layouting.create_tab(vec![perf_key]);
        let text_layout = window_layouting.create_tab(vec![text_layout_key]);
        let text_animation = window_layouting.create_tab(vec![text_animation_key]);

        let single_1 = window_layouting
            .create_single_layout_element(tab_1)
            .unwrap();
        let single_2 = window_layouting
            .create_single_layout_element(tab_2)
            .unwrap();
        let single_3 = window_layouting
            .create_single_layout_element(tab_3)
            .unwrap();
        let single_perf = window_layouting
            .create_single_layout_element(perf_tab)
            .unwrap();
        let single_text_lay = window_layouting
            .create_single_layout_element(text_layout)
            .unwrap();
        let single_text_animation = window_layouting
            .create_single_layout_element(text_animation)
            .unwrap();

        let perf_text_anim_vert = window_layouting
            .create_vertical_layout_element(vec![
                DividedElement {
                    layout_key: single_perf,
                    size: 1.0,
                },
                DividedElement {
                    layout_key: single_text_animation,
                    size: 1.0,
                },
            ])
            .unwrap();

        let perf_text_horiz = window_layouting
            .create_horizontal_layout_element(vec![
                DividedElement {
                    layout_key: perf_text_anim_vert,
                    size: 1.0,
                },
                DividedElement {
                    layout_key: single_text_lay,
                    size: 1.2,
                },
            ])
            .unwrap();

        let center_vertical = window_layouting
            .create_vertical_layout_element(vec![
                DividedElement {
                    layout_key: single_1,
                    size: 1.0,
                },
                DividedElement {
                    layout_key: perf_text_horiz,
                    size: 2.0,
                },
            ])
            .unwrap();

        let horizontal_1 = window_layouting
            .create_horizontal_layout_element(vec![
                DividedElement {
                    layout_key: single_1,
                    size: 1.0,
                },
                DividedElement {
                    layout_key: single_2,
                    size: 1.0,
                },
            ])
            .unwrap();

        let vertical_1 = window_layouting
            .create_vertical_layout_element(vec![
                DividedElement {
                    layout_key: single_2,
                    size: 3.0,
                },
                DividedElement {
                    layout_key: horizontal_1,
                    size: 1.0,
                },
            ])
            .unwrap();

        let vertical_2 = window_layouting
            .create_vertical_layout_element(vec![
                DividedElement {
                    layout_key: single_1,
                    size: 1.5,
                },
                DividedElement {
                    layout_key: single_2,
                    size: 1.0,
                },
                DividedElement {
                    layout_key: single_3,
                    size: 2.0,
                },
            ])
            .unwrap();

        let horizontal_2 = window_layouting
            .create_horizontal_layout_element(vec![
                DividedElement {
                    layout_key: vertical_2,
                    size: 1.0,
                },
                DividedElement {
                    layout_key: center_vertical,
                    size: 2.0,
                },
                DividedElement {
                    layout_key: vertical_1,
                    size: 1.25,
                },
            ])
            .unwrap();

        let window_1 = window_layouting.create_window(
            horizontal_2,
            screen_size.as_vec2(),
            screen_size.as_vec2() * 0.5,
        );

        Self {
            window_layouting,
            container_collection,
            screen_size,
        }
    }

    pub fn handle_event(&mut self, event: &mut UIEvent, public_data: &mut PublicData) {
        // Handle Any event FGUI
        match event {
            UIEvent::Resize(new_size) => {
                self.resize(*new_size);
            }
            _ => {}
        }
        self.window_layouting.handle_event(event, public_data)
    }

    pub fn update(&mut self, public_data: &PublicData) {
        /* Nothing yet - The UIEvent to be sent to the GUI containers is going to be created here */
        let mut event = UIEvent::Update;
        self.window_layouting.handle_event(&mut event, public_data)
    }

    pub fn resize(&mut self, new_size: UVec2) {
        self.screen_size = new_size;
    }

    pub fn render(
        &mut self,
        engine: &Engine,
        gui_rects: &mut GUIRects,
        encoder: &mut rwge::wgpu::CommandEncoder,
        public_data: &mut PublicData,
    ) {
        gui_rects.rect_collection.clear_buffers();
        {
            self.window_layouting.render_event(public_data, gui_rects);
        }
        gui_rects
            .rect_collection
            .update_gpu_buffers(&engine.render_system);

        {
            let color_rt =
                get_render_texture(&public_data, &gui_rects.render_texture.color_texture_key)
                    .expect("GUI Color render target was not present");
            let mask_rt =
                get_render_texture(&public_data, &gui_rects.render_texture.mask_texture_key)
                    .expect("GUI Masking render target was not present");

            rwge::gui::rect_ui::render_pass::render_gui(
                encoder,
                &gui_rects,
                &engine.system_bind_group,
                &color_rt.texture_view,
                &mask_rt.texture_view,
            );
        }
    }
}
