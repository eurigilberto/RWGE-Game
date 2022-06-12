use std::{
    collections::HashMap,
    mem::{discriminant, Discriminant},
};

mod test;
use test::test_screen;

use rwge::{
    engine,
    entity_component::{EngineDataTypeKey, PublicDataCollection},
    font::font_atlas::FontAtlas,
    glam::{uvec2, UVec2},
    gui::rect_ui::{event::UIEvent, graphic::RectGraphic, slotmap::GUIContainer, system::GUIRects},
    render_system::RenderSystem,
    slotmap::slotmap::{SlotKey, Slotmap},
    wgpu,
    winit::window,
    Engine, EngineDataType,
};

use crate::{DataType, DataTypeKey};

pub struct WindowOne {
    pub size: UVec2,
    pub name: String,
    pub value: f32,
}

impl GUIContainer<PublicDataCollection<DataTypeKey, DataType>> for WindowOne {
    fn get_name(&self) -> &String {
        &self.name
    }

    fn handle_event(
        &self,
        event: &mut UIEvent,
        public_data: &mut PublicDataCollection<DataTypeKey, DataType>,
        engine: &Engine,
    ) {
        match event {
            UIEvent::Render {
                gui_rects,
                container_size,
                container_position,
            } => {
                let texture_mask_val: u32 = 3;
                let element_type: u32 = 0;

                let dv13 = texture_mask_val << 8 | element_type;

                let color_index = gui_rects.rect_collection.color.cpu_vector.len() + 1;
                gui_rects
                    .rect_collection
                    .color
                    .cpu_vector
                    .push([0.3, 0.75, 0.3, 1.0]);

                let test_rect = RectGraphic {
                    position_size: [
                        container_position.x,
                        container_position.y,
                        container_size.x,
                        container_size.y,
                    ],
                    data_vector_0: [0, 0, 0, color_index as u32],
                    data_vector_1: [0, 0, 0, dv13],
                };
                gui_rects
                    .rect_collection
                    .rect_graphic
                    .cpu_vector
                    .push(test_rect);
            }
            _ => {}
        }
    }

    fn allow_resize(&self) -> bool {
        true
    }

    fn get_size(&self) -> UVec2 {
        self.size
    }
}

pub struct WindowContainer {
    pub slot_key: SlotKey,
    pub position: UVec2,
    pub size: UVec2,
}

impl WindowContainer {
    fn handle_event(&mut self, event: &mut UIEvent, engine: &Engine) {
        match event {
            UIEvent::MouseButton(button) => {},
            UIEvent::MouseMove(position) => {
				self.position = position.data.as_uvec2();
			},
            UIEvent::MouseWheel(_) => {},
            UIEvent::KeyboardInput(_) => {},
            UIEvent::Update => {},
            UIEvent::Render {
                gui_rects,
                container_size,
                container_position,
            } => {
				
			},
        }
    }
}

/// This version of the window system is only going to work with windowed spaces. This needs to be refactored in the future to support docking.
pub struct GUISystem {
    pub active_window_collection: Vec<WindowContainer>,
    pub window_collection:
        Slotmap<Box<dyn GUIContainer<PublicDataCollection<DataTypeKey, DataType>>>>,
    pub screen_size: UVec2,
}

impl GUISystem {
    pub fn new(screen_size: UVec2) -> Self {
        let mut window_collection: Slotmap<
            Box<dyn GUIContainer<PublicDataCollection<DataTypeKey, DataType>>>,
        > = Slotmap::new_with_capacity(20);
        let slot_key = window_collection
            .push(Box::new(WindowOne {
                size: uvec2(100, 200),
                name: String::from("Test 1"),
                value: 20.0,
            }))
            .expect("Could not push a new window to collection");

        let mut active_window_collection: Vec<WindowContainer> =
            Vec::<WindowContainer>::with_capacity(20);

        active_window_collection.push(WindowContainer {
            slot_key: slot_key,
            position: uvec2(500, 500),
            size: uvec2(200, 250),
        });

        Self {
            active_window_collection,
            window_collection,
            screen_size,
        }
    }

    pub fn handle_event(
        &mut self,
        event: &mut UIEvent,
        public_data: &mut PublicDataCollection<DataTypeKey, DataType>,
        engine: &Engine,
    ) {
        for window_container in self.active_window_collection.iter_mut() {
            //window_container.handle event or something

            window_container.handle_event(event, engine);

            self.window_collection
                .get_value_mut(&window_container.slot_key)
                .expect("Window was removed")
                .handle_event(event, public_data, engine);
        }
    }

    pub fn update(&mut self, public_data: &mut PublicDataCollection<DataTypeKey, DataType>) {
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
        public_data: &mut PublicDataCollection<DataTypeKey, DataType>,
        font_atlas_collection: &Vec<FontAtlas>,
    ) {
        gui_rects.rect_collection.clear_buffers();

        {
            let window_count = self.active_window_collection.len();
            for forward_index in 0..window_count {
                let window_index = window_count - 1 - forward_index;
                let window_container = &mut self.active_window_collection[window_index];
                //window_container.render background
                let mut event = UIEvent::Render {
                    gui_rects: gui_rects,
                    container_size: window_container.size,
                    container_position: window_container.position,
                };
                self.window_collection
                    .get_value_mut(&window_container.slot_key)
                    .expect("Window not found")
                    .handle_event(&mut event, public_data, engine);
            }
            test_screen(&engine.time, gui_rects, font_atlas_collection);
        }

        gui_rects
            .rect_collection
            .update_gpu_buffers(&engine.render_system);

        if let DataType::Base(EngineDataType::RenderTexture(render_texture_slotmap)) = public_data
            .collection
            .get(&DataTypeKey::Base(EngineDataTypeKey::RenderTexture))
            .expect("No Render Texture collection was found")
        {
            let color_rt = render_texture_slotmap
                .get_value(&gui_rects.render_texture.color_texture_key.key)
                .expect("Color Render Texture not found");
            let mask_rt = render_texture_slotmap
                .get_value(&gui_rects.render_texture.mask_texture_key.key)
                .expect("Mask Render Texture not found");
            rwge::gui::rect_ui::render_pass::render_gui(
                encoder,
                &gui_rects,
                &engine.system_bind_group,
                &color_rt.texture_view,
                &mask_rt.texture_view,
            );
        } else {
            panic!("Render texture slotmap not found or Color / Mask RT not found");
        }
    }
}
