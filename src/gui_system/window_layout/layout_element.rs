use rwge::{
    glam::{vec2, Vec2},
    gui::rect_ui::event::UIEvent,
    slotmap::slotmap::Slotmap,
};

use crate::gui_system::ContainerInfo;

use super::{DividedElement, LayoutOrTabInfo, LayoutOrTabKey, LayoutSlotKey, TabsSlotKey};

pub enum LayoutElement {
    Single(TabsSlotKey),
    Horizontal(Vec<DividedElement>),
    Vertical(Vec<DividedElement>),
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
                (LayoutElement::Horizontal(_), LayoutElement::Horizontal(_)) => return false,
                (LayoutElement::Vertical(_), LayoutElement::Vertical(_)) => return false,
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
            LayoutElement::Horizontal(children) => {
                children.reserve(new_children.len());
                children.extend(new_children);
            }
            LayoutElement::Vertical(children) => {
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
        let horizontal_layout = LayoutElement::Horizontal(Vec::<DividedElement>::new());
        Self::validate_and_create(horizontal_layout, new_children, layout_elements)
    }

    pub fn create_vertical(
        new_children: Vec<DividedElement>,
        layout_elements: &mut Slotmap<LayoutElement>,
    ) -> Option<LayoutSlotKey> {
        let vertical_layout = LayoutElement::Vertical(Vec::<DividedElement>::new());
        Self::validate_and_create(vertical_layout, new_children, layout_elements)
    }

    pub fn handle_event(
        &mut self,
        event: &mut UIEvent,
        container_info: ContainerInfo,
    ) -> Vec<LayoutOrTabInfo> {
        let size = container_info.size;
        let position = container_info.position;
        match self {
            LayoutElement::Single(tab_container) => {
                // How to render the tab?
                let mut tab = Vec::<LayoutOrTabInfo>::new();
                tab.push(LayoutOrTabInfo {
                    key: LayoutOrTabKey::TabKey(tab_container.clone()),
                    container_info: ContainerInfo { position, size }
                });
                tab
            }
            LayoutElement::Horizontal(children) => {
                let margin: f32 = 10.0;
                let inner_size = size;
                let start_pos = position.x - f32::ceil(inner_size.x * 0.5);

                let children_sizes =
                    compute_children_sizes(children, start_pos, inner_size.x, 10.0, Sign::Positive);
                children_sizes
                    .iter()
                    .map(|(key, size, pos)| LayoutOrTabInfo {
                        key: key.clone(),
                        container_info: ContainerInfo {
                            position: vec2(*pos, position.y),
                            size: vec2(*size, inner_size.y),
                        },
                    })
                    .collect()
            }
            LayoutElement::Vertical(children) => {
                let margin: f32 = 10.0;
                let inner_size = size;
                let start_pos = position.y + f32::ceil(inner_size.y * 0.5);

                let children_sizes =
                    compute_children_sizes(children, start_pos, inner_size.y, 10.0, Sign::Negative);
                children_sizes
                    .iter()
                    .map(|(key, size, pos)| LayoutOrTabInfo {
                        key: key.clone(),
                        container_info: ContainerInfo{
                            position: vec2(position.x, *pos),
                            size: vec2(inner_size.x, *size),
                        }
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

fn compute_children_sizes(
    children: &Vec<DividedElement>,
    start_pos: f32,
    inner_size: f32,
    margin: f32,
    sign: Sign,
) -> Vec<(LayoutOrTabKey, f32, f32)> {
    let mut children_sizes = Vec::<(LayoutOrTabKey, f32, f32)>::new();
    let total_size = children
        .iter()
        .fold(0.0, |acum: f32, div: &DividedElement| acum + div.size);

    let mut start_pos = start_pos;
    let gap_count = (children.len() as i32 - 1) as f32;
    let children_available_size = inner_size - (margin * gap_count);

    for child in children {
        let element_proportion = child.size / total_size;
        let child_size = children_available_size * element_proportion;
        let child_position = start_pos + child_size * 0.5 * sign.as_f32();

        children_sizes.push((
            LayoutOrTabKey::LayoutKey(child.layout_key),
            child_size,
            child_position,
        ));
        start_pos += (child_size + margin) * sign.as_f32();
    }
    children_sizes
}
