pub mod container_one;
pub mod performance_monitor;
pub mod text_layout_test;
pub mod text_animation;

use std::any::Any;

use rwge::{
    glam::{UVec2, Vec2},
    gui::rect_ui::{event::UIEvent, GUIRects, element::builder::ElementBuilder, Rect},
    Engine,
};

use crate::{as_any::AsAny, public_data::PublicData};

use super::{control::ControlState, ContainerInfo, window_layout::GUI_ACTIVE_COLOR};

pub trait GUIContainer: AsAny {
    fn get_name(&self) -> &str;
    fn handle_event(
        &mut self,
        event: &mut UIEvent,
        public_data: &PublicData,
        container_info: ContainerInfo,
        control_state: &mut ControlState,
    );
}

impl<T: GUIContainer + 'static> AsAny for T {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

pub fn render_container_background(gui_rects: &mut GUIRects, container_info: &ContainerInfo) {
    ElementBuilder::new(container_info.rect.position, container_info.rect.size)
        .set_color(GUI_ACTIVE_COLOR.into())
        .set_rect_mask(
            container_info.rect
            .into(),
        )
        .build(gui_rects);
}
