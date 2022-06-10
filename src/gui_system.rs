use std::{
    collections::HashMap,
    mem::{discriminant, Discriminant},
};

use rwge::{
    entity_component::{EngineDataTypeKey, PublicDataCollection},
    glam::{uvec2, UVec2},
    gui::rect_ui::{
        event::UIEvent,
        graphic::RectGraphic,
        slotmap::{GUIContainer, GUISlotmaps},
        system::GUIRects,
    },
    render_system::RenderSystem,
    slotmap::slotmap::{SlotKey, Slotmap},
    wgpu,
    winit::window,
    EngineDataType,
};

use crate::{DataType, DataTypeKey};

pub struct WindowOne {
    pub size: UVec2,
    pub name: String,
    pub value: f32,
}

pub struct WindowTwo {
    pub size: UVec2,
    pub name: String,
    pub value: f32,
}

impl GUIContainer<DataTypeKey, DataType> for WindowOne {
    fn get_name(&self) -> &String {
        &self.name
    }

    fn allow_resize(&self) -> bool {
        true
    }

    fn get_size(&self) -> UVec2 {
        self.size
    }

    fn update(
        &self,
        event: &UIEvent,
        public_data: &mut rwge::entity_component::PublicDataCollection<DataTypeKey, DataType>,
    ) {
        todo!()
    }
}

impl GUIContainer<DataTypeKey, DataType> for WindowTwo {
    fn get_name(&self) -> &String {
        &self.name
    }

    fn allow_resize(&self) -> bool {
        true
    }

    fn get_size(&self) -> UVec2 {
        self.size
    }

    fn update(
        &self,
        event: &UIEvent,
        public_data: &mut rwge::entity_component::PublicDataCollection<DataTypeKey, DataType>,
    ) {
        todo!()
    }
}

pub enum WindowType {
    WindowOne(Option<GUISlotmaps<DataTypeKey, DataType, WindowOne>>),
    WindowTwo(Option<GUISlotmaps<DataTypeKey, DataType, WindowTwo>>),
}

pub struct WindowKey {
    pub map_key: WindowType,
    pub key: SlotKey,
}

/// This version of the window system is only going to work with windowed spaces. This needs to be refactored in the future to support docking.
pub struct GUISystem {
    pub active_window_collection: Vec<WindowKey>,
    pub window_collection: HashMap<Discriminant<WindowType>, WindowType>,
    pub screen_size: UVec2,
}

impl GUISystem {
    pub fn new(screen_size: UVec2) -> Self {
        let window_one_sm = GUISlotmaps::<DataTypeKey, DataType, WindowOne>::new_with_capacity(20);
        let window_two_sm = GUISlotmaps::<DataTypeKey, DataType, WindowTwo>::new_with_capacity(20);

        let mut window_collection: HashMap<Discriminant<WindowType>, WindowType> =
            HashMap::<Discriminant<WindowType>, WindowType>::new();

        let window_one_sm = WindowType::WindowOne(Some(window_one_sm));
        window_collection.insert(discriminant(&WindowType::WindowOne(None)), window_one_sm);
        let window_two_sm = WindowType::WindowTwo(Some(window_two_sm));
        window_collection.insert(discriminant(&WindowType::WindowOne(None)), window_two_sm);

        let active_window_collection: Vec<WindowKey> = Vec::<WindowKey>::with_capacity(20);

        Self {
            active_window_collection,
            window_collection,
            screen_size,
        }
    }

    pub fn update_window(
        window_collection: &mut HashMap<Discriminant<WindowType>, WindowType>,
        key_data: &WindowKey,
        event: &UIEvent,
        public_data: &mut PublicDataCollection<DataTypeKey, DataType>,
    ) {
        let window_slotmap = window_collection
            .get_mut(&discriminant(&key_data.map_key))
            .unwrap();
        match window_slotmap {
            WindowType::WindowOne(slotmap) => {
                slotmap
                    .as_mut()
                    .expect("Slotmap window one not found")
                    .get_value_mut(key_data.key)
                    .expect("Window one could not be retrieved")
                    .update(&event, public_data);
            }
            WindowType::WindowTwo(slotmap) => {
                slotmap
                    .as_mut()
                    .expect("Slotmap window one not found")
                    .get_value_mut(key_data.key)
                    .expect("Window two could not be retrieved")
                    .update(&event, public_data);
            }
        }
    }

    pub fn handle_event(
        &mut self,
        event: &UIEvent,
        public_data: &mut PublicDataCollection<DataTypeKey, DataType>,
    ) {
        for key in self.active_window_collection.iter() {
            GUISystem::update_window(&mut self.window_collection, key, event, public_data);
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
        gui_rects: &mut GUIRects,
        encoder: &mut rwge::wgpu::CommandEncoder,
        system_bind_group: &wgpu::BindGroup,
        render_system: &RenderSystem,
        public_data: &mut PublicDataCollection<DataTypeKey, DataType>,
    ) {
        gui_rects.rect_collection.clear_buffers();

        {
            //This section updates the CPU buffers
            {
                //Example rectangle
                let texture_mask_val: u32 = 3;
                let element_type: u32 = 0;

                let dv13 = texture_mask_val << 8 + element_type;

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
                    data_vector_1: [0, 0, 2, dv13],
                };
                gui_rects
                    .rect_collection
                    .rect_graphic
                    .cpu_vector
                    .push(test_rect);

                let test_rect = RectGraphic {
                    position_size: [250, 100, 70, 70],
                    data_vector_0: [0, 0, 1, 1],
                    data_vector_1: [0, 0, 1, dv13],
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
            }

            {
                //Right now this should not do anything, as there are not going to be any active windows
                let window_count = self.active_window_collection.len();
                let event = UIEvent::Render(gui_rects);
                for forward_index in 0..window_count {
                    let window_index = window_count - 1 - forward_index;
                    let key = &self.active_window_collection[window_index];
                    GUISystem::update_window(&mut self.window_collection, key, &event, public_data);
                }
            }
        }

        gui_rects.rect_collection.update_gpu_buffers(render_system);

        if let DataType::Base(EngineDataType::RenderTexture(render_texture_slotmap)) = public_data
            .collection
            .get(&DataTypeKey::Base(EngineDataTypeKey::RenderTexture))
            .expect("No Render Texture collection was found")
        {
            let color_rt = render_texture_slotmap
                .get_value(gui_rects.render_texture.color_texture_key.key)
                .expect("Color Render Texture not found");
            let mask_rt = render_texture_slotmap
                .get_value(gui_rects.render_texture.mask_texture_key.key)
                .expect("Mask Render Texture not found");
            rwge::gui::rect_ui::render_pass::render_gui(
                encoder,
                &gui_rects,
                system_bind_group,
                &color_rt.texture_view,
                &mask_rt.texture_view,
            );
        } else {
            panic!("Render texture slotmap not found or Color / Mask RT not found");
        }
    }
}
