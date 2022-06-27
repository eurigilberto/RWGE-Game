use rwge::{
    color::{HSLA, RGBA},
    glam::{dvec2, ivec2, vec2, DVec2, Vec2},
    gui::rect_ui::{
        element::builder::ElementBuilder,
        event::{MouseInput, UIEvent},
        Rect,
    },
    math_utils::lerp_f32,
    uuid::Uuid,
    winit::{
        dpi::{LogicalPosition, PhysicalPosition, PhysicalSize},
        event::{ElementState, MouseButton},
        platform::windows::WindowExtWindows,
    },
};

use crate::{
    gui_system::{
        control::{drag_element::DragElement, main_window_top_bar, ControlId, ControlState, State},
        ContainerInfo,
    },
    public_data::{
        utils::{get_engine_data, get_window},
        PublicData,
    },
};

use super::{LayoutOrTabInfo, LayoutOrTabKey, LayoutSlotKey, RESIZE_CONTROL_DEPTH_OFFSET};

pub struct ResizeDrag {
    active_id: Uuid,
    new_size: Vec2,
    get_count: u32,
    changed: bool,
}

impl ResizeDrag {
    pub fn new(id: Uuid, size: Vec2) -> Self {
        Self {
            active_id: id,
            new_size: size,
            get_count: 0,
            changed: false,
        }
    }

    pub fn update_counter(&mut self) {
        self.get_count += 1;
        self.get_count %= 2;
    }

    pub fn update_size(&mut self, size: Vec2) {
        self.new_size = size;
        self.changed = true;
    }

    pub fn get_size(&mut self) -> Option<Vec2> {
        if self.changed {
            self.update_counter();
            if self.get_count == 0 {
                Some(self.new_size)
            } else {
                None
            }
        } else {
            None
        }
    }
}

pub struct UIWindow {
    pub layout_key: LayoutSlotKey,
    pub size: Vec2,
    pub position: Vec2,
    /////
    drag_window: DragElement,
    top_bar_active_id: Option<Uuid>,
    resize_drag_active_id: Option<ResizeDrag>,
}

pub fn resize_controls(
    position: Vec2,
    size: Vec2,
    event: &mut UIEvent,
    public_data: &PublicData,
    control_state: &mut ControlState,
    active_id: &mut Option<ResizeDrag>,
    container_info: &ContainerInfo
) {
    let offset_multipliers: [Vec2; 4] = [
        vec2(-1.0, -1.0),
        vec2(1.0, -1.0),
        vec2(1.0, 1.0),
        vec2(-1.0, 1.0),
    ];
    control_state.set_depth_and_save(container_info.depth_range.0 + RESIZE_CONTROL_DEPTH_OFFSET);
    let rect_mask = Rect { position, size };
    /*for mult in offset_multipliers*/
    {
        let mult = offset_multipliers[1];
        let c_position = position + (size * 0.5) * mult;
        let c_size = vec2(60.0, 60.0);

        let control_id = control_state.get_id();

        match event {
            UIEvent::MouseButton(mouse_input) => {
                if let MouseInput {
                    button: MouseButton::Left,
                    state: ElementState::Pressed,
                } = mouse_input
                {
                    let id = control_state.set_active(control_id);
                    if let Some(id) = id {
                        *active_id = Some(ResizeDrag::new(
                            id,
                            get_engine_data(public_data).screen_size.as_vec2(),
                        ));
                    }
                }

                if let MouseInput {
                    button: MouseButton::Left,
                    state: ElementState::Released,
                } = mouse_input
                {
                    *active_id = None;
                }
            }
            UIEvent::MouseMove { raw, .. } => {
                if let Some(drag_resize) = active_id {
                    let new_size = (*raw).max(vec2(100.0, 100.0));
                    drag_resize.update_size(new_size);
                }
            }
            UIEvent::Update => {
                if active_id.is_some() {
                    control_state.hold_active_state(active_id.as_ref().unwrap().active_id);

                    if let Some(new_size) = active_id.as_mut().unwrap().get_size() {
                        let new_size = new_size;
                        public_data.push_mut(Box::new(move |public_data| {
                            let wind = public_data
                                .get_mut::<rwge::winit::window::Window>()
                                .unwrap();
                            wind.set_inner_size(PhysicalSize::new(
                                new_size.x as i32,
                                new_size.y as i32,
                            ));
                        }));
                    }
                } else {
                    if let Some(cursor_pos) = control_state.last_cursor_position {
                        if (cursor_pos - c_position).length() <= 30.0 {
                            control_state.set_hot(control_id);
                        }
                    }
                }
            }
            UIEvent::Render {
                gui_rects,
                extra_render_steps,
                ..
            } => {
                let state = control_state.get_control_state(ControlId::Control(control_id));
                match state {
                    State::Hovered => extra_render_steps.push(
                        Box::new(move |gui_rects| {
                            ElementBuilder::new(c_position, c_size)
                                .set_circle()
                                .set_rect_mask(rect_mask.into())
                                .set_color(RGBA::RED.into())
                                .build(gui_rects);
                        }),
                        container_info.depth_range.0 + RESIZE_CONTROL_DEPTH_OFFSET,
                    ),
                    State::Active => extra_render_steps.push(
                        Box::new(move |gui_rects| {
                            ElementBuilder::new(c_position, c_size)
                                .set_circle()
                                .set_rect_mask(rect_mask.into())
                                .set_color(RGBA::GREEN.into())
                                .build(gui_rects);
                        }),
                        container_info.depth_range.0 + RESIZE_CONTROL_DEPTH_OFFSET,
                    ),
                    State::Inactive => {}
                }
            }
            _ => {}
        }
    }
    control_state.restore_depth();
}

impl UIWindow {
    pub fn new_with_contianer(layout_key: LayoutSlotKey, size: Vec2, position: Vec2) -> Self {
        Self {
            layout_key: layout_key,
            size,
            position,
            ////
            drag_window: DragElement::new(),
            top_bar_active_id: None,
            resize_drag_active_id: None,
        }
    }

    pub fn handle_event(
        &mut self,
        event: &mut UIEvent,
        public_data: &PublicData,
        control_state: &mut ControlState,
        depth_range: (u32, u32),
    ) -> LayoutOrTabInfo {
        let inner_size = self.size - vec2(10.0, 30.0);
        let inner_position = self.position - vec2(0.0, 10.0);

        let menu_bar_pos = self.position + vec2(0.0, self.size.y * 0.5 - 10.0);
        let menu_bar_size = vec2(self.size.x, 20.0);

        main_window_top_bar::main_window_top_bar(
            menu_bar_pos,
            menu_bar_size,
            event,
            public_data,
            control_state,
            &mut self.top_bar_active_id,
            &mut self.drag_window,
        );

        resize_controls(
            self.position,
            self.size,
            event,
            public_data,
            control_state,
            &mut self.resize_drag_active_id,
            &ContainerInfo { rect: Rect::default(), depth_range}
        );

        match event {
            UIEvent::Resize(screen_size) => {
                self.size = screen_size.as_vec2();
                self.position = self.size * 0.5;
            }
            UIEvent::Update => {}
            _ => {}
        }
        LayoutOrTabInfo {
            key: LayoutOrTabKey::LayoutKey(self.layout_key),
            container_info: ContainerInfo {
                rect: Rect {
                    position: inner_position,
                    size: inner_size,
                },
                depth_range: depth_range
            },
        }
    }
}
