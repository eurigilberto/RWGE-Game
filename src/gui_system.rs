use std::{
    collections::HashMap,
    mem::{discriminant, Discriminant},
};

use rwge::{
    entity_component::{EngineDataTypeKey, PublicDataCollection},
    font::font_atlas::FontAtlas,
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

    pub fn render_char(gui_rects: &mut GUIRects, font_atlas_collection: &Vec<FontAtlas>, collection_index: usize, component_index: u32, glyph_char: char, position: UVec2) {
        let e_char = font_atlas_collection[collection_index]
            .font_glyphs
            .iter()
            .find(|elem| elem.character == glyph_char)
            .expect( format!("Glyph {} not found", glyph_char).as_str());

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
            position_size: [position.x, position.y, ((char_size.x as f32) * 2.0) as u32, ((char_size.y as f32) * 2.0) as u32],
            data_vector_0: [0, tx_pos_index as u32 + 1, 0, 2],
            data_vector_1: [0, 0, 0, dv13],
        };
        gui_rects
            .rect_collection
            .rect_graphic
            .cpu_vector
            .push(test_rect);
    }

    pub fn render(
        &mut self,
        gui_rects: &mut GUIRects,
        encoder: &mut rwge::wgpu::CommandEncoder,
        system_bind_group: &wgpu::BindGroup,
        render_system: &RenderSystem,
        public_data: &mut PublicDataCollection<DataTypeKey, DataType>,
        font_atlas_collection: &Vec<FontAtlas>,
    ) {
        gui_rects.rect_collection.clear_buffers();

        {
            //This section updates the CPU buffers
            {
                //Example rectangle
                let texture_mask_val: u32 = 3;
                let element_type: u32 = 0;

                let dv13 = texture_mask_val << 8 | element_type;

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

                // FONT RENDER TESTING
                GUISystem::render_char(gui_rects, font_atlas_collection, 0, 0, 'E', uvec2(450, 100));
                GUISystem::render_char(gui_rects, font_atlas_collection, 0, 0, 'U', uvec2(510, 100));
                GUISystem::render_char(gui_rects, font_atlas_collection, 0, 0, 'R', uvec2(570, 100));
                GUISystem::render_char(gui_rects, font_atlas_collection, 0, 0, 'I', uvec2(620, 100));

                GUISystem::render_char(gui_rects, font_atlas_collection, 1, 1, 'E', uvec2(450, 200));
                GUISystem::render_char(gui_rects, font_atlas_collection, 1, 1, 'U', uvec2(510, 200));
                GUISystem::render_char(gui_rects, font_atlas_collection, 1, 1, 'R', uvec2(570, 200));
                GUISystem::render_char(gui_rects, font_atlas_collection, 1, 1, 'I', uvec2(620, 200));

                GUISystem::render_char(gui_rects, font_atlas_collection, 2, 2, 'E', uvec2(450, 300));
                GUISystem::render_char(gui_rects, font_atlas_collection, 2, 2, 'U', uvec2(490, 300));
                GUISystem::render_char(gui_rects, font_atlas_collection, 2, 2, 'R', uvec2(530, 300));
                GUISystem::render_char(gui_rects, font_atlas_collection, 2, 2, 'I', uvec2(560, 300));
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
