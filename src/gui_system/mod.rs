mod testing_structure;
use testing_structure::test_screen;

use std::any::{Any, TypeId};

mod gui_container;
mod window_layout;

use rwge::{
    font::font_atlas::FontAtlas,
    glam::{uvec2, UVec2},
    gui::rect_ui::{
        element::{create_new_rect_element, ColoringType, MaskType},
        event::UIEvent,
        graphic::RectGraphic,
        GUIRects,
    },
    slotmap::slotmap::Slotmap,
    Engine,
};

use crate::public_data::{utils::get_render_texture, PublicData};

use self::{
    gui_container::{container_one::ContainerOne, GUIContainer},
    window_layout::{DividedElement, GUIContainerSlotkey, WindowLayouting},
};

/// This version of the window system is only going to work with windowed spaces. This needs to be refactored in the future to support docking.
pub struct GUISystem {
    pub window_layouting: WindowLayouting,
    pub container_collection: Slotmap<Box<dyn GUIContainer>>,
    pub screen_size: UVec2,
}

impl GUISystem {
    pub fn new(screen_size: UVec2) -> Self {
        let mut container_collection = Slotmap::<Box<dyn GUIContainer>>::new_with_capacity(20);

        let mut window_layouting = WindowLayouting::new();

        let c1_key = window_layouting
            .push_gui_container(Box::new(ContainerOne {
                name: String::from("Wind C1"),
                value: 0.4,
                cound: 10,
            }))
            .unwrap();

        let c2_key = window_layouting
            .push_gui_container(Box::new(ContainerOne {
                name: String::from("Wind C2"),
                value: 0.6,
                cound: 5,
            }))
            .unwrap();

        let c3_key = window_layouting
            .push_gui_container(Box::new(ContainerOne {
                name: String::from("Wind C3"),
                value: 0.75,
                cound: 7,
            }))
            .unwrap();

        let tab_1 = window_layouting.create_tab(vec![c1_key, c2_key]);
        let tab_2 = window_layouting.create_tab(vec![c2_key, c3_key]);
        let tab_3 = window_layouting.create_tab(vec![c3_key]);

        let single_1 = window_layouting
            .create_single_layout_element(tab_1)
            .unwrap();
        let single_2 = window_layouting
            .create_single_layout_element(tab_2)
            .unwrap();
        let single_3 = window_layouting
            .create_single_layout_element(tab_3)
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
                    size: 1.0,
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
                    size: 1.0,
                },
                DividedElement {
                    layout_key: single_2,
                    size: 1.0,
                },
                DividedElement {
                    layout_key: single_3,
                    size: 1.0,
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
                    layout_key: single_1,
                    size: 1.0,
                },
                DividedElement {
                    layout_key: vertical_1,
                    size: 1.0,
                },
            ])
            .unwrap();

        let window_1 = window_layouting.create_window(
            horizontal_2,
            screen_size,
            (screen_size.as_vec2() * 0.5).as_uvec2(),
        );

        Self {
            window_layouting,
            container_collection,
            screen_size,
        }
    }

    pub fn handle_event(
        &mut self,
        event: &mut UIEvent,
        public_data: &mut PublicData,
        public_data_changes: &Option<&mut Vec<Box<dyn FnMut(&mut PublicData) -> ()>>>,
        engine: &Engine,
    ) {
        // Handle Any event FGUI
        self.window_layouting.handle_event(event, public_data_changes, public_data)
    }

    pub fn update(&mut self, public_data: &mut PublicData) {
        /* Nothing yet - The UIEvent to be sent to the GUI containers is going to be created here */
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
        font_atlas_collection: &Vec<FontAtlas>,
    ) {
        gui_rects.rect_collection.clear_buffers();

        {
            let mut event = UIEvent::Render { gui_rects };
            self.window_layouting.handle_event(&mut event, &None, public_data);
            test_screen(
                &engine.time,
                gui_rects,
                font_atlas_collection,
                self.screen_size,
            );
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
