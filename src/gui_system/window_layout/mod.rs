use std::ops::Deref;
mod layout;
use layout::LayoutElement;
mod tabs_container;
use tabs_container::TabsContainer;
mod window;
use window::UIWindow;

//For now the style of the tabs is going to be fixed
use rwge::{
    glam::Vec2,
    gui::{
        self,
        rect_ui::{
            element::builder::ElementBuilder,
            event::{ExtraRenderSteps, UIEvent},
            GUIRects,
        },
    },
    slotmap::slotmap::{SlotKey, Slotmap},
};

use crate::runtime_data::RuntimeData;

pub use tabs_container::{GUI_ACTIVE_COLOR, GUI_HOVER_COLOR, GUI_INACTIVE_COLOR};

pub use self::layout::DividedElement;

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

impl Into<LayoutOrTabKey> for TabsSlotKey {
    fn into(self) -> LayoutOrTabKey {
        LayoutOrTabKey::TabKey(self)
    }
}

impl Into<LayoutOrTabKey> for LayoutSlotKey {
    fn into(self) -> LayoutOrTabKey {
        LayoutOrTabKey::LayoutKey(self)
    }
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
    gui_container_slotmap: Slotmap<Box<dyn GUIContainer>>,
    tabs_slotmap: Slotmap<TabsContainer>,
    layout_slotmap: Slotmap<LayoutElement>,
    window_collection: Slotmap<UIWindow>,
    window_order: Vec<WindowSlotKey>,
    pub control_state: ControlState,
}

pub const DEPTH_SLICE_SIZE: u32 = 15;

pub mod depth_offset {
    pub const TAB_SHADOW: u32 = 5;
    pub const RESIZE_CONTROL: u32 = 10;
    pub const DIVIDER: u32 = 2;
    pub const SELECT_COUNT: u32 = 6;
    pub const FONT_ANIM_OFFSET: u32 = 8;
}

impl WindowSystem {
    pub fn new() -> Self {
        Self {
            //gui instance
            gui_container_slotmap: Slotmap::<Box<dyn GUIContainer>>::with_capacity(20),
            //layout
            tabs_slotmap: Slotmap::<TabsContainer>::with_capacity(10),
            layout_slotmap: Slotmap::<LayoutElement>::with_capacity(20),
            //windowing
            window_collection: Slotmap::<UIWindow>::with_capacity(5),
            window_order: Vec::<WindowSlotKey>::with_capacity(5),
            //control
            control_state: ControlState::new(),
        }
    }

    pub fn create_tab(&mut self, gui_container: Vec<GUIContainerSlotkey>) -> TabsSlotKey {
        let t_container = TabsContainer::new(gui_container);
        let t_container_key = self.tabs_slotmap.push(t_container).unwrap();
        TabsSlotKey(t_container_key)
    }

    pub fn create_window(
        &mut self,
        layout_key: LayoutSlotKey,
        size: Vec2,
        position: Vec2,
    ) -> WindowSlotKey {
        let window_layout = UIWindow::new_with_contianer(layout_key, size, position);
        let window_key = WindowSlotKey(self.window_collection.push(window_layout).unwrap());
        self.window_order.push(window_key);
        window_key
    }

    pub fn push_vertical(&mut self, children: Vec<DividedElement>) -> Option<LayoutSlotKey> {
        LayoutElement::create_vertical(children, &mut self.layout_slotmap)
    }

    pub fn push_horizontal(&mut self, children: Vec<DividedElement>) -> Option<LayoutSlotKey> {
        LayoutElement::create_horizontal(children, &mut self.layout_slotmap)
    }

    pub fn push_gui_container(
        &mut self,
        container: Box<dyn GUIContainer>,
    ) -> Option<GUIContainerSlotkey> {
        let key = self.gui_container_slotmap.push(container);
        match key {
            Some(key) => Some(GUIContainerSlotkey(key)),
            None => None,
        }
    }

    pub fn layouts_handle_event(
        control_state: &mut ControlState,
        layout_slotmap: &mut Slotmap<LayoutElement>,
        root_layout: LayoutOrTabInfo,
        event: &mut UIEvent,
        public_data: &RuntimeData,
        depth_range: (u32, u32),
    ) -> Vec<TabLayoutInfo> {
        let mut tab_handle_stack = Vec::<TabLayoutInfo>::new();
        let mut layout_handle_stack = Vec::<LayoutOrTabInfo>::new();

        layout_handle_stack.push(root_layout);

        //println!("Layout handle push");
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
                        let children = layout_slotmap
                            .get_value_mut(&layout_key)
                            .unwrap()
                            .handle_event(event, layout_handle.container_info, control_state);
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

        tab_handle_stack
    }

    pub fn tabs_handle_event(
        control_state: &mut ControlState,
        tabs_slotmap: &mut Slotmap<TabsContainer>,
        gui_container_slotmap: &Slotmap<Box<dyn GUIContainer>>,
        mut tab_handle_stack: Vec<TabLayoutInfo>,
        event: &mut UIEvent,
        public_data: &RuntimeData,
    ) -> Vec<GUIContainerInfo> {
        let mut gui_handle_stack = Vec::with_capacity(tab_handle_stack.len());
        for (index, tab) in tab_handle_stack.drain(..).enumerate() {
            let tab_container = tabs_slotmap.get_value_mut(&tab.key).unwrap();

            let tab_names: Vec<&str> = tab_container
                .tabs
                .iter()
                .map(|gui_key| gui_container_slotmap.get_value(gui_key).unwrap().get_name())
                .collect();

            let gui_container_info = tab_container.handle_event(
                event,
                public_data,
                tab.container_info,
                control_state,
                &tab_names,
            );
            gui_handle_stack.push(gui_container_info);
        }
        gui_handle_stack
    }

    pub fn windows_handle_event(&mut self, event: &mut UIEvent, public_data: &RuntimeData) {
        for (index, window_key) in self.window_order.iter().enumerate() {
            match self.window_collection.get_value_mut(&window_key.0) {
                Some(window_mut) => {
                    let depth_range = (
                        index as u32 * DEPTH_SLICE_SIZE,
                        ((index as u32 + 1) * DEPTH_SLICE_SIZE) - 1,
                    );

                    let root_layout = window_mut.handle_event(
                        event,
                        public_data,
                        &mut self.control_state,
                        depth_range,
                    );

                    let tab_handle_stack = WindowSystem::layouts_handle_event(
                        &mut self.control_state,
                        &mut self.layout_slotmap,
                        root_layout,
                        event,
                        public_data,
                        depth_range,
                    );

                    let gui_handle_stack = WindowSystem::tabs_handle_event(
                        &mut self.control_state,
                        &mut self.tabs_slotmap,
                        &self.gui_container_slotmap,
                        tab_handle_stack,
                        event,
                        public_data,
                    );

                    for gui_handle in gui_handle_stack {
                        let gui_container = self
                            .gui_container_slotmap
                            .get_value_mut(&gui_handle.key)
                            .unwrap();
                        gui_container.handle_event(
                            event,
                            public_data,
                            gui_handle.container_info,
                            &mut self.control_state,
                        );
                    }
                }
                None => { /* No op */ }
            }

            if let UIEvent::Render {
                gui_rects,
                extra_render_steps,
            } = event
            {
                extra_render_steps.execute_render_steps(gui_rects);
            }
        }
    }

    pub fn handle_event(&mut self, event: &mut UIEvent, public_data: &RuntimeData) {
        self.control_state.on_gui_start();
        if let UIEvent::MouseMove { corrected, .. } = event {
            self.control_state.last_cursor_position = Some(*corrected);
        }

        self.windows_handle_event(event, public_data);

        self.control_state.on_gui_end();

        if let UIEvent::CursorExit = event {
            self.control_state.on_cursor_exit();
        }

        if let UIEvent::Update = event {
            self.control_state.on_after_update();
        }
    }

    pub fn render_event(&mut self, public_data: &RuntimeData, gui_rects: &mut GUIRects) {
        let extra_render_steps = ExtraRenderSteps::new(25);
        let mut event = UIEvent::Render {
            gui_rects,
            extra_render_steps,
        };
        self.control_state.on_gui_start();
        self.windows_handle_event(&mut event, public_data);
        self.control_state.on_gui_end();
    }
}
