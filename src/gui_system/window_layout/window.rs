use rwge::{
    color::{HSLA, RGBA},
    glam::{dvec2, ivec2, vec2, DVec2, Vec2},
    gui::rect_ui::{element::builder::ElementBuilder, event::UIEvent, RectMask},
    winit::dpi::{LogicalPosition, PhysicalPosition},
};

use crate::{
    gui_system::ContainerInfo,
    public_data::{
        utils::{get_engine_data, get_window},
        PublicData,
    },
};

use super::{LayoutOrTabInfo, LayoutOrTabKey, LayoutSlotKey};

#[derive(Clone, Copy)]
enum WindowState {
    Idle,
    HoverintTopBar,
    PressedTopBar,
}

impl WindowState {
    pub fn new() -> Self {
        Self::Idle
    }

    pub fn mouse_moved(&mut self, mouse_position: Vec2, menu_pos: Vec2, menu_size: Vec2) {
        let is_inside = mouse_inside_menu_bar(mouse_position, menu_pos, menu_size);
        match self {
            WindowState::Idle => {
                if is_inside {
                    *self = WindowState::HoverintTopBar;
                } else {
                    /* No Op */
                }
            }
            WindowState::HoverintTopBar => {
                if is_inside {
                    *self = WindowState::HoverintTopBar;
                } else {
                    *self = WindowState::Idle;
                }
            }
            _ => { /* No Op */ }
        }
    }

    pub fn left_button_pressed(&mut self) {
        match self {
            WindowState::Idle => { /* No Op */ }
            WindowState::HoverintTopBar => *self = WindowState::PressedTopBar,
            WindowState::PressedTopBar => { /* No Op */ }
        }
    }

    pub fn left_button_released(&mut self) {
        match self {
            WindowState::Idle => { /* No Op */ }
            WindowState::HoverintTopBar => { /* No Op */ }
            WindowState::PressedTopBar => *self = WindowState::HoverintTopBar,
        }
    }
}

pub fn mouse_inside_menu_bar(mouse_position: Vec2, menu_pos: Vec2, menu_size: Vec2) -> bool {
    let is_inside = RectMask {
        position: menu_pos,
        size: menu_size,
    }
    .inside_rect(mouse_position);
    is_inside
}

/// All positions should be computed from the same coordinate system
pub struct DragElement {
    start_mouse_position: Vec2,
    start_element_position: Vec2,
    current_mouse_position: Vec2,
    is_draggin: bool,
}

impl DragElement {
    pub fn new() -> Self {
        Self {
            start_mouse_position: Vec2::ZERO,
            start_element_position: Vec2::ZERO,
            current_mouse_position: Vec2::ZERO,
            is_draggin: false,
        }
    }
    pub fn start_dragging(&mut self, element_position: Vec2) {
        self.start_element_position = element_position;
        self.is_draggin = true;
    }
    pub fn stop_dragging(&mut self) {
        self.is_draggin = false;
    }
    pub fn update_position(&mut self, current_position: Vec2) {
        if !self.is_draggin {
            self.start_mouse_position = current_position;
        }
        self.current_mouse_position = current_position;
    }
    pub fn compute_element_position(&self) -> Vec2 {
        let delta_mouse_pos = self.current_mouse_position - self.start_mouse_position;
        self.start_element_position + delta_mouse_pos
    }
}

enum ControlState {
    Inactive,
    Hovered,
    Active,
}

pub struct ControlStateManager {
    current_id: u32,
    hot: u32,
    pub hovered: u32,
    pub active: u32,
    
    pub cursor_position: Vec2
}

impl ControlStateManager {
    pub fn new() -> Self {
        Self {
            current_id: 0,
            hot: 0,
            active: 0,
            hovered: 0,
            cursor_position: Vec2::ZERO
        }
    }

    fn get_id(&mut self) -> u32{
		self.current_id += 1;
		self.current_id
	}

	fn set_hot(&mut self, id: u32){
		if self.active == 0 {
			self.hot = id;
		}
	}

	fn update_hot_hovered(&mut self, id: u32, rect: &RectMask){
		//self.mouse_position
        if rect.inside_rect(self.cursor_position){
			self.set_hot(id);
		}else{
			if self.hovered == id {
				self.hovered = 0;
			}
		}
	}

	fn get_id_state(&self, id: u32)->ControlState{
		if self.active == id {
			ControlState::Active
		}else if self.hovered == id {
			ControlState::Hovered
		}else {
			ControlState::Inactive
		}
	}

    pub fn on_gui_start(&mut self, event: &UIEvent){
		self.current_id = 1;
		self.hot = 0;

        if let UIEvent::Update | UIEvent::Render { .. } = event{
            self.
        }
		self.event_used_on_start = event.used;
	}

	pub fn on_gui_end(&mut self)->GUIState{
		if !self.event_used_on_start {
			if self.hot != 0 {
				self.hovered = self.hot;
			}
	
			if self.active != 0{
				GUIState::Active
			}else if self.hovered != 0 {
				GUIState::Hovered
			}else {
				GUIState::Inactive
			}
		}else{
			self.hovered = 0;
			GUIState::Inactive
		}
	}
}

pub struct WindowTopBar {
    drag_element: DragElement,
}

impl WindowTopBar {
    pub fn new() -> Self {
        Self {
            drag_element: DragElement::new(),
        }
    }

    pub fn handle_event(&mut self, event: &mut UIEvent, public_data: &PublicData) {
        match event {
            UIEvent::Render { gui_rects } => {
                let color: RGBA = if let WindowState::HoverintTopBar | WindowState::PressedTopBar =
                    self.window_state
                {
                    RGBA::GREEN
                } else {
                    HSLA {
                        h: get_engine_data(public_data).time.time * 10.0,
                        s: get_engine_data(public_data).time.sin_time(10.0) * 0.5 + 0.5,
                        l: 0.8,
                        a: 1.0,
                    }
                    .into()
                };

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
            UIEvent::Update => {
                match self.window_state {
                    WindowState::PressedTopBar => {
                        let window_pos = self.draggin_window.compute_element_position();
                        public_data.push_mut(Box::new(move |public_data| {
                            let window = public_data
                                .get_mut::<rwge::winit::window::Window>()
                                .unwrap();

                            let new_position = PhysicalPosition::new(window_pos.x, window_pos.y);

                            window.set_outer_position(new_position);
                        }));
                    }
                    _ => { /* No Op */ }
                }
            }
            UIEvent::MouseMove { corrected, raw } => {
                self.window_state
                    .mouse_moved(corrected.data, menu_bar_pos, menu_bar_size);

                let outer_pos = get_window(public_data).outer_position().unwrap();
                let outer_pos = (ivec2(outer_pos.x, outer_pos.y)).as_vec2();
                self.draggin_window.update_position(outer_pos + *raw);
            }
            UIEvent::MouseButton(input) => match (input.data.button, input.data.state) {
                (
                    rwge::winit::event::MouseButton::Left,
                    rwge::winit::event::ElementState::Pressed,
                ) => {
                    self.window_state.left_button_pressed();
                    if let WindowState::PressedTopBar = self.window_state {
                        let outer_pos = public_data
                            .collection
                            .get::<rwge::winit::window::Window>()
                            .unwrap()
                            .outer_position()
                            .unwrap();
                        self.draggin_window
                            .start_dragging(vec2(outer_pos.x as f32, outer_pos.y as f32));
                    }
                }
                (
                    rwge::winit::event::MouseButton::Left,
                    rwge::winit::event::ElementState::Released,
                ) => {
                    self.window_state.left_button_released();
                    self.draggin_window.stop_dragging();
                }
                _ => {}
            },
            _ => {}
        }
    }
}

pub struct Window {
    pub layout_key: LayoutSlotKey,
    pub size: Vec2,
    pub position: Vec2,
    /////
    window_state: WindowState,
    draggin_window: DragElement,
}

impl Window {
    pub fn new_with_contianer(layout_key: LayoutSlotKey, size: Vec2, position: Vec2) -> Self {
        Self {
            layout_key: layout_key,
            size,
            position,
            ////
            window_state: WindowState::new(),
            draggin_window: DragElement::new(),
        }
    }

    pub fn handle_event(
        &mut self,
        event: &mut UIEvent,
        public_data: &PublicData,
    ) -> LayoutOrTabInfo {
        let inner_size = self.size - vec2(20.0, 40.0);
        let inner_position = self.position - vec2(0.0, 10.0);

        let menu_bar_pos = self.position + vec2(0.0, self.size.y * 0.5 - 10.0);
        let menu_bar_size = vec2(self.size.x, 20.0);

        match event {
            UIEvent::Render { gui_rects } => {
                let color: RGBA = if let WindowState::HoverintTopBar | WindowState::PressedTopBar =
                    self.window_state
                {
                    RGBA::GREEN
                } else {
                    HSLA {
                        h: get_engine_data(public_data).time.time * 10.0,
                        s: get_engine_data(public_data).time.sin_time(10.0) * 0.5 + 0.5,
                        l: 0.8,
                        a: 1.0,
                    }
                    .into()
                };

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
            UIEvent::Update => {
                match self.window_state {
                    WindowState::PressedTopBar => {
                        let window_pos = self.draggin_window.compute_element_position();
                        public_data.push_mut(Box::new(move |public_data| {
                            let window = public_data
                                .get_mut::<rwge::winit::window::Window>()
                                .unwrap();

                            let new_position = PhysicalPosition::new(window_pos.x, window_pos.y);

                            window.set_outer_position(new_position);
                        }));
                    }
                    _ => { /* No Op */ }
                }
            }
            UIEvent::MouseMove { corrected, raw } => {
                self.window_state
                    .mouse_moved(corrected.data, menu_bar_pos, menu_bar_size);

                let outer_pos = get_window(public_data).outer_position().unwrap();
                let outer_pos = (ivec2(outer_pos.x, outer_pos.y)).as_vec2();
                self.draggin_window.update_position(outer_pos + *raw);
            }
            UIEvent::MouseButton(input) => match (input.data.button, input.data.state) {
                (
                    rwge::winit::event::MouseButton::Left,
                    rwge::winit::event::ElementState::Pressed,
                ) => {
                    self.window_state.left_button_pressed();
                    if let WindowState::PressedTopBar = self.window_state {
                        let outer_pos = public_data
                            .collection
                            .get::<rwge::winit::window::Window>()
                            .unwrap()
                            .outer_position()
                            .unwrap();
                        self.draggin_window
                            .start_dragging(vec2(outer_pos.x as f32, outer_pos.y as f32));
                    }
                }
                (
                    rwge::winit::event::MouseButton::Left,
                    rwge::winit::event::ElementState::Released,
                ) => {
                    self.window_state.left_button_released();
                    self.draggin_window.stop_dragging();
                }
                _ => {}
            },
            _ => {}
        }
        LayoutOrTabInfo {
            key: LayoutOrTabKey::LayoutKey(self.layout_key),
            container_info: ContainerInfo {
                position: inner_position,
                size: inner_size,
            },
        }
    }
}
/*
impl Window {
    pub fn new_with_contianer(layout_key: LayoutSlotKey, size: Vec2, position: Vec2) -> Self {
        Self {
            layout_key: layout_key,
            size,
            position,
            ////
            window_state: WindowState::new(),
            draggin_window: DragElement::new(),
        }
    }

    pub fn handle_event(
        &mut self,
        event: &mut UIEvent,
        public_data: &PublicData,
    ) -> LayoutOrTabInfo {
        let inner_size = self.size - vec2(20.0, 40.0);
        let inner_position = self.position - vec2(0.0, 10.0);

        let menu_bar_pos = self.position + vec2(0.0, self.size.y * 0.5 - 10.0);
        let menu_bar_size = vec2(self.size.x, 20.0);

        match event {
            UIEvent::Render { gui_rects } => {
                let color: RGBA = if let WindowState::HoverintTopBar | WindowState::PressedTopBar =
                    self.window_state
                {
                    RGBA::GREEN
                } else {
                    HSLA {
                        h: get_engine_data(public_data).time.time * 10.0,
                        s: get_engine_data(public_data).time.sin_time(10.0) * 0.5 + 0.5,
                        l: 0.8,
                        a: 1.0,
                    }
                    .into()
                };

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
            UIEvent::Update => {
                match self.window_state {
                    WindowState::PressedTopBar => {
                        let window_pos = self.draggin_window.compute_element_position();
                        public_data.push_mut(Box::new(move |public_data| {
                            let window = public_data
                                .get_mut::<rwge::winit::window::Window>()
                                .unwrap();

                            let new_position = PhysicalPosition::new(window_pos.x, window_pos.y);

                            window.set_outer_position(new_position);
                        }));
                    }
                    _ => { /* No Op */ }
                }
            }
            UIEvent::MouseMove { corrected, raw } => {
                self.window_state
                    .mouse_moved(corrected.data, menu_bar_pos, menu_bar_size);

                let outer_pos = get_window(public_data).outer_position().unwrap();
                let outer_pos = (ivec2(outer_pos.x, outer_pos.y)).as_vec2();
                self.draggin_window.update_position(outer_pos + *raw);
            }
            UIEvent::MouseButton(input) => match (input.data.button, input.data.state) {
                (
                    rwge::winit::event::MouseButton::Left,
                    rwge::winit::event::ElementState::Pressed,
                ) => {
                    self.window_state.left_button_pressed();
                    if let WindowState::PressedTopBar = self.window_state {
                        let outer_pos = public_data
                            .collection
                            .get::<rwge::winit::window::Window>()
                            .unwrap()
                            .outer_position()
                            .unwrap();
                        self.draggin_window
                            .start_dragging(vec2(outer_pos.x as f32, outer_pos.y as f32));
                    }
                }
                (
                    rwge::winit::event::MouseButton::Left,
                    rwge::winit::event::ElementState::Released,
                ) => {
                    self.window_state.left_button_released();
                    self.draggin_window.stop_dragging();
                }
                _ => {}
            },
            _ => {}
        }
        LayoutOrTabInfo {
            key: LayoutOrTabKey::LayoutKey(self.layout_key),
            container_info: ContainerInfo {
                position: inner_position,
                size: inner_size,
            },
        }
    }
}
 */