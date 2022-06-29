use std::{cell::RefCell, sync::Arc};

use rwge::{
    color::{HSLA, RGBA},
    glam::{vec2, Vec2},
    gui::rect_ui::{
        element::{builder::ElementBuilder, LinearGradient, Border},
        event::{MouseInput, UIEvent},
        BorderRadius, Rect,
    },
    math_utils::lerp_f32,
    slotmap::slotmap::Slotmap,
    winit::event::{ElementState, MouseButton},
};

use crate::{
    gui_system::{
        control::{ControlState, State},
        gui_container::GUIContainer,
        ContainerInfo,
    },
    public_data::{utils::get_engine_data, PublicData},
};

use super::{depth_offset, GUIContainerInfo, GUIContainerSlotkey};

pub struct TabsContainer {
    tabs: Vec<GUIContainerSlotkey>,
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
        mut position: Vec2,
        mut size: Vec2,
        rect_mask: &Rect,
        is_active_tab: bool,
        public_data: &PublicData,
    ) -> bool {
        let control_id = control_state.get_id();

        match event {
            UIEvent::MouseButton(mouse_input) => {
                if let MouseInput {
                    button: MouseButton::Left,
                    state: ElementState::Pressed,
                } = mouse_input
                {
                    if control_state.is_hovered(control_id) {
                        return true;
                    }
                }
            }
            UIEvent::MouseMove { corrected, raw } => {}
            UIEvent::Update => {
                let btn_rect = Rect {
                    position: position,
                    size: size,
                };
                let mouse_test_rect = btn_rect.combine_rects(rect_mask);
                if let Some(rect_mask) = mouse_test_rect {
                    control_state.set_hot_with_rect(control_id, &rect_mask);
                }
            }
            UIEvent::Render {
                gui_rects,
                extra_render_steps,
            } => {
                let (round_rect, color) = if is_active_tab {
                    position -= vec2(0.0, TAB_GAP * 0.5);
                    size += vec2(0.0, TAB_GAP);
                    (
                        BorderRadius::ForTopBottom {
                            top: size.y * 0.5 - TAB_GAP,
                            bottom: 0.0,
                        },
                        GUI_ACTIVE_COLOR,
                    )
                } else {
                    (
                        BorderRadius::ForAll(size.y * 0.5 - 2.0),
                        GUI_INACTIVE_COLOR,
                    )
                };

                let state = control_state.get_control_state(control_id.into());
                let btn_color = if let State::Hovered = state {
                    let color_interp = get_engine_data(public_data).time.sin_time(0.5) * 0.5 + 0.5;
                    let color : RGBA = HSLA{
                        h: color_interp * 360.0,
                        s: 0.5,
                        l: 0.5,
                        a: 1.0,
                    }.into();
                    color
                } else {
                    color
                };

                let elem_build =
                    ElementBuilder::new(position, size).set_round_rect(round_rect.into());
                if is_active_tab {
                    let lin_gradient = LinearGradient {
                        colors: [RGBA::rrr1(0.10), color],
                        start_position: vec2(0.0, size.y * 0.5),
                        end_position: vec2(0.0, size.y * 0.5 - 6.0),
                    };
                    elem_build.set_linear_gradient(lin_gradient.into())
                } else {
                    let lin_gradient = LinearGradient {
                        colors: [GUI_INACTIVE_COLOR * 1.5, btn_color],
                        start_position: vec2(size.x * 0.5, 0.0),
                        end_position: vec2(-size.x * 0.5, 0.0),
                    };
                    elem_build.set_linear_gradient(lin_gradient.into())/*.set_border(Some(Border{
                        size: 2,
                        color: RGBA::BLACK.set_alpha(0.25).into(),
                    }))*/
                }
                .set_rect_mask((*rect_mask).into())
                .build(gui_rects);
            }
            UIEvent::Consumed => {},
            _ => {}
        }
        return false;
    }

    pub fn create_tab_buttons(
        &mut self,
        control_state: &mut ControlState,
        container_info: &ContainerInfo,
        event: &mut UIEvent,
        public_data: &PublicData,
    ) {
        let tab_menu_size = vec2(container_info.rect.size.x, TAB_SIZE);
        let tab_menu_position = vec2(
            container_info.rect.position.x,
            container_info.rect.position.y + (container_info.rect.size.y - TAB_SIZE) * 0.5,
        );

        let rect_mask = Rect {
            position: tab_menu_position,
            size: tab_menu_size,
        };

        let left_position = tab_menu_position - vec2(tab_menu_size.x * 0.5, 0.0);
        let mut current_pos = left_position;

        for index in 0..self.tabs.len() {
            let tab_btn_pos = current_pos + vec2(TAB_GAP + TAB_WIDTH * 0.5, 0.0);
            let tab_btn_size = vec2(TAB_WIDTH, tab_menu_size.y - TAB_GAP * 2.0);
            current_pos += vec2(TAB_GAP + TAB_WIDTH, 0.0);

            if Self::tab_button(
                control_state,
                event,
                tab_btn_pos,
                tab_btn_size,
                &rect_mask,
                index == self.active_tab,
                public_data,
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
        gui_container_collection: &Slotmap<Box<dyn GUIContainer>>,
        control_state: &mut ControlState,
        instance_index: usize,
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

        match event {
            UIEvent::Render {
                gui_rects,
                extra_render_steps,
            } => {
                let color: RGBA = TAB_BG_COLOR.into();

                ElementBuilder::new(tab_menu_position, tab_menu_size)
                    .set_color(color.into())
                    .build(gui_rects);

                let lin_gradient = LinearGradient {
                    colors: [RGBA::rrr1(0.10), RGBA::rrr1(0.10).set_alpha(0.0)],
                    start_position: vec2(0.0, 3.0),
                    end_position: vec2(0.0, -3.0),
                };

                let container_top_left = container_info.get_top_left_position();

                let left_shadow_size = (self.active_tab as f32) * (TAB_GAP + TAB_WIDTH) + TAB_GAP;
                let left_shadow_size = left_shadow_size.min(container_info.rect.size.x);
                let left_shadow_position = container_top_left.x + left_shadow_size * 0.5;

                let right_shadow_start_pos = (self.active_tab as f32 + 1.0) * (TAB_GAP + TAB_WIDTH);
                if right_shadow_start_pos < container_info.rect.size.x {
                    let right_shadow_size = container_info.rect.size.x - right_shadow_start_pos;
                    let right_shadow_pos = right_shadow_start_pos
                        + container_info.get_top_left_position().x
                        + right_shadow_size * 0.5;
                    extra_render_steps.push(
                        Box::new(move |gui_rects| {
                            ElementBuilder::new(
                                vec2(
                                    right_shadow_pos,
                                    tab_menu_position.y - (tab_menu_size.y * 0.5 + 3.0),
                                ),
                                vec2(right_shadow_size, 6.0),
                            )
                            .set_linear_gradient(lin_gradient.into())
                            .build(gui_rects);

                            ElementBuilder::new(
                                vec2(
                                    left_shadow_position,
                                    tab_menu_position.y - (tab_menu_size.y * 0.5 + 3.0),
                                ),
                                vec2(left_shadow_size, 6.0),
                            )
                            .set_linear_gradient(lin_gradient.into())
                            .build(gui_rects);
                        }),
                        container_info.depth_range.0 + depth_offset::TAB_SHADOW,
                    );
                } else {
                    extra_render_steps.push(
                        Box::new(move |gui_rects| {
                            ElementBuilder::new(
                                vec2(
                                    left_shadow_position,
                                    tab_menu_position.y - (tab_menu_size.y * 0.5 + 3.0),
                                ),
                                vec2(left_shadow_size, 6.0),
                            )
                            .set_linear_gradient(lin_gradient.into())
                            .build(gui_rects);
                        }),
                        container_info.depth_range.0 + depth_offset::TAB_SHADOW,
                    );
                }
            }
            _ => {}
        };

        let gui_container = GUIContainerInfo {
            key: active_tab_key,
            container_info: ContainerInfo {
                rect: Rect {
                    position: container_position,
                    size: vec2(container_size.x, container_size.y.round()),
                },
                depth_range: container_info.depth_range,
            },
        };

        self.create_tab_buttons(control_state, &container_info, event, public_data);
        gui_container
    }
}
