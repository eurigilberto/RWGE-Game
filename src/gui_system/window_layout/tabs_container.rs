use rwge::{
    color::*,
    font::font_layout::create_single_line,
    glam::*,
    gui::rect_ui::{
        element::{builder::ElementBuilder, LinearGradient},
        event::UIEvent,
        BorderRadius, GUIRects, Rect,
    },
};

use crate::{
    gui_system::{
        control::{ControlState, State},
        ContainerInfo,
    },
    runtime_data::{
        utils::{get_font_collections, get_time},
        PublicData,
    },
};

use super::{depth_offset, GUIContainerInfo, GUIContainerSlotkey};

pub struct TabsContainer {
    pub tabs: Vec<GUIContainerSlotkey>,
    active_tab: usize,
}

pub const TAB_SIZE: f32 = 30.0;
pub const TAB_WIDTH: f32 = 100.0;
pub const TAB_GAP: f32 = 5.0;
pub const TAB_BG_COLOR: RGBA = RGBA::rrr1(0.25);
pub const GUI_ACTIVE_COLOR: RGBA = RGBA::rrr1(0.2);
pub const GUI_HOVER_COLOR: RGBA = RGBA::rgb(0.4, 0.9, 0.0);
pub const GUI_INACTIVE_COLOR: RGBA = RGBA::rrr1(0.35);

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

    pub fn tab_button(
        control_state: &mut ControlState,
        event: &mut UIEvent,
        mut rect: Rect,
        rect_mask: &Rect,
        is_active_tab: bool,
        public_data: &PublicData,
        tab_name: &str,
    ) -> bool {
        let control_id = control_state.get_id();

        if let UIEvent::MouseButton(mouse_input) = event {
            if mouse_input.is_left_pressed() {
                if control_state.is_hovered(control_id) {
                    return true;
                }
            }
        }

        if let UIEvent::Update = event {
            let mouse_test_rect = rect.combine_rects(rect_mask);
            if let Some(rect_mask) = mouse_test_rect {
                control_state.set_hot_with_rect(control_id, &rect_mask);
            }
        }

        if let UIEvent::Render { gui_rects, .. } = event {
            let (round_rect, color) = if is_active_tab {
                rect = rect
                    .offset_position(-vec2(0.0, TAB_GAP * 0.5))
                    .offset_size(vec2(0.0, TAB_GAP + 2.0));
                (
                    BorderRadius::ForTopBottom {
                        top: rect.size.y * 0.5 - TAB_GAP,
                        bottom: 0.0,
                    },
                    GUI_ACTIVE_COLOR,
                )
            } else {
                (
                    BorderRadius::ForAll(rect.size.y * 0.5 - 2.0),
                    GUI_INACTIVE_COLOR,
                )
            };

            let state = control_state.get_control_state(control_id.into());
            let btn_color = if let State::Hovered = state {
                let color_interp = get_time(public_data).sin_time(0.5) * 0.5 + 0.5;
                let color: RGBA = HSLA {
                    h: color_interp * 360.0,
                    s: 0.5,
                    l: 0.5,
                    a: 1.0,
                }
                .into();
                color
            } else {
                color
            };

            let elem_build = ElementBuilder::new_with_rect(rect).set_round_rect(round_rect.into());
            if is_active_tab {
                let lin_gradient = LinearGradient {
                    colors: [RGBA::rrr1(0.10), color],
                    start_position: vec2(0.0, rect.size.y * 0.5),
                    end_position: vec2(0.0, rect.size.y * 0.5 - 6.0),
                };
                elem_build.set_linear_gradient(lin_gradient.into())
            } else {
                let lin_gradient = LinearGradient {
                    colors: [GUI_INACTIVE_COLOR * 1.5, btn_color],
                    start_position: vec2(rect.size.x * 0.5, 0.0),
                    end_position: vec2(-rect.size.x * 0.5, 0.0),
                };
                elem_build.set_linear_gradient(lin_gradient.into())
            }
            .set_rect_mask((*rect_mask).into())
            .build(gui_rects);

            let font_collection = &get_font_collections(public_data)[0];
            let (font_elements, mut text_rect) =
                create_single_line(tab_name, 16.0, font_collection, 0, 0.0);

            for font_elem in font_elements {
                ElementBuilder::new_with_rect(
                    font_elem
                        .rect
                        .offset_position(rect.position - text_rect.size * 0.5),
                )
                .set_rect_mask((*rect_mask).into())
                .set_sdffont(font_elem.tx_slice.into())
                .build(gui_rects);
            }
        }
        return false;
    }

    pub fn create_tab_buttons(
        &mut self,
        control_state: &mut ControlState,
        container_info: &ContainerInfo,
        event: &mut UIEvent,
        public_data: &PublicData,
        tab_rect: Rect,
        tab_names: &Vec<&str>,
    ) {
        let mut current_pos = tab_rect.left_position();

        for index in 0..self.tabs.len() {
            let tab_btn_pos = current_pos + vec2(TAB_GAP + TAB_WIDTH * 0.5, 0.0);
            let tab_btn_size = vec2(TAB_WIDTH, tab_rect.size.y - TAB_GAP * 2.0);

            let rect = Rect {
                position: tab_btn_pos,
                size: tab_btn_size,
            };

            current_pos += vec2(TAB_GAP + TAB_WIDTH, 0.0);

            if Self::tab_button(
                control_state,
                event,
                rect,
                &tab_rect,
                index == self.active_tab,
                public_data,
                &tab_names[index],
            ) {
                self.active_tab = index;
            }
        }
    }

    pub fn handle_event(
        &mut self,
        event: &mut UIEvent,
        public_data: &PublicData,
        container_info: ContainerInfo,
        control_state: &mut ControlState,
        tab_names: &Vec<&str>,
    ) -> GUIContainerInfo {
        let active_tab_key = self.tabs[self.active_tab];

        let container_position = vec2(
            container_info.rect.position.x,
            container_info.rect.position.y - (TAB_SIZE * 0.5),
        );
        let container_size = vec2(
            container_info.rect.size.x,
            container_info.rect.size.y - TAB_SIZE,
        );

        let tab_menu_size = vec2(container_info.rect.size.x, TAB_SIZE);
        let tab_menu_position = vec2(
            container_info.rect.position.x,
            container_info.rect.position.y + (container_info.rect.size.y - TAB_SIZE) * 0.5,
        );

        let tab_rect = Rect {
            position: tab_menu_position,
            size: tab_menu_size,
        };

        if let UIEvent::Render {
            gui_rects,
            extra_render_steps,
        } = event
        {
            ElementBuilder::new(tab_menu_position, tab_menu_size)
                .set_color(TAB_BG_COLOR.into())
                .build(gui_rects);

            extra_render_steps.push(
                render_shadow_under_tab(self.active_tab, container_info, tab_rect),
                container_info.depth_range.0 + depth_offset::TAB_SHADOW,
            );
        }

        self.create_tab_buttons(
            control_state,
            &container_info,
            event,
            public_data,
            tab_rect,
            tab_names,
        );

        GUIContainerInfo {
            key: active_tab_key,
            container_info: ContainerInfo {
                rect: Rect {
                    position: container_position,
                    size: vec2(container_size.x, container_size.y.round()),
                },
                depth_range: container_info.depth_range,
            },
        }
    }
}

fn render_shadow_under_tab(
    active_tab: usize,
    container_info: ContainerInfo,
    tab_rect: Rect,
) -> Box<dyn FnOnce(&mut GUIRects) -> ()> {
    let left_shadow_size = (active_tab as f32) * (TAB_GAP + TAB_WIDTH) + TAB_GAP;
    let left_shadow_size = left_shadow_size.min(container_info.rect.size.x);
    let left_shadow_position = container_info.top_left_position().x + left_shadow_size * 0.5;

    let right_shadow_start_pos = (active_tab as f32 + 1.0) * (TAB_GAP + TAB_WIDTH);
    let right_shadow_size = container_info.rect.size.x - right_shadow_start_pos;
    let right_shadow_pos =
        right_shadow_start_pos + container_info.top_left_position().x + right_shadow_size * 0.5;

    let show_right_shadow = right_shadow_start_pos < container_info.rect.size.x;

    let bottom_position = tab_rect.position.y - (tab_rect.size.y * 0.5 + 3.0);
    let right_shadow_rect = Rect {
        position: vec2(right_shadow_pos, bottom_position),
        size: vec2(right_shadow_size, 6.0),
    };
    let left_shadow_rect = Rect {
        position: vec2(left_shadow_position, bottom_position),
        size: vec2(left_shadow_size, 6.0),
    };
    let lin_gradient = LinearGradient {
        colors: [RGBA::rrr1(0.10), RGBA::rrr1(0.10).set_alpha(0.0)],
        start_position: vec2(0.0, 3.0),
        end_position: vec2(0.0, -3.0),
    };

    let mut result_elems = Vec::with_capacity(2);
    if show_right_shadow {
        result_elems.push(
            ElementBuilder::new_with_rect(right_shadow_rect)
                .set_linear_gradient(lin_gradient.into()),
        );
    }
    result_elems.push(
        ElementBuilder::new_with_rect(left_shadow_rect).set_linear_gradient(lin_gradient.into()),
    );

    Box::new(move |gui_rects| {
        for elem in result_elems.drain(..){
            elem.build(gui_rects);
        }
    })
}
