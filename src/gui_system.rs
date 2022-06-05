use std::collections::HashMap;

use rwge::{
    glam::{UVec2, uvec2},
    gui::rect_ui::{slotmap::{GUIContainer, GUISlotmaps}, system::GUIRects, event::UIEvent}, slotmap::slotmap::{Slotmap, SlotKey}, EngineDataSlotmapTypes,
};

use crate::DataSlotmap;

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

impl GUIContainer<DataSlotmap> for WindowOne {
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
        event: &rwge::gui::rect_ui::event::UIEvent,
        public_data: &mut Slotmap<DataSlotmap>,
    ) {
        
    }
}

impl GUIContainer<DataSlotmap> for WindowTwo {
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
        event: &rwge::gui::rect_ui::event::UIEvent,
        public_data: &mut Slotmap<DataSlotmap>,
    ) {
        
    }
}

pub enum WindowType{
    WindowOne(GUISlotmaps<DataSlotmap, WindowOne>),
    WindowTwo(GUISlotmaps<DataSlotmap, WindowTwo>)
}

#[derive(Hash, PartialEq, Eq)]
pub enum WindowTypeKey{
    WindowOne,
    WindowTwo
}

pub struct WindowKey{
    pub map_key: WindowTypeKey,
    pub key: SlotKey
}

/// This version of the window system is only going to work with windowed spaces. This needs to be refactored in the future to support docking.
pub struct GUISystem {
    pub gui_rects: GUIRects,
    pub active_window_collection: Vec<(WindowTypeKey, SlotKey)>,
    pub window_collection: HashMap<WindowTypeKey, WindowType>
}

impl GUISystem {
    pub fn new(gui_rects: GUIRects)->Self{

        let window_one_sm = GUISlotmaps::<DataSlotmap, WindowOne>::new_with_capacity(20);
        let window_two_sm = GUISlotmaps::<DataSlotmap, WindowTwo>::new_with_capacity(20);

        let mut window_collection: HashMap<WindowTypeKey, WindowType> = HashMap::<WindowTypeKey, WindowType>::new();
        window_collection.insert(WindowTypeKey::WindowOne, WindowType::WindowOne(window_one_sm));
        window_collection.insert(WindowTypeKey::WindowTwo, WindowType::WindowTwo(window_two_sm));

        let active_window_collection: Vec<(WindowTypeKey, SlotKey)> = Vec::<(WindowTypeKey, SlotKey)>::with_capacity(20);

        Self{
            gui_rects,
            active_window_collection,
            window_collection,
        }
    }

    pub fn handle_event(&mut self, event: &UIEvent, public_data: &mut Slotmap<DataSlotmap>){
        for (window_type, slot_key) in self.active_window_collection.iter(){
            let window_slotmap = self.window_collection.get_mut(window_type).unwrap();
            match window_slotmap {
                WindowType::WindowOne(slotmap) => {
                    slotmap.get_value_mut(slot_key.clone()).unwrap().update(event, public_data)
                },
                WindowType::WindowTwo(slotmap) => {
                    slotmap.get_value_mut(slot_key.clone()).unwrap().update(event, public_data)
                },
            }
        }
    }
}
