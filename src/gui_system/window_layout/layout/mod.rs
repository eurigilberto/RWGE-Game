use rwge::{
    color::RGBA,
    glam::{vec2, Vec2},
    gui::rect_ui::{element::builder::ElementBuilder, event::UIEvent, Rect},
    slotmap::slotmap::Slotmap,
};

use crate::gui_system::{
    control::{self, ControlId, ControlState, State, Uiid},
    ContainerInfo,
};

use self::active_divider::{ActiveDivider, DivData};

use super::{depth_offset, LayoutOrTabInfo, LayoutOrTabKey, LayoutSlotKey, TabsSlotKey};

pub struct DividedElement {
    pub layout_or_tab_key: LayoutOrTabKey,
    pub size: f32,
}

impl DividedElement {
    pub fn new(key: LayoutOrTabKey, size: f32) -> Self {
        Self {
            layout_or_tab_key: key,
            size,
        }
    }
    pub fn new_layout(key: LayoutSlotKey, size: f32) -> Self {
        Self {
            layout_or_tab_key: key.into(),
            size,
        }
    }
    pub fn new_tab(key: TabsSlotKey, size: f32) -> Self {
        Self {
            layout_or_tab_key: key.into(),
            size,
        }
    }
}

pub mod active_divider;

pub enum LayoutElement {
    Horizontal {
        children: Vec<DividedElement>,
        active_divider: Option<ActiveDivider>,
    },
    Vertical {
        children: Vec<DividedElement>,
        active_divider: Option<ActiveDivider>,
    },
}

#[derive(Clone, Copy)]
pub enum Orientation {
    Horizontal,
    Vertical,
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

struct ChildrenInfo {
    key: LayoutOrTabKey,
    size: f32,
    position: f32,
}

const DIVISION_SIZE: f32 = 2.0;

impl LayoutElement {
    pub fn validate_children(
        //This does not preoprtly validate the childrens
        &self,
        children: &Vec<DividedElement>,
        layout_elements: &Slotmap<LayoutElement>,
    ) -> bool {
        for child in children {
            match child.layout_or_tab_key {
                LayoutOrTabKey::TabKey(_) => return true,
                LayoutOrTabKey::LayoutKey(layout_key) => {
                    let layout_element = layout_elements.get_value(&layout_key).unwrap();
                    match (self, layout_element) {
                        (LayoutElement::Horizontal { .. }, LayoutElement::Horizontal { .. }) => {
                            return false
                        }
                        (LayoutElement::Vertical { .. }, LayoutElement::Vertical { .. }) => {
                            return false
                        }
                        (_, _) => {}
                    }
                }
            }
        }
        if children.len() == 1 {
            return false;
        }
        return true;
    }

    pub fn push_children(&mut self, new_children: Vec<DividedElement>) {
        match self {
            LayoutElement::Horizontal { children, .. } => {
                children.reserve(new_children.len());
                children.extend(new_children);
            }
            LayoutElement::Vertical { children, .. } => {
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
        let horizontal_layout = LayoutElement::Horizontal {
            children: Vec::<DividedElement>::new(),
            active_divider: None,
        };
        Self::validate_and_create(horizontal_layout, new_children, layout_elements)
    }

    pub fn create_vertical(
        new_children: Vec<DividedElement>,
        layout_elements: &mut Slotmap<LayoutElement>,
    ) -> Option<LayoutSlotKey> {
        let vertical_layout = LayoutElement::Vertical {
            children: Vec::<DividedElement>::new(),
            active_divider: None,
        };
        Self::validate_and_create(vertical_layout, new_children, layout_elements)
    }

    pub fn handle_event(
        &mut self,
        event: &mut UIEvent,
        container_info: ContainerInfo,
        control_state: &mut ControlState,
    ) -> Vec<LayoutOrTabInfo> {
        let size = container_info.rect.size;
        let position = container_info.rect.position;
        match self {
            LayoutElement::Horizontal {
                children,
                active_divider,
                ..
            } => handle_event_layout_element(
                children,
                control_state,
                container_info,
                event,
                active_divider,
                Sign::Positive,
                position.x - size.x * 0.5,
                size.x,
                Orientation::Horizontal,
                &|child_info| Rect {
                    position: vec2(child_info.position, position.y),
                    size: vec2(child_info.size, size.y),
                },
            ),
            LayoutElement::Vertical {
                children,
                active_divider,
                ..
            } => handle_event_layout_element(
                children,
                control_state,
                container_info,
                event,
                active_divider,
                Sign::Negative,
                position.y + size.y * 0.5,
                size.y,
                Orientation::Vertical,
                &|child_info| Rect {
                    position: vec2(position.x, child_info.position),
                    size: vec2(size.x, child_info.size),
                },
            ),
        }
    }
}

fn handle_event_layout_element(
    children: &mut Vec<DividedElement>,
    control_state: &mut ControlState,
    container_info: ContainerInfo,
    event: &mut UIEvent,
    active_divider: &mut Option<ActiveDivider>,
    sign: Sign,
    start_position: f32,
    inner_size: f32,
    orientation: Orientation,
    create_child_rect: &dyn Fn(&ChildrenInfo) -> Rect,
) -> Vec<LayoutOrTabInfo> {
    let (children_sizes, division_positions) =
        compute_children_sizes(children, start_position, inner_size, DIVISION_SIZE, sign);

    handle_event_layout_dividers(
        children,
        &children_sizes,
        division_positions,
        control_state,
        container_info,
        event,
        orientation,
        active_divider,
    );

    children_sizes
        .iter()
        .map(|info| LayoutOrTabInfo {
            key: info.key.clone(),
            container_info: ContainerInfo {
                rect: create_child_rect(info),
                depth_range: container_info.depth_range,
            },
        })
        .collect()
}

fn create_div_rects(
    div_position: f32,
    orientation: Orientation,
    container_info: ContainerInfo,
) -> Rect {
    let (div_position, div_size) = if let Orientation::Horizontal = orientation {
        (
            vec2(div_position, container_info.rect.position.y),
            vec2(DIVISION_SIZE * 4.0, container_info.rect.size.y),
        )
    } else {
        (
            vec2(container_info.rect.position.x, div_position),
            vec2(container_info.rect.size.x, DIVISION_SIZE * 4.0),
        )
    };

    Rect {
        //draw rect
        position: div_position,
        size: div_size,
    }
}

fn handle_event_layout_dividers(
    children_elements: &mut Vec<DividedElement>,
    children_sizes: &Vec<ChildrenInfo>,
    division_positions: Vec<f32>,
    control_state: &mut ControlState,
    container_info: ContainerInfo,
    event: &mut UIEvent,
    orientation: Orientation,
    active_divider: &mut Option<ActiveDivider>,
) {
    for (div_index, div_pos) in division_positions.iter().enumerate() {
        let draw_rect = create_div_rects(*div_pos, orientation, container_info);

        control_state.set_depth_and_save(container_info.depth_range.0 + depth_offset::DIVIDER);
        let control_id = control_state.get_id();
        control_state.restore_depth();

        match event {
            UIEvent::Update => {
                if let Some(active_div) = active_divider {
                    if active_div.index == div_index {
                        control_state.hold_active_state(active_div.active_id);
                    }
                } else {
                    control_state.set_hot_with_rect(control_id, &draw_rect.into());
                }
            }
            UIEvent::MouseMove { corrected, .. } => {
                if let Some(active_div) = active_divider {
                    if active_div.index == div_index {
                        active_div.drag_divider.update_cursor_position(
                            if let Orientation::Horizontal = orientation {
                                corrected.x
                            } else {
                                corrected.y
                            },
                        );

                        let new_div_sizes = active_div
                            .drag_divider
                            .compute_new_division_sizes(orientation);
                        children_elements[div_index].size = new_div_sizes.0;
                        children_elements[div_index + 1].size = new_div_sizes.1;
                    }
                }
            }
            UIEvent::MouseButton(mouse_input) => {
                if mouse_input.is_left_pressed() && active_divider.is_none() {
                    if let Some(active_id) = control_state.set_active(control_id) {
                        let cursor_pos = control_state.last_cursor_position.unwrap();
                        let start_cursor_position = if let Orientation::Horizontal = orientation {
                            cursor_pos.x
                        } else {
                            cursor_pos.y
                        };
                        *active_divider = Some(ActiveDivider::new(
                            active_id,
                            div_index,
                            [
                                DivData {
                                    div_size: children_elements[div_index].size,
                                    div_px_size: children_sizes[div_index].size,
                                },
                                DivData {
                                    div_size: children_elements[div_index + 1].size,
                                    div_px_size: children_sizes[div_index + 1].size,
                                },
                            ],
                            start_cursor_position,
                        ));
                    }
                }

                if mouse_input.is_left_released() {
                    *active_divider = None;
                }
            }
            UIEvent::Render {
                extra_render_steps, ..
            } => {
                let state =
                    get_current_state(&control_state, &active_divider, div_index, control_id);
                let divider_color = match state {
                    control::State::Hovered => RGBA::new(0.0, 0.25, 0.75, 1.0),
                    control::State::Active => RGBA::RED,
                    _ => RGBA::TRANSPARENT,
                };
                match state {
                    control::State::Hovered | control::State::Active => extra_render_steps.push(
                        Box::new(move |gui_rects| {
                            ElementBuilder::new_with_rect(draw_rect)
                                .set_color(divider_color.into())
                                .build(gui_rects);
                        }),
                        container_info.depth_range.0 + depth_offset::RESIZE_CONTROL,
                    ),
                    _ => {}
                }
            }
            _ => { /* No op */ }
        }
    }
}

fn get_current_state(
    control_state: &ControlState,
    divider: &Option<ActiveDivider>,
    div_index: usize,
    control_id: Uiid,
) -> State {
    control_state.get_control_state(
        if let Some(ActiveDivider {
            active_id, index, ..
        }) = divider
        {
            if div_index == *index {
                ControlId::Active(*active_id)
            } else {
                control_id.into()
            }
        } else {
            control_id.into()
        },
    )
}

fn compute_children_sizes(
    children: &Vec<DividedElement>,
    start_pos: f32,
    inner_size: f32,
    margin: f32,
    sign: Sign,
) -> (Vec<ChildrenInfo>, Vec<f32>) {
    let mut children_sizes = Vec::<ChildrenInfo>::with_capacity(children.len());
    let mut division_positions = Vec::<f32>::with_capacity(children.len());

    let total_size = children
        .iter()
        .fold(0.0, |acum: f32, div: &DividedElement| acum + div.size);

    let mut start_pos = start_pos;
    let gap_count = (children.len() as i32 - 1) as f32;
    let children_available_size = inner_size - (margin * gap_count);

    for (index, child) in children.iter().enumerate() {
        let element_proportion = child.size / total_size;
        let child_size = children_available_size * element_proportion;
        let child_position = start_pos + child_size * 0.5 * sign.as_f32();

        children_sizes.push(ChildrenInfo {
            key: child.layout_or_tab_key,
            size: child_size,
            position: child_position,
        });
        start_pos += (child_size + margin) * sign.as_f32();
        if index != children.len() - 1 {
            division_positions.push(start_pos - (margin * 0.5 * sign.as_f32()))
        }
    }
    (children_sizes, division_positions)
}
