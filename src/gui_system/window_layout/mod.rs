use std::ops::Deref;

//For now the style of the tabs is going to be fixed
use rwge::{
    color::{HSLA, RGBA},
    glam::{uvec2, vec2, UVec2, Vec2},
    gui::rect_ui::{
        element::{builder::ElementBuilder, create_new_rect_element, ColoringType, MaskType},
        event::UIEvent,
        ExtraBufferData, RectMask,
    },
    slotmap::slotmap::{SlotKey, Slotmap}, math_utils::lerp_f32,
};

use crate::public_data::{self, utils::get_engine_data, EngineData, PublicData};

use super::gui_container::GUIContainer;

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

pub enum LayoutElement {
    Single(TabsSlotKey),
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
        if children.len() == 1 {
            return false;
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
    ) -> Option<LayoutSlotKey> {
        let valid = layout.validate_children(&new_children, &layout_elements);
        if !valid {
            None
        } else {
            layout.push_children(new_children);
            match layout_elements.push(layout) {
                Some(slot_key) => Some(LayoutSlotKey(slot_key)),
                None => None,
            }
        }
    }

    pub fn create_horizontal(
        new_children: Vec<DividedElement>,
        layout_elements: &mut Slotmap<LayoutElement>,
    ) -> Option<LayoutSlotKey> {
        let horizontal_layout = LayoutElement::Horizontal(Vec::<DividedElement>::new());
        Self::validate_and_create(horizontal_layout, new_children, layout_elements)
    }

    pub fn create_vertical(
        new_children: Vec<DividedElement>,
        layout_elements: &mut Slotmap<LayoutElement>,
    ) -> Option<LayoutSlotKey> {
        let vertical_layout = LayoutElement::Vertical(Vec::<DividedElement>::new());
        Self::validate_and_create(vertical_layout, new_children, layout_elements)
    }

    pub fn handle_event(
        &mut self,
        event: &mut UIEvent,
        size: Vec2,
        position: Vec2,
    ) -> Vec<LayoutOrTabInfo> {
        match self {
            LayoutElement::Single(tab_container) => {
                // How to render the tab?
                let mut tab = Vec::<LayoutOrTabInfo>::new();
                tab.push(LayoutOrTabInfo {
                    key: LayoutOrTabKey::TabKey(tab_container.clone()),
                    size,
                    position,
                });
                tab
            }
            LayoutElement::Horizontal(children) => {
                let margin: f32 = 10.0;
                let inner_size = size;
                let start_pos = position.x - inner_size.x * 0.5;

                let children_sizes =
                    compute_children_sizes(children, start_pos, inner_size.x, 10.0, Sign::Positive);
                children_sizes
                    .iter()
                    .map(|(key, size, pos)| LayoutOrTabInfo {
                        key: key.clone(),
                        size: vec2(*size, inner_size.y),
                        position: vec2(*pos, position.y),
                    })
                    .collect()
            }
            LayoutElement::Vertical(children) => {
                let margin: f32 = 10.0;
                let inner_size = size;
                let start_pos = position.y + inner_size.y * 0.5;

                let children_sizes =
                    compute_children_sizes(children, start_pos, inner_size.y, 10.0, Sign::Negative);
                children_sizes
                    .iter()
                    .map(|(key, size, pos)| LayoutOrTabInfo {
                        key: key.clone(),
                        size: vec2(inner_size.x, *size),
                        position: vec2(position.x, *pos),
                    })
                    .collect()
            }
        }
    }
}

enum Sign {
    Positive,
    Negative,
}

impl Sign {
    pub fn as_f32(&self) -> f32 {
        match self {
            Sign::Positive => 1.0,
            Sign::Negative => -1.0,
        }
    }
}

fn compute_children_sizes(
    children: &Vec<DividedElement>,
    start_pos: f32,
    inner_size: f32,
    margin: f32,
    sign: Sign,
) -> Vec<(LayoutOrTabKey, f32, f32)> {
    let mut children_sizes = Vec::<(LayoutOrTabKey, f32, f32)>::new();
    let total_size = children
        .iter()
        .fold(0.0, |acum: f32, div: &DividedElement| acum + div.size);

    let mut start_pos = start_pos;
    let gap_count = (children.len() as i32 - 1) as f32;
    let children_available_size = inner_size - (margin * gap_count);

    for child in children {
        let element_proportion = child.size / total_size;
        let child_size = children_available_size * element_proportion;
        let child_position = start_pos + child_size * 0.5 * sign.as_f32();

        children_sizes.push((
            LayoutOrTabKey::LayoutKey(child.layout_key),
            child_size,
            child_position,
        ));
        start_pos += (child_size + margin) * sign.as_f32();
    }
    children_sizes
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
        size: Vec2,
        position: Vec2,
        gui_container_collection: &Slotmap<Box<dyn GUIContainer>>,
    ) -> GUIContainerInfo {
        const TAB_SIZE: f32 = 30.0;

        let active_tab_key = self.tabs[self.active_tab];

        let container_position = vec2(position.x, position.y - (TAB_SIZE * 0.5));
        let container_size = vec2(size.x, size.y - TAB_SIZE);

        let tab_menu_size = vec2(size.x, TAB_SIZE);
        let tab_menu_position = vec2(position.x, position.y + (size.y - TAB_SIZE) * 0.5);

        match event {
            UIEvent::Render { gui_rects } => {
                let engine_data = public_data::utils::get_engine_data(public_data);
                let screen_size = engine_data.screen_size;
                let hue = lerp_f32(0.0, 360.0, engine_data.time.sin_time(0.25) * 0.5 + 0.5);
                let color: RGBA = HSLA {
                    h: hue,
                    s: 0.75,
                    l: 0.5,
                    a: 1.0,
                }
                .into();

                ElementBuilder::new(screen_size, tab_menu_position, tab_menu_size)
                    .set_color(color.into())
                    .build(gui_rects);

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

pub struct TabLayoutInfo {
    key: TabsSlotKey,
    size: Vec2,
    position: Vec2,
}
#[derive(Clone, Copy)]
pub enum LayoutOrTabKey {
    TabKey(TabsSlotKey),
    LayoutKey(LayoutSlotKey),
}
pub struct LayoutOrTabInfo {
    key: LayoutOrTabKey,
    size: Vec2,
    position: Vec2,
}
pub struct GUIContainerInfo {
    key: GUIContainerSlotkey,
    size: Vec2,
    position: Vec2,
}

pub struct WindowLayout {
    pub layout_key: LayoutSlotKey,
    pub size: Vec2,
    pub position: Vec2,
}

impl WindowLayout {
    pub fn new_with_contianer(layout_key: LayoutSlotKey, size: Vec2, position: Vec2) -> Self {
        Self {
            layout_key: layout_key,
            size,
            position,
        }
    }

    pub fn handle_event(
        &mut self,
        event: &mut UIEvent,
        public_data: &PublicData,
    ) -> LayoutOrTabInfo {
        let inner_size = self.size - vec2(20.0, 40.0);
        let inner_position = self.position - vec2(0.0, 10.0);

        match event {
            UIEvent::Render { gui_rects } => {
                let menu_bar_pos = self.position + vec2(0.0, self.size.y * 0.5 - 10.0);
                let menu_bar_size = vec2(self.size.x, 20.0);

                let color: RGBA = HSLA {
                    h: get_engine_data(public_data).time.time * 10.0,
                    s: get_engine_data(public_data).time.sin_time(10.0) * 0.5 + 0.5,
                    l: 0.8,
                    a: 1.0,
                }
                .into();

                ElementBuilder::new(
                    get_engine_data(public_data).screen_size,
                    menu_bar_pos,
                    menu_bar_size,
                )
                .set_color(color.into())
                .build(gui_rects);
            }
            UIEvent::Resize(screen_size) => {
                self.size = screen_size.as_vec2();
                self.position = self.size * 0.5;
            }
            _ => {}
        }
        LayoutOrTabInfo {
            key: LayoutOrTabKey::LayoutKey(self.layout_key),
            size: inner_size,
            position: inner_position,
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
        let window_layout = WindowLayout::new_with_contianer(layout_key, size, position);
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

    pub fn handle_event(
        &mut self,
        event: &mut UIEvent,
        public_data_changes: &Option<&mut Vec<Box<dyn FnMut(&mut PublicData) -> ()>>>,
        public_data: &PublicData,
    ) {
        for window_key in &self.window_order {
            match self.window_collection.get_value_mut(&window_key.0) {
                Some(window_mut) => {
                    // Should only contain tabs!
                    let mut tab_handle_stack = Vec::<TabLayoutInfo>::new();
                    // Should only contain tabs!
                    let mut layout_handle_stack = Vec::<LayoutOrTabInfo>::new();

                    layout_handle_stack.push(window_mut.handle_event(event, public_data));
                    loop {
                        match layout_handle_stack.pop() {
                            Some(layout_handle) => match layout_handle.key {
                                LayoutOrTabKey::TabKey(tab_key) => {
                                    tab_handle_stack.push(TabLayoutInfo {
                                        key: tab_key,
                                        size: layout_handle.size,
                                        position: layout_handle.position,
                                    });
                                }
                                LayoutOrTabKey::LayoutKey(layout_key) => {
                                    let children = self
                                        .layout_elements_collection
                                        .get_value_mut(&layout_key)
                                        .unwrap()
                                        .handle_event(
                                            event,
                                            layout_handle.size,
                                            layout_handle.position,
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
                    /*let mut gui_container_stack =
                    Vec::<GUIContainerInfo>::with_capacity(tab_handle_stack.len());*/
                    for tab in tab_handle_stack.drain(..) {
                        let tab_container = self
                            .tabs_container_collection
                            .get_value_mut(&tab.key)
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
