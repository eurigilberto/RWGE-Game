//For now the style of the tabs is going to be fixed
use rwge::{
    color::RGBA,
    glam::{uvec2, UVec2},
    gui::rect_ui::{
        element::{create_new_rect_element, ColoringType, MaskType},
        event::UIEvent,
        ExtraBufferData, RectMask,
    },
    slotmap::slotmap::{SlotKey, Slotmap},
};

use crate::public_data::{self, EngineData, PublicData};

use super::gui_container::GUIContainer;

#[derive(Clone, Copy)]
pub struct GUIContainerSlotkey(pub SlotKey);
#[derive(Clone, Copy)]
pub struct TabsContainerSlotKey(pub SlotKey);
#[derive(Clone, Copy)]
pub struct LayoutElementSlotKey(pub SlotKey);
#[derive(Clone, Copy)]
pub struct WindowSlotKey(pub SlotKey);

pub struct DividedElement {
    pub layout_key: LayoutElementSlotKey,
    pub size: f32,
}

pub enum LayoutElement {
    Single(TabsContainerSlotKey),
    Horizontal(Vec<DividedElement>),
    Vertical(Vec<DividedElement>),
}

impl LayoutElement {
    pub fn validate_children(
        &self,
        children: &Vec<DividedElement>,
        layout_elements: &Slotmap<LayoutElement>,
    ) -> bool {
        for child in children {
            let layout_element = layout_elements.get_value(&child.layout_key.0).unwrap();
            match (self, layout_element) {
                (LayoutElement::Single(_), _) => {
                    panic!("Single cannot have divided element as children")
                }
                (LayoutElement::Horizontal(_), LayoutElement::Horizontal(_)) => return false,
                (LayoutElement::Vertical(_), LayoutElement::Vertical(_)) => return false,
                (_, _) => {}
            }
        }
        return true;
    }

    pub fn push_children(&mut self, new_children: Vec<DividedElement>) {
        match self {
            LayoutElement::Single(_) => {
                panic!("Should not be pushing divided elements to a single")
            }
            LayoutElement::Horizontal(children) => {
                children.reserve(new_children.len());
                children.extend(new_children);
            }
            LayoutElement::Vertical(children) => {
                children.reserve(new_children.len());
                children.extend(new_children);
            }
        }
    }

    pub fn validate_and_create(
        mut layout: LayoutElement,
        new_children: Vec<DividedElement>,
        layout_elements: &mut Slotmap<LayoutElement>,
    ) -> Option<LayoutElementSlotKey> {
        let valid = layout.validate_children(&new_children, &layout_elements);
        if !valid {
            None
        } else {
            layout.push_children(new_children);
            match layout_elements.push(layout) {
                Some(slot_key) => Some(LayoutElementSlotKey(slot_key)),
                None => None,
            }
        }
    }

    pub fn create_horizontal(
        new_children: Vec<DividedElement>,
        layout_elements: &mut Slotmap<LayoutElement>,
    ) -> Option<LayoutElementSlotKey> {
        let horizontal_layout = LayoutElement::Horizontal(Vec::<DividedElement>::new());
        Self::validate_and_create(horizontal_layout, new_children, layout_elements)
    }

    pub fn create_vertical(
        new_children: Vec<DividedElement>,
        layout_elements: &mut Slotmap<LayoutElement>,
    ) -> Option<LayoutElementSlotKey> {
        let vertical_layout = LayoutElement::Vertical(Vec::<DividedElement>::new());
        Self::validate_and_create(vertical_layout, new_children, layout_elements)
    }

    pub fn handle_event(
        &mut self,
        event: &mut UIEvent,
        size: UVec2,
        position: UVec2,
    ) -> Vec<LayoutInfo> {
        match self {
            LayoutElement::Single(tab_container) => {
                // How to render the tab?
                Vec::<LayoutInfo>::new()
            }
            LayoutElement::Horizontal(children) => {
                Vec::<LayoutInfo>::new()
            }
            LayoutElement::Vertical(children) =>{
                Vec::<LayoutInfo>::new()
            },
        }
    }
}

pub struct TabsContainer {
    tabs: Vec<GUIContainerSlotkey>,
    active_tab: usize,
}

impl TabsContainer {
    pub fn new(mut containers: Vec<GUIContainerSlotkey>) -> Self {
        if containers.len() == 0 {
            panic!("Cannot create a tab contianer with no guicontainers")
        };

        let mut tabs = Vec::<GUIContainerSlotkey>::new();
        tabs.extend(containers.drain(..));

        Self {
            tabs,
            active_tab: 0,
        }
    }
    pub fn handle_event(
        &mut self,
        event: &mut UIEvent,
        public_data: &PublicData,
        public_data_changes: &Option<&mut Vec<Box<dyn FnMut(&mut PublicData) -> ()>>>,
        size: UVec2,
        position: UVec2,
        gui_container_collection: &Slotmap<Box<dyn GUIContainer>>,
    ) -> GUIContainerInfo {
        let active_tab_key = self.tabs[self.active_tab];
        let offset = ((size.y as f32) * 0.75) as u32;
        let container_position = uvec2(position.x, position.y - (offset as f32 * 0.5) as u32);
        let container_size = uvec2(size.x, size.y - offset);

        let tab_menu_size = uvec2(size.x, offset - ((size.y as f32) * 0.05) as u32);
        let tab_menu_position = uvec2(
            position.x,
            position.y + ((size.y - offset) as f32 * 0.5) as u32,
        );

        match event {
            UIEvent::Render { gui_rects } => {
                let mask_type = MaskType::Rect { border: None };
                let coloring_type = ColoringType::Color(ExtraBufferData::NewData(RGBA::RED));
                let rect_mask = RectMask {
                    position: tab_menu_position,
                    size: tab_menu_size,
                };
                create_new_rect_element(
                    gui_rects,
                    public_data
                        .collection
                        .get::<EngineData>()
                        .unwrap()
                        .screen_size,
                    tab_menu_position,
                    tab_menu_size,
                    0.0,
                    ExtraBufferData::NewData(rect_mask),
                    &mask_type,
                    &coloring_type,
                );

                GUIContainerInfo {
                    key: active_tab_key,
                    position: container_position,
                    size: container_size,
                }
            }
            _ => GUIContainerInfo {
                key: active_tab_key,
                position: container_position,
                size: container_size,
            },
        }
    }
}

pub struct WindowLayout {
    pub layout_key: LayoutElementSlotKey,
    pub size: UVec2,
    pub position: UVec2,
}

pub struct TabLayoutInfo {
    key: TabsContainerSlotKey,
    size: UVec2,
    position: UVec2,
}
pub struct LayoutInfo {
    key: LayoutElementSlotKey,
    size: UVec2,
    position: UVec2,
}
pub struct GUIContainerInfo {
    key: GUIContainerSlotkey,
    size: UVec2,
    position: UVec2,
}

impl WindowLayout {
    pub fn new_with_contianer(
        layout_key: LayoutElementSlotKey,
        size: UVec2,
        position: UVec2,
    ) -> Self {
        Self {
            layout_key: layout_key,
            size,
            position,
        }
    }

    pub fn handle_event(&mut self, event: &mut UIEvent, public_data: &PublicData) -> LayoutInfo {
        match event {
            UIEvent::Render { gui_rects } => LayoutInfo {
                key: self.layout_key,
                size: (self.size.as_vec2() * 0.95).as_uvec2(),
                position: self.position.clone(),
            },
            _ => LayoutInfo {
                key: self.layout_key,
                size: (self.size.as_vec2() * 0.95).as_uvec2(),
                position: self.position.clone(),
            },
        }
    }
}

pub struct WindowLayouting {
    gui_contianer_collection: Slotmap<Box<dyn GUIContainer>>,
    tabs_container_collection: Slotmap<TabsContainer>,
    layout_elements_collection: Slotmap<LayoutElement>,
    window_collection: Slotmap<WindowLayout>,
    window_order: Vec<WindowSlotKey>,
}

fn from_layout_element_vec_into_divided_element_vec(
    layout_elements: Vec<LayoutElementSlotKey>,
) -> Vec<DividedElement> {
    let mut div_element_vec = Vec::<DividedElement>::with_capacity(layout_elements.len());
    for element in layout_elements {
        div_element_vec.push(DividedElement {
            layout_key: element,
            size: 1.0,
        });
    }
    div_element_vec
}

impl WindowLayouting {
    pub fn new() -> Self {
        Self {
            gui_contianer_collection: Slotmap::<Box<dyn GUIContainer>>::new_with_capacity(20),
            tabs_container_collection: Slotmap::<TabsContainer>::new_with_capacity(10),
            layout_elements_collection: Slotmap::<LayoutElement>::new_with_capacity(10),
            window_collection: Slotmap::<WindowLayout>::new_with_capacity(5),
            window_order: Vec::<WindowSlotKey>::with_capacity(5),
        }
    }

    pub fn create_tab(&mut self, gui_container: Vec<GUIContainerSlotkey>) -> TabsContainerSlotKey {
        let t_container = TabsContainer::new(gui_container);
        let t_container_key = self.tabs_container_collection.push(t_container).unwrap();
        TabsContainerSlotKey(t_container_key)
    }

    pub fn create_window(
        &mut self,
        layout_key: LayoutElementSlotKey,
        size: UVec2,
        position: UVec2,
    ) -> WindowSlotKey {
        let window_layout = WindowLayout::new_with_contianer(layout_key, size, position);
        let window_key = WindowSlotKey(self.window_collection.push(window_layout).unwrap());
        self.window_order.push(window_key);
        window_key
    }

    pub fn create_vertical_layout_element(
        &mut self,
        children: Vec<DividedElement>,
    ) -> Option<LayoutElementSlotKey> {
        LayoutElement::create_vertical(children, &mut self.layout_elements_collection)
    }

    pub fn create_horizontal_layout_element(
        &mut self,
        children: Vec<DividedElement>,
    ) -> Option<LayoutElementSlotKey> {
        LayoutElement::create_horizontal(children, &mut self.layout_elements_collection)
    }

    pub fn create_single_layout_element(
        &mut self,
        tab_key: TabsContainerSlotKey,
    ) -> Option<LayoutElementSlotKey> {
        match self
            .layout_elements_collection
            .push(LayoutElement::Single(tab_key))
        {
            Some(key) => Some(LayoutElementSlotKey(key)),
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

    pub fn handle_event(
        &mut self,
        event: &mut UIEvent,
        public_data_changes: &Option<&mut Vec<Box<dyn FnMut(&mut PublicData) -> ()>>>,
        public_data: &PublicData,
    ) {
        for window_key in &self.window_order {
            match self.window_collection.get_value_mut(&window_key.0) {
                Some(window_mut) => {
                    let mut tab_handle_stack = Vec::<TabLayoutInfo>::new();
                    let mut layout_handle_stack = Vec::<LayoutInfo>::new();
                    layout_handle_stack.push(window_mut.handle_event(event, public_data));
                    loop {
                        match layout_handle_stack.pop() {
                            Some(layout_handle) => {
                                let layout_element = self
                                    .layout_elements_collection
                                    .get_value_mut(&layout_handle.key.0)
                                    .unwrap();
                                match layout_element {
                                    LayoutElement::Single(tab) => {
                                        tab_handle_stack.push(TabLayoutInfo {
                                            key: tab.clone(),
                                            size: layout_handle.size,
                                            position: layout_handle.position,
                                        });
                                    }
                                    _ => {
                                        let layout_handlers = layout_element.handle_event(
                                            event,
                                            layout_handle.size,
                                            layout_handle.position,
                                        );
                                        layout_handle_stack.extend(layout_handlers);
                                    }
                                }
                            }
                            None => break,
                        }
                    }
                    assert_eq!(
                        layout_handle_stack.len(),
                        0,
                        "Layout handle stack should be empty"
                    );
                    /*let mut gui_container_stack =
                    Vec::<GUIContainerInfo>::with_capacity(tab_handle_stack.len());*/
                    for tab in tab_handle_stack.drain(..) {
                        let tab_container = self
                            .tabs_container_collection
                            .get_value_mut(&tab.key.0)
                            .unwrap();

                        let gui_container_info = tab_container.handle_event(
                            event,
                            public_data,
                            public_data_changes,
                            tab.size,
                            tab.position,
                            &self.gui_contianer_collection,
                        );

                        let gui_container = self
                            .gui_contianer_collection
                            .get_value_mut(&gui_container_info.key.0)
                            .unwrap();
                        gui_container.handle_event(
                            event,
                            public_data_changes,
                            public_data,
                            gui_container_info.size,
                            gui_container_info.position,
                        );
                    }
                }
                None => {
                    // Window does not exist you need to
                }
            }
        }
    }
}
