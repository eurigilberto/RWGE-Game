mod testing_structure;
use testing_structure::test_screen;

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
    glam::{UVec2, Vec2},
    gui::rect_ui::{
        event::UIEvent,
        GUIRects, Rect,
    },
    slotmap::slotmap::Slotmap,
    Engine,
};

use crate::{
    gui_system::{
        gui_container::{text_animation::TextAnimation, text_layout_test::TextLayoutTest},
        window_layout::TabsSlotKey,
    },
    runtime_data::{
        utils::{get_engine_data, get_render_texture},
        RuntimeData, PublicData,
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
        let mut container_collection = Slotmap::<Box<dyn GUIContainer>>::with_capacity(30);

        let mut window_layouting = WindowSystem::new();

        //const ELLIPSIS: char = '…';
        const COL_1: RGBA = RGBA::rgb(0.0, 0.5, 0.5);
        const COL_2: RGBA = RGBA::rgb(0.75, 0.25, 0.0);
        const COL_3: RGBA = RGBA::rgb(0.1, 0.75, 0.25);

        let window_box_props = vec![
            ("W | 1", COL_1, 50),
            ("W | 2", COL_2, 500),
            //-------------------------||
            ("W | 3", COL_1, 50),
            ("W | 4", COL_2, 500),
            ("W | 5", COL_3, 100),
            //-------------------------||
            ("W | 6", COL_1, 50),
            ("W | 7", COL_2, 500),
            //-------------------------||
            ("W | 8", COL_1, 50),
            ("W | 9", COL_2, 500),
            //-------------------------||
            ("W | 10", COL_1, 50),
            ("W | 11", COL_2, 100),
            ("W | 12", COL_3, 500),
            //-------------------------||
            ("W | 13", COL_1, 50),
            ("W | 14", COL_2, 500),
            //-------------------------||
            ("W | 15", COL_1, 50),
            ("W | 16", COL_2, 100),
            ("W | 17", COL_3, 500),
        ];

        let tab_groups: Vec<Vec<usize>> = vec![
            vec![0, 1],
            vec![2, 3, 4],
            vec![5, 6],
            vec![7, 8],
            vec![9, 10, 11],
            vec![12, 13],
            vec![14, 15, 16],
        ];

        let window_box_keys: Vec<GUIContainerSlotkey> = window_box_props
            .iter()
            .map(|w_box| {
                window_layouting
                    .push_gui_container(Box::new(ContainerOne::new(
                        String::from(w_box.0),
                        0.0,
                        w_box.1,
                        w_box.2,
                    )))
                    .unwrap()
            })
            .collect();

        let tab_group_keys: Vec<TabsSlotKey> = tab_groups
            .iter()
            .map(|group| {
                window_layouting
                    .create_tab(group.iter().map(|g_id| window_box_keys[*g_id]).collect())
            })
            .collect();

        let perf_key = window_layouting
            .push_gui_container(Box::new(PerformanceMonitor::new()))
            .unwrap();
        let text_layout_key = window_layouting
            .push_gui_container(Box::new(TextLayoutTest::new()))
            .unwrap();
        let text_animation_key = window_layouting
            .push_gui_container(Box::new(TextAnimation::new()))
            .unwrap();

        let perf_tab = window_layouting.create_tab(vec![perf_key]);
        let text_layout_tab = window_layouting.create_tab(vec![text_layout_key]);
        let text_animation_tab = window_layouting.create_tab(vec![text_animation_key]);

        let mut wl = window_layouting;
        let v1 = DividedElement::new_layout(
            wl.push_vertical(vec![
                DividedElement::new_tab(tab_group_keys[4], 1.5),
                DividedElement::new_tab(tab_group_keys[5], 1.0),
                DividedElement::new_tab(tab_group_keys[6], 2.0),
            ])
            .unwrap(),
            1.0,
        );

        let v2 = {
            let d_1 = DividedElement::new_layout(
                wl.push_vertical(vec![
                    DividedElement::new_tab(perf_tab, 1.0),
                    DividedElement::new_tab(text_animation_tab, 1.0),
                ])
                .unwrap(),
                1.0,
            );
            let tl = DividedElement::new_tab(text_layout_tab, 1.2);
            let d_2 = DividedElement::new_layout(
                wl.push_horizontal(vec![d_1, tl])
                    .unwrap(),
                2.0,
            );

            DividedElement::new_layout(
                wl.push_vertical(vec![
                    DividedElement::new_tab(tab_group_keys[0], 1.0),
                    d_2,
                ])
                .unwrap(),
                2.0,
            )
        };

        let h1 = DividedElement::new_layout(
            wl.push_horizontal(vec![
                DividedElement::new_tab(tab_group_keys[1], 1.0),
                DividedElement::new_tab(tab_group_keys[2], 1.0),
            ])
            .unwrap(),
            1.0,
        );

        let v3 = DividedElement::new_layout(
            wl.push_vertical(vec![DividedElement::new_tab(tab_group_keys[3], 3.0), h1])
                .unwrap(),
            1.25,
        );

        let horizontal_2 = wl.push_horizontal(vec![v1, v2, v3]).unwrap();

        let _ = wl.create_window(
            horizontal_2,
            screen_size.as_vec2(),
            screen_size.as_vec2() * 0.5,
        );

        Self {
            window_layouting: wl,
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
