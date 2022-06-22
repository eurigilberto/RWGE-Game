use rwge::{
    color::RGBA,
    glam::{vec2, Vec2},
    gui::rect_ui::{element::builder::ElementBuilder, event::UIEvent, RectMask},
    slotmap::slotmap::Slotmap,
    uuid::Uuid,
};

use crate::gui_system::{control::ControlState, ContainerInfo};

use super::{DividedElement, LayoutOrTabInfo, LayoutOrTabKey, LayoutSlotKey, TabsSlotKey};

pub enum LayoutElement {
    Single(TabsSlotKey),
    Horizontal {
        children: Vec<DividedElement>,
        div_active_id: Option<Uuid>,
    },
    Vertical {
        children: Vec<DividedElement>,
        div_active_id: Option<Uuid>,
    },
}

enum Orientation {
    Horizontal,
    Vertical,
}

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
            LayoutElement::Horizontal {
                children,
                div_active_id,
            } => {
                children.reserve(new_children.len());
                children.extend(new_children);
            }
            LayoutElement::Vertical {
                children,
                div_active_id,
            } => {
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
        let horizontal_layout = LayoutElement::Horizontal{
            children: Vec::<DividedElement>::new(),
            div_active_id: None,
        };
        Self::validate_and_create(horizontal_layout, new_children, layout_elements)
    }

    pub fn create_vertical(
        new_children: Vec<DividedElement>,
        layout_elements: &mut Slotmap<LayoutElement>,
    ) -> Option<LayoutSlotKey> {
        let vertical_layout = LayoutElement::Vertical{
            children: Vec::<DividedElement>::new(),
            div_active_id: None,
        };
        Self::validate_and_create(vertical_layout, new_children, layout_elements)
    }

    fn handle_event_layout_dividers(
        division_positions: Vec<f32>,
        control_state: &mut ControlState,
        container_info: ContainerInfo,
        event: &mut UIEvent,
        orientation: Orientation,
        active_id: &mut Option<Uuid>
    ) {
        for div_pos in division_positions.iter() {
            let (div_position, div_size, padding) = if let Orientation::Horizontal = orientation {
                (
                    vec2(*div_pos, container_info.position.y),
                    vec2(4.0, container_info.size.y),
                    vec2(4.0, 0.0),
                )
            } else {
                (
                    vec2(container_info.position.x, *div_pos),
                    vec2(container_info.size.x, 4.0),
                    vec2(0.0, 4.0),
                )
            };

            let r_mask = RectMask {
                position: div_position,
                size: div_size + padding,
            };

            control_state.set_depth_and_save(container_info.depth_range.0);
            let control_id = control_state.get_id();

            match event {
                UIEvent::Update => {
                    control_state.update_hot_with_rect(control_id, &r_mask.into());
                }
                UIEvent::Render {
                    extra_render_steps, ..
                } => {
                    let state = control_state.get_control_state(control_id.into());
                    match state {
                        crate::gui_system::control::State::Hovered => {
                            extra_render_steps.push(
                                Box::new(move |gui_rects| {
                                    ElementBuilder::new(div_position, div_size)
                                        .set_color(RGBA::new(0.0, 0.25, 0.75, 1.0).into())
                                        .build(gui_rects);
                                }),
                                20,
                            );
                        }
                        crate::gui_system::control::State::Inactive => {}
                        crate::gui_system::control::State::Active => {}
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
            LayoutElement::Horizontal { children, div_active_id } => {
                let inner_size = size;
                let start_pos = position.x - inner_size.x * 0.5;

                let (children_sizes, division_positions) =
                    compute_children_sizes(children, start_pos, inner_size.x, 4.0, Sign::Positive);

                Self::handle_event_layout_dividers(
                    division_positions,
                    control_state,
                    container_info,
                    event,
                    Orientation::Horizontal,
                    div_active_id
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
            LayoutElement::Vertical { children, div_active_id } => {
                let inner_size = size;
                let start_pos = position.y + inner_size.y * 0.5;

                let (children_sizes, division_positions) =
                    compute_children_sizes(children, start_pos, inner_size.y, 4.0, Sign::Negative);

                Self::handle_event_layout_dividers(
                    division_positions,
                    control_state,
                    container_info,
                    event,
                    Orientation::Vertical,
                    div_active_id
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

struct ChildrenInfo{
    key: LayoutOrTabKey, 
    size: f32,
    position: f32
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
