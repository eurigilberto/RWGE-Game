//For now the style of the tabs is going to be fixed

use rwge::{
    glam::UVec2,
    gui::rect_ui::event::UIEvent,
    slotmap::slotmap::{SlotKey, Slotmap},
};

use crate::public_data::{self, PublicData};

use super::gui_container::GUIContainer;

#[derive(Clone, Copy)]
pub struct GUIContainerSlotkey(SlotKey);

#[derive(Clone, Copy)]
pub struct TabsContainerSlotKey(SlotKey);
pub struct WindowSlotKey(SlotKey);

#[derive(Clone)]
enum SingleOrHorizontal{
	Single(TabsContainerSlotKey),
	Horizontal(Horizontal)
}
#[derive(Clone)]
enum SingleOrVertical{
	Single(TabsContainerSlotKey),
	Vertical(Vertical)
}

#[derive(Clone)]
struct Vertical {
	pub children: Vec<SingleOrHorizontal>
}

#[derive(Clone)]
struct Horizontal{
	pub children: Vec<SingleOrVertical>
}

pub struct TabsContainer {
	tabs: Vec<GUIContainerSlotkey>,
	active_tab: usize
}

impl TabsContainer {
    pub fn new(mut containers: Vec<GUIContainerSlotkey>) -> Self {
		if containers.len() == 0{
			panic!("Cannot create a tab contianer with no guicontainers")
		};

        let mut tabs = Vec::<GUIContainerSlotkey>::new();
		tabs.extend(containers.drain(..));
		
		Self{
            tabs,
            active_tab: 0,
        }
    }
    pub fn handle_event(&mut self, event: &mut UIEvent, public_data: &mut PublicData) {
        //Assume it is the render event
        //First thing would be to render the current window layout things
        match event {
            UIEvent::Render {
                gui_rects,
                container_size,
                container_position,
            } => {
				
			},
            _ => {}
        }
    }
}

pub struct WindowLayout {
    pub window: TabsContainerSlotKey,
    pub size: UVec2,
    pub position: UVec2,
}

impl WindowLayout {
    pub fn new_with_contianer(
        container_key: TabsContainerSlotKey,
        size: UVec2,
        position: UVec2,
    ) -> Self {
        Self {
            window: container_key,
            size,
            position,
        }
    }

    pub fn handle_event(&mut self, event: &mut UIEvent, public_data: &mut PublicData) {
        match event {
            UIEvent::Render {
                gui_rects,
                container_size,
                container_position,
            } => {

			}
            _ => {}
        }
    }
}

pub struct WindowLayouting {
    tabs_container_collection: Slotmap<TabsContainer>,
    window_collection: Vec<WindowLayout>,
    window_order: Vec<usize>,
}

impl WindowLayouting {
    pub fn new() -> Self {
        Self {
            tabs_container_collection: Slotmap::<TabsContainer>::new_with_capacity(10),
            window_collection: Vec::<WindowLayout>::with_capacity(5),
            window_order: Vec::<usize>::with_capacity(5),
        }
    }
}
