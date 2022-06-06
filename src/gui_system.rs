use std::{collections::HashMap, mem::discriminant};

use rwge::{
    entity_component::{EngineDataTypeKey, PublicDataCollection},
    glam::{uvec2, UVec2},
    gui::rect_ui::{
        event::UIEvent,
        slotmap::{GUIContainer, GUISlotmaps},
        system::GUIRects,
    },
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
    WindowOne(GUISlotmaps<DataTypeKey, DataType, WindowOne>),
    WindowTwo(GUISlotmaps<DataTypeKey, DataType, WindowTwo>),
}

#[derive(Hash, PartialEq, Eq, Clone, Copy)]
pub enum WindowTypeKey {
    WindowOne,
    WindowTwo,
}

pub struct WindowKey {
    pub map_key: WindowTypeKey,
    pub key: SlotKey,
}

/// This version of the window system is only going to work with windowed spaces. This needs to be refactored in the future to support docking.
pub struct GUISystem {
    pub active_window_collection: Vec<(WindowTypeKey, SlotKey)>,
    pub window_collection: HashMap<WindowTypeKey, WindowType>,
}

fn update_window(
    window_collection: &mut HashMap<WindowTypeKey, WindowType>,
    gui_type: WindowTypeKey,
    slot_key: SlotKey,
    event: &UIEvent,
    public_data: &mut PublicDataCollection<DataTypeKey, DataType>,
) {
    let window_slotmap = window_collection.get_mut(&gui_type).unwrap();
    match window_slotmap {
        WindowType::WindowOne(slotmap) => slotmap
            .get_value_mut(slot_key)
            .expect("Window one could not be retrieved")
            .update(&event, public_data),
        WindowType::WindowTwo(slotmap) => slotmap
            .get_value_mut(slot_key)
            .expect("Window two could not be retrieved")
            .update(&event, public_data),
    }
}

impl GUISystem {
    pub fn new() -> Self {
        let window_one_sm = GUISlotmaps::<DataTypeKey, DataType, WindowOne>::new_with_capacity(20);
        let window_two_sm = GUISlotmaps::<DataTypeKey, DataType, WindowTwo>::new_with_capacity(20);

        let mut window_collection: HashMap<WindowTypeKey, WindowType> =
            HashMap::<WindowTypeKey, WindowType>::new();
        window_collection.insert(
            WindowTypeKey::WindowOne,
            WindowType::WindowOne(window_one_sm),
        );
        window_collection.insert(
            WindowTypeKey::WindowTwo,
            WindowType::WindowTwo(window_two_sm),
        );

        let active_window_collection: Vec<(WindowTypeKey, SlotKey)> =
            Vec::<(WindowTypeKey, SlotKey)>::with_capacity(20);

        Self {
            active_window_collection,
            window_collection,
        }
    }

    pub fn handle_event(
        &mut self,
        event: &UIEvent,
        public_data: &mut PublicDataCollection<DataTypeKey, DataType>,
    ) {
        for (gui_type, slot_key) in self.active_window_collection.iter() {
            update_window(&mut self.window_collection, *gui_type, *slot_key, event, public_data);
        }
    }

    pub fn update(&mut self, public_data: &mut PublicDataCollection<DataTypeKey, DataType>) {
        /* Nothing yet - The UIEvent to be sent to the GUI containers is going to be created here */
    }

    pub fn render(
        &mut self,
        gui_rects: &mut GUIRects,
        encoder: &mut rwge::wgpu::CommandEncoder,
        system_bind_group: &wgpu::BindGroup,
        public_data: &mut PublicDataCollection<DataTypeKey, DataType>,
    ) {
        {
            let window_count = self.active_window_collection.len();
            let event = UIEvent::Render(gui_rects);
            for forward_index in 0..window_count {
                let window_index = window_count - 1 - forward_index;
                let (gui_type, slot_key) = self.active_window_collection[window_index];
                update_window(&mut self.window_collection, gui_type, slot_key, &event, public_data);
            }
        }

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
