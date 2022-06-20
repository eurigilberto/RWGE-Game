use rwge::{
    color::{HSLA, RGBA},
    glam::{dvec2, ivec2, vec2, DVec2, Vec2},
    gui::rect_ui::{element::builder::ElementBuilder, event::UIEvent, RectMask},
    uuid::Uuid,
    winit::dpi::{LogicalPosition, PhysicalPosition},
};

use crate::{
    gui_system::{
        control::{ControlId, ControlState, State},
        ContainerInfo,
    },
    public_data::{
        utils::{get_engine_data, get_window},
        PublicData,
    },
};

use super::{LayoutOrTabInfo, LayoutOrTabKey, LayoutSlotKey};

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
        self.start_mouse_position = self.current_mouse_position;
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

fn main_window_top_bar(
    position: Vec2,
    size: Vec2,
    event: &mut UIEvent,
    public_data: &PublicData,
    control_state: &mut ControlState,
    active_id: &mut Option<Uuid>,
    drag_element: &mut DragElement,
) {
    let control_id = control_state.get_id();

    let get_control_id = || {
        if active_id.is_some() {
            ControlId::Active(active_id.unwrap())
        } else {
            ControlId::Control(control_id)
        }
    };

    match event {
        UIEvent::Render { gui_rects } => {
            let state = control_state.get_control_state(get_control_id());
            let color: RGBA = if let State::Active | State::Hovered = state {
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

            ElementBuilder::new(get_engine_data(public_data).screen_size, position, size)
                .set_color(color.into())
                .build(gui_rects);
        }
        UIEvent::Update => {
            control_state.update_hot_hovered(control_id, &RectMask { position, size });
            match control_state.get_control_state(get_control_id()) {
                State::Active => {
                    let window_pos = drag_element.compute_element_position();
                    public_data.push_mut(Box::new(move |public_data| {
                        let window = public_data
                            .get_mut::<rwge::winit::window::Window>()
                            .unwrap();

                        let new_position = PhysicalPosition::new(window_pos.x, window_pos.y);

                        window.set_outer_position(new_position);
                    }));
                    control_state.hold_active_state(active_id.unwrap());
                }
                _ => { /* No Op */ }
            }
        }
        UIEvent::MouseMove { raw, .. } => {
            control_state.update_hot_hovered(control_id, &RectMask { position, size });

            let outer_pos = get_window(public_data).outer_position().unwrap();
            let outer_pos = (ivec2(outer_pos.x, outer_pos.y)).as_vec2();
            drag_element.update_position(outer_pos + *raw);
        }
        UIEvent::MouseButton(input) => match (input.data.button, input.data.state) {
            (rwge::winit::event::MouseButton::Left, rwge::winit::event::ElementState::Pressed) => {
                *active_id = control_state.set_active(control_id);
                if let Some(_) = active_id {
                    let outer_pos = public_data
                        .collection
                        .get::<rwge::winit::window::Window>()
                        .unwrap()
                        .outer_position()
                        .unwrap();

                    drag_element.start_dragging(vec2(outer_pos.x as f32, outer_pos.y as f32));
                }
            }
            (rwge::winit::event::MouseButton::Left, rwge::winit::event::ElementState::Released) => {
                if let State::Active = control_state.get_control_state(get_control_id()) {
                    match control_state.remove_active(active_id.unwrap()) {
                        Ok(_) => {
                            drag_element.stop_dragging();
                            *active_id = None;
                        },
                        Err(_) => {
                            *active_id = None;
                        },
                    }
                }
            }
            _ => {}
        },
        _ => {}
    }
}

pub struct Window {
    pub layout_key: LayoutSlotKey,
    pub size: Vec2,
    pub position: Vec2,
    /////
    drag_window: DragElement,
    active_id: Option<Uuid>
}

impl Window {
    pub fn new_with_contianer(layout_key: LayoutSlotKey, size: Vec2, position: Vec2) -> Self {
        Self {
            layout_key: layout_key,
            size,
            position,
            ////
            drag_window: DragElement::new(),
            active_id: None
        }
    }

    pub fn handle_event(
        &mut self,
        event: &mut UIEvent,
        public_data: &PublicData,
        control_state: &mut ControlState,
    ) -> LayoutOrTabInfo {
        let inner_size = self.size - vec2(20.0, 40.0);
        let inner_position = self.position - vec2(0.0, 10.0);

        let menu_bar_pos = self.position + vec2(0.0, self.size.y * 0.5 - 10.0);
        let menu_bar_size = vec2(self.size.x, 20.0);

        main_window_top_bar(menu_bar_pos, menu_bar_size, event, public_data, control_state, &mut self.active_id, &mut self.drag_window);

        match event {
            UIEvent::Resize(screen_size) => {
                self.size = screen_size.as_vec2();
                self.position = self.size * 0.5;
            }
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
