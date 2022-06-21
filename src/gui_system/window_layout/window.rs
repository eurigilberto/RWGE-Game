use rwge::{
    color::{HSLA, RGBA},
    glam::{dvec2, ivec2, vec2, DVec2, Vec2},
    gui::rect_ui::{element::builder::ElementBuilder, event::UIEvent, RectMask},
    math_utils::lerp_f32,
    uuid::Uuid,
    winit::{
        dpi::{LogicalPosition, PhysicalPosition, PhysicalSize},
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

use super::{LayoutOrTabInfo, LayoutOrTabKey, LayoutSlotKey};

pub struct Window {
    pub layout_key: LayoutSlotKey,
    pub size: Vec2,
    pub position: Vec2,
    /////
    drag_window: DragElement,
    active_id: Option<Uuid>,
}

impl Window {
    pub fn new_with_contianer(layout_key: LayoutSlotKey, size: Vec2, position: Vec2) -> Self {
        Self {
            layout_key: layout_key,
            size,
            position,
            ////
            drag_window: DragElement::new(),
            active_id: None,
        }
    }

    pub fn handle_event(
        &mut self,
        event: &mut UIEvent,
        public_data: &PublicData,
        control_state: &mut ControlState,
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
            &mut self.active_id,
            &mut self.drag_window,
        );

        match event {
            UIEvent::Resize(screen_size) => {
                self.size = screen_size.as_vec2();
                self.position = self.size * 0.5;
            }
            UIEvent::Update => {
                if let Some(..) = self.active_id {
                    let time = get_engine_data(public_data).time.time_millis;
                    let mod_time = time % 50.0;
                    if mod_time <= 20.0 {
                        let param_x = get_engine_data(public_data).time.sin_time(2.0) * 0.5 + 0.5;
                        let param_y = get_engine_data(public_data).time.cos_time(2.0) * 0.5 + 0.5;

                        public_data.push_mut(Box::new(move |public_data| {
                            let wind = public_data
                                .get_mut::<rwge::winit::window::Window>()
                                .unwrap();
                            wind.set_inner_size(PhysicalSize::new(
                                lerp_f32(800.0, 1200.0, param_x).floor() as i32,
                                lerp_f32(500.0, 700.0, param_y).floor() as i32,
                            ));
                        }))
                    }
                }
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
