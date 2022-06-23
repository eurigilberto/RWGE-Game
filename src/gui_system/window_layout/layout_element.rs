use rwge::{
    color::RGBA,
    glam::{vec2, Vec2},
    gui::rect_ui::{
        element::builder::ElementBuilder,
        event::{MouseInput, UIEvent},
        Rect,
    },
    slotmap::slotmap::Slotmap,
    uuid::Uuid,
    winit::event::{ElementState, MouseButton},
};

use crate::gui_system::{
    control::{self, drag_element::DragElement, ControlId, ControlState},
    ContainerInfo,
};

use super::{LayoutOrTabInfo, LayoutOrTabKey, LayoutSlotKey, TabsSlotKey};

pub struct DividedElement {
    pub layout_key: LayoutSlotKey,
    pub size: f32,
}

pub struct ActiveDivider {
    active_id: Uuid,
    index: usize,
    drag_divider: LayoutDragDivider,
}

pub struct DivData {
    div_size: f32,
    div_index: usize,
    div_px_size: f32,
}

pub struct LayoutDragDivider {
    div_data: [DivData; 2],
    // Positions are only for the afected axis
    div_position: f32,
    start_cursor_position: f32,
    current_cursor_position: f32,
    total_px_size: f32,
    total_div_size: f32,
}

impl LayoutDragDivider {
    pub fn new(div_data: [DivData; 2], div_position: f32, start_cursor_position: f32) -> Self {
        let total_px_size = div_data[0].div_px_size + div_data[1].div_px_size;
        let total_div_size = div_data[0].div_size + div_data[1].div_size;
        Self {
            div_data,
            div_position,
            start_cursor_position,
            current_cursor_position: start_cursor_position,
            total_px_size,
            total_div_size,
        }
    }

    pub fn update_cursor_position(&mut self, current_position: f32) {
        self.current_cursor_position = current_position;
    }

    pub fn compute_new_division_sizes(&self, orientation: Orientation) -> (f32, f32) {
        let mut cursor_movement = self.current_cursor_position - self.start_cursor_position;
        if let Orientation::Vertical = orientation {
            cursor_movement *= -1.0;
        };
        let (div_px_size_0, div_px_size_1) = if cursor_movement < 0.0 {
            //negative means moving into the first node
            let abs_movement = cursor_movement.abs();
            let clamped_movement = abs_movement
                .max(0.0)
                .min(self.div_data[0].div_px_size - super::tabs_container::TAB_SIZE);
            (
                self.div_data[0].div_px_size - clamped_movement,
                self.div_data[1].div_px_size + clamped_movement,
            )
        } else {
            let clamped_movement = cursor_movement
                .max(0.0)
                .min(self.div_data[1].div_px_size - super::tabs_container::TAB_SIZE);
            (
                self.div_data[0].div_px_size + clamped_movement,
                self.div_data[1].div_px_size - clamped_movement,
            )
        };
        (
            (div_px_size_0 / self.total_px_size) * self.total_div_size,
            (div_px_size_1 / self.total_px_size) * self.total_div_size,
        )
    }
}

impl ActiveDivider {
    pub fn new(
        active_id: Uuid,
        index: usize,
        div_data: [DivData; 2],
        div_position: f32,
        start_cursor_position: f32,
    ) -> Self {
        Self {
            active_id,
            index,
            drag_divider: LayoutDragDivider::new(div_data, div_position, start_cursor_position),
        }
    }
}

pub enum LayoutElement {
    Single(TabsSlotKey),
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

const DIVISION_SIZE: f32 = 2.0;

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
                (LayoutElement::Horizontal { .. }, LayoutElement::Horizontal { .. }) => {
                    return false
                }
                (LayoutElement::Vertical { .. }, LayoutElement::Vertical { .. }) => return false,
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

    fn create_div_data(
        children_elements: &Vec<DividedElement>,
        children_sizes: &Vec<ChildrenInfo>,
        index: usize,
    ) -> DivData {
        DivData {
            div_size: children_elements[index].size,
            div_index: index,
            div_px_size: children_sizes[index].size,
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
            let (div_position, div_size, padding) = if let Orientation::Horizontal = orientation {
                (
                    vec2(*div_pos, container_info.position.y),
                    vec2(DIVISION_SIZE * 2.0, container_info.size.y),
                    vec2(DIVISION_SIZE * 2.0, 0.0),
                )
            } else {
                (
                    vec2(container_info.position.x, *div_pos),
                    vec2(container_info.size.x, DIVISION_SIZE * 2.0),
                    vec2(0.0, DIVISION_SIZE * 2.0),
                )
            };

            let r_mask = Rect {
                position: div_position,
                size: div_size + padding,
            };

            control_state.set_depth_and_save(container_info.depth_range.0);
            let control_id = control_state.get_id();

            match event {
                UIEvent::Update => {
                    if let Some(active_div) = active_divider {
                        if active_div.index == div_index {
                            control_state.hold_active_state(active_div.active_id);
                        }
                    } else {
                        control_state.update_hot_with_rect(control_id, &r_mask.into());
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
                UIEvent::MouseButton(MouseInput {
                    button: MouseButton::Left,
                    state: ElementState::Pressed,
                }) => {
                    if active_divider.is_none() {
                        if let Some(active_id) = control_state.set_active(control_id) {
                            let cursor_pos = control_state.last_cursor_position.unwrap();
                            let (div_position, start_cursor_position) =
                                if let Orientation::Horizontal = orientation {
                                    (div_position.x, cursor_pos.x)
                                } else {
                                    (div_position.y, cursor_pos.y)
                                };
                            *active_divider = Some(ActiveDivider::new(
                                active_id,
                                div_index,
                                [
                                    Self::create_div_data(
                                        children_elements,
                                        children_sizes,
                                        div_index,
                                    ),
                                    Self::create_div_data(
                                        children_elements,
                                        children_sizes,
                                        div_index + 1,
                                    ),
                                ],
                                div_position,
                                start_cursor_position,
                            ));
                        }
                    }
                }
                UIEvent::MouseButton(MouseInput {
                    button: MouseButton::Left,
                    state: ElementState::Released,
                }) => {
                    *active_divider = None;
                }
                UIEvent::Render {
                    extra_render_steps, ..
                } => {
                    let state = control_state.get_control_state(
                        if let Some(ActiveDivider {
                            active_id, index, ..
                        }) = active_divider
                        {
                            if div_index == *index {
                                ControlId::Active(*active_id)
                            } else {
                                control_id.into()
                            }
                        } else {
                            control_id.into()
                        },
                    );
                    match state {
                        control::State::Hovered => {
                            extra_render_steps.push(
                                Box::new(move |gui_rects| {
                                    ElementBuilder::new(div_position, div_size)
                                        .set_color(RGBA::new(0.0, 0.25, 0.75, 1.0).into())
                                        .build(gui_rects);
                                }),
                                20,
                            );
                        }
                        control::State::Active => {
                            extra_render_steps.push(
                                Box::new(move |gui_rects| {
                                    ElementBuilder::new(div_position, div_size)
                                        .set_color(RGBA::RED.into())
                                        .build(gui_rects);
                                }),
                                20,
                            );
                        }
                        _ => {}
                    }
                }
                _ => { /* No op */ }
            }
            control_state.restore_depth();
        }
    }

    pub fn handle_event(
        &mut self,
        event: &mut UIEvent,
        container_info: ContainerInfo,
        control_state: &mut ControlState,
    ) -> Vec<LayoutOrTabInfo> {
        let size = container_info.size;
        let position = container_info.position;
        match self {
            LayoutElement::Single(tab_container) => {
                // How to render the tab?
                let mut tab = Vec::<LayoutOrTabInfo>::new();
                tab.push(LayoutOrTabInfo {
                    key: LayoutOrTabKey::TabKey(tab_container.clone()),
                    container_info: ContainerInfo {
                        position,
                        size,
                        depth_range: container_info.depth_range,
                    },
                });
                tab
            }
            LayoutElement::Horizontal {
                children,
                active_divider,
                ..
            } => {
                let inner_size = size;
                let start_pos = position.x - inner_size.x * 0.5;

                let (children_sizes, division_positions) = compute_children_sizes(
                    children,
                    start_pos,
                    inner_size.x,
                    DIVISION_SIZE,
                    Sign::Positive,
                );

                Self::handle_event_layout_dividers(
                    children,
                    &children_sizes,
                    division_positions,
                    control_state,
                    container_info,
                    event,
                    Orientation::Horizontal,
                    active_divider,
                );

                children_sizes
                    .iter()
                    .map(|info| LayoutOrTabInfo {
                        key: info.key.clone(),
                        container_info: ContainerInfo {
                            position: vec2(info.position, position.y),
                            size: vec2(info.size, inner_size.y),
                            depth_range: container_info.depth_range,
                        },
                    })
                    .collect()
            }
            LayoutElement::Vertical {
                children,
                active_divider,
                ..
            } => {
                let inner_size = size;
                let start_pos = position.y + inner_size.y * 0.5;

                let (children_sizes, division_positions) = compute_children_sizes(
                    children,
                    start_pos,
                    inner_size.y,
                    DIVISION_SIZE,
                    Sign::Negative,
                );

                Self::handle_event_layout_dividers(
                    children,
                    &children_sizes,
                    division_positions,
                    control_state,
                    container_info,
                    event,
                    Orientation::Vertical,
                    active_divider,
                );

                children_sizes
                    .iter()
                    .map(|info| LayoutOrTabInfo {
                        key: info.key.clone(),
                        container_info: ContainerInfo {
                            position: vec2(position.x, info.position),
                            size: vec2(inner_size.x, info.size),
                            depth_range: container_info.depth_range,
                        },
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

struct ChildrenInfo {
    key: LayoutOrTabKey,
    size: f32,
    position: f32,
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
            key: LayoutOrTabKey::LayoutKey(child.layout_key),
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
