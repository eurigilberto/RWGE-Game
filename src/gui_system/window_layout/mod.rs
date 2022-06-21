use std::ops::Deref;
mod layout_element;
use layout_element::LayoutElement;
mod tabs_container;
use tabs_container::TabsContainer;
mod window;
use window::Window;

//For now the style of the tabs is going to be fixed
use rwge::{
    glam::Vec2,
    gui::rect_ui::{event::UIEvent, element::builder::ElementBuilder, GUIRects},
    slotmap::slotmap::{SlotKey, Slotmap},
};

use crate::public_data::PublicData;

use super::{control::ControlState, gui_container::GUIContainer, ContainerInfo};

rwge::create_custom_key!(
    GUIContainerSlotkey;
);
rwge::create_custom_key!(
    TabsSlotKey;
);
rwge::create_custom_key!(
    LayoutSlotKey;
);
rwge::create_custom_key!(
    WindowSlotKey;
);

pub struct DividedElement {
    pub layout_key: LayoutSlotKey,
    pub size: f32,
}

pub struct TabLayoutInfo {
    key: TabsSlotKey,
    container_info: ContainerInfo,
}
#[derive(Clone, Copy)]
pub enum LayoutOrTabKey {
    TabKey(TabsSlotKey),
    LayoutKey(LayoutSlotKey),
}
pub struct LayoutOrTabInfo {
    key: LayoutOrTabKey,
    container_info: ContainerInfo,
}
pub struct GUIContainerInfo {
    key: GUIContainerSlotkey,
    container_info: ContainerInfo,
}

pub struct WindowSystem {
    gui_contianer_collection: Slotmap<Box<dyn GUIContainer>>,
    tabs_container_collection: Slotmap<TabsContainer>,
    layout_elements_collection: Slotmap<LayoutElement>,
    window_collection: Slotmap<Window>,
    window_order: Vec<WindowSlotKey>,
    control_state: ControlState,
}

impl WindowSystem {
    pub fn new() -> Self {
        Self {
            gui_contianer_collection: Slotmap::<Box<dyn GUIContainer>>::new_with_capacity(20),
            tabs_container_collection: Slotmap::<TabsContainer>::new_with_capacity(10),
            layout_elements_collection: Slotmap::<LayoutElement>::new_with_capacity(10),
            window_collection: Slotmap::<Window>::new_with_capacity(5),
            window_order: Vec::<WindowSlotKey>::with_capacity(5),
            control_state: ControlState::new(),
        }
    }

    pub fn create_tab(&mut self, gui_container: Vec<GUIContainerSlotkey>) -> TabsSlotKey {
        let t_container = TabsContainer::new(gui_container);
        let t_container_key = self.tabs_container_collection.push(t_container).unwrap();
        TabsSlotKey(t_container_key)
    }

    pub fn create_window(
        &mut self,
        layout_key: LayoutSlotKey,
        size: Vec2,
        position: Vec2,
    ) -> WindowSlotKey {
        let window_layout = Window::new_with_contianer(layout_key, size, position);
        let window_key = WindowSlotKey(self.window_collection.push(window_layout).unwrap());
        self.window_order.push(window_key);
        window_key
    }

    pub fn create_vertical_layout_element(
        &mut self,
        children: Vec<DividedElement>,
    ) -> Option<LayoutSlotKey> {
        LayoutElement::create_vertical(children, &mut self.layout_elements_collection)
    }

    pub fn create_horizontal_layout_element(
        &mut self,
        children: Vec<DividedElement>,
    ) -> Option<LayoutSlotKey> {
        LayoutElement::create_horizontal(children, &mut self.layout_elements_collection)
    }

    pub fn create_single_layout_element(&mut self, tab_key: TabsSlotKey) -> Option<LayoutSlotKey> {
        match self
            .layout_elements_collection
            .push(LayoutElement::Single(tab_key))
        {
            Some(key) => Some(LayoutSlotKey(key)),
            None => None,
        }
    }

    pub fn push_gui_container(
        &mut self,
        container: Box<dyn GUIContainer>,
    ) -> Option<GUIContainerSlotkey> {
        let key = self.gui_contianer_collection.push(container);
        match key {
            Some(key) => Some(GUIContainerSlotkey(key)),
            None => None,
        }
    }

    pub fn handle_event(&mut self, event: &mut UIEvent, public_data: &PublicData) {
        self.control_state.on_gui_start();
        if let UIEvent::MouseMove { corrected, .. } = event {
            self.control_state.last_cursor_position = corrected.data;
        }

        let mut extra_elements = Vec::<(Box<dyn FnOnce(&mut GUIRects)->()>, u32)>::with_capacity(100);
        for window_key in &self.window_order {
            match self.window_collection.get_value_mut(&window_key.0) {
                Some(window_mut) => {
                    let mut tab_handle_stack = Vec::<TabLayoutInfo>::new();
                    let mut layout_handle_stack = Vec::<LayoutOrTabInfo>::new();

                    layout_handle_stack.push(window_mut.handle_event(
                        event,
                        public_data,
                        &mut self.control_state,
                    ));
                    loop {
                        match layout_handle_stack.pop() {
                            Some(layout_handle) => match layout_handle.key {
                                LayoutOrTabKey::TabKey(tab_key) => {
                                    tab_handle_stack.push(TabLayoutInfo {
                                        key: tab_key,
                                        container_info: layout_handle.container_info,
                                    });
                                }
                                LayoutOrTabKey::LayoutKey(layout_key) => {
                                    let children = self
                                        .layout_elements_collection
                                        .get_value_mut(&layout_key)
                                        .unwrap()
                                        .handle_event(
                                            event,
                                            layout_handle.container_info,
                                            &mut self.control_state,
                                            
                                        );
                                    layout_handle_stack.extend(children);
                                }
                            },
                            None => break,
                        }
                    }
                    assert_eq!(
                        layout_handle_stack.len(),
                        0,
                        "Layout handle stack should be empty"
                    );
                    for tab in tab_handle_stack.drain(..) {
                        let tab_container = self
                            .tabs_container_collection
                            .get_value_mut(&tab.key)
                            .unwrap();

                        let gui_container_info = tab_container.handle_event(
                            event,
                            public_data,
                            tab.container_info,
                            &self.gui_contianer_collection,
                            &mut self.control_state,
                        );

                        let gui_container = self
                            .gui_contianer_collection
                            .get_value_mut(&gui_container_info.key)
                            .unwrap();
                        gui_container.handle_event(
                            event,
                            public_data,
                            gui_container_info.container_info,
                            &mut self.control_state,
                        );
                    }
                }
                None => { /* No op */ }
            }
        }
        self.control_state.on_gui_end();

        if let UIEvent::CursorExit = event {
            self.control_state.on_cursor_exit();
        }

        if let UIEvent::Update = event {
            self.control_state.on_frame_end();
        }
    }
}
