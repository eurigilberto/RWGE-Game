use std::{
    cell::RefCell,
    collections::{HashMap, VecDeque},
};

use rwge::{
    color::{HSLA, RGBA},
    font::font_layout::{FontElement, WordRect},
    glam::{vec2, Vec2},
    gui::rect_ui::{element::builder::ElementBuilder, event::UIEvent, Rect},
    math_utils::{easeInBack, easeOutBack},
    slotmap::slotmap::Slotmap,
    uuid::Uuid,
};

use crate::{
    gui_system::window_layout::depth_offset,
    public_data::{self, utils::get_time, PublicData},
};

use super::{render_container_background, GUIContainer};

#[derive(Clone)]
pub struct WordAnimData {
    word_rect: Rect,
    font_elements: Vec<FontElement>,
    initial_offset: Vec2,
    initial_time: f32,
    final_offset: Option<Vec2>,

    current_offset: Vec<Vec2>,
    current_scale_mult: Vec<Vec2>,
    current_color: Vec<RGBA>,

    char_indices: Vec<usize>,
    char_count: usize,
    rand_value: f32,
}
impl WordAnimData {
    pub fn new(
        word_rect: Rect,
        font_elements: Vec<FontElement>,
        initial_offset: Vec2,
        initial_time: f32,
    ) -> Self {
        let char_count = font_elements.len();
        Self {
            word_rect,
            font_elements,
            initial_offset,
            initial_time,
            final_offset: None,
            current_offset: vec![Vec2::ZERO; char_count],
            current_scale_mult: vec![Vec2::ONE; char_count],
            current_color: vec![RGBA::WHITE; char_count],
            char_indices: (0..char_count).collect(),
            char_count,
            rand_value: rwge::rand::random(),
        }
    }
}

pub struct WordAnimation {
    anims: RefCell<Vec<WordAnimData>>,
}

impl WordAnimation {
    pub fn new() -> Self {
        Self {
            anims: RefCell::new(Vec::new()),
        }
    }

    fn push_anim_data(&self, anim_data: WordAnimData) {
        self.anims.borrow_mut().push(anim_data);
    }
}

pub struct TextAnimationData {
    instance_queues: HashMap<Uuid, WordAnimation>,
}

impl TextAnimationData {
    pub fn new() -> Self {
        Self {
            instance_queues: HashMap::new(),
        }
    }
    pub fn insert_queue(&mut self, id: Uuid) {
        self.instance_queues.insert(id, WordAnimation::new());
    }
    pub fn push_anim_data(&self, anim_data: WordAnimData) {
        for (_, queue) in self.instance_queues.iter() {
            queue.push_anim_data(anim_data.clone());
        }
    }

    pub fn get_instance_word_anim<'a>(&'a self, id: Uuid) -> &'a WordAnimation {
        self.instance_queues.get(&id).unwrap()
    }
}

fn contains_instance_anim(public_data: &PublicData, id: Uuid) -> bool {
    public_data
        .collection
        .get::<TextAnimationData>()
        .unwrap()
        .instance_queues
        .contains_key(&id)
}

fn get_instance_word_anim(public_data: &PublicData, id: Uuid) -> &WordAnimation {
    public_data
        .collection
        .get::<TextAnimationData>()
        .unwrap()
        .get_instance_word_anim(id)
}

const ANIM_DURATION: f32 = 4.0;

const ANIM_TO_BOX: (f32, f32) = (0.0, 0.25);
const ANIM_TO_ROTATION: (f32, f32) = (0.23, 0.225);
const ANIM_TO_CENTER_FADE: (f32, f32) = (0.75, 0.24);

const ANIM_OFFSET: f32 = 0.1;

pub struct TextAnimation {
    first_update_done: bool,
    uuid: Uuid,
}

impl TextAnimation {
    pub fn new() -> Self {
        Self {
            first_update_done: false,
            uuid: Uuid::new_v4(),
        }
    }
}

impl GUIContainer for TextAnimation {
    fn get_name(&self) -> &str {
        "Text Anim"
    }

    fn handle_event(
        &mut self,
        event: &mut rwge::gui::rect_ui::event::UIEvent,
        public_data: &crate::public_data::PublicData,
        container_info: crate::gui_system::ContainerInfo,
        control_state: &mut crate::gui_system::control::ControlState,
    ) {
        //initialize public data struct
        if self.first_update_done == false {
            if let UIEvent::Update = event {
                let uuid = self.uuid;
                public_data.push_mut(Box::new(move |public_data| {
                    public_data
                        .get_mut::<TextAnimationData>()
                        .unwrap()
                        .insert_queue(uuid);
                }));
                self.first_update_done = true;
            }
        } else {
            if let UIEvent::Update = event {
                let current_time = get_time(public_data).time;
                let word_anim = get_instance_word_anim(public_data, self.uuid);
                let mut anim_instances = word_anim.anims.borrow_mut();

                let mut delete_ad_indices = Vec::with_capacity(anim_instances.len());
                for (ad_index, anim) in anim_instances.iter_mut().enumerate() {
                    let total_duration = ANIM_OFFSET * (anim.char_count as f32 - 1.0) + 1.0;
                    match anim.final_offset {
                        Some(final_offset) => {
                            let delta_time = current_time - anim.initial_time;
                            let norm_time = delta_time / ANIM_DURATION;
                            let current_time = norm_time * total_duration;

                            let mut delete_indices = Vec::with_capacity(anim.font_elements.len());

                            for (index, (((offset, (scale, color)), font_elem), char_index)) in anim
                                .current_offset
                                .iter_mut()
                                .zip(
                                    anim.current_scale_mult
                                        .iter_mut()
                                        .zip(anim.current_color.iter_mut()),
                                )
                                .zip(anim.font_elements.iter())
                                .zip(anim.char_indices.iter())
                                .enumerate()
                            {
                                let char_index = *char_index;
                                let char_time = current_time - (ANIM_OFFSET * char_index as f32);

                                if char_time > 1.0 {
                                    delete_indices.push(char_index);
                                }

                                let char_time = char_time.max(0.0).min(1.0);

                                {
                                    let to_box_time = ((char_time - ANIM_TO_BOX.0) / ANIM_TO_BOX.1)
                                        .max(0.0)
                                        .min(1.0);
                                    let to_box_time = easeInBack(to_box_time);

                                    let to_circle_time = ((char_time - ANIM_TO_ROTATION.0)
                                        / ANIM_TO_ROTATION.1)
                                        .max(0.0)
                                        .min(1.0);
                                    let to_circle_time = easeOutBack(to_circle_time);

                                    let to_center_fade = ((char_time - ANIM_TO_CENTER_FADE.0)
                                        / ANIM_TO_CENTER_FADE.1)
                                        .max(0.0)
                                        .min(1.0);
                                    let to_center_fade = easeInBack(to_center_fade);

                                    let font_position =
                                        font_elem.rect.position + anim.initial_offset;
                                    let rotation = current_time
                                        + anim.rand_value * 6.28
                                        + (font_elem.rect.position.x as f32) * 0.01;
                                    let rotation_offset =
                                        vec2(f32::sin(rotation), f32::cos(rotation))
                                            * container_info.rect.width()
                                            * 0.35;

                                    *offset = (final_offset - font_position) * to_box_time
                                        + rotation_offset * to_circle_time * (1.0 - to_center_fade);
                                    *scale = vec2(1.0 - to_center_fade, 1.0 - to_center_fade);
                                    *color = RGBA::WHITE.lerp_rgba(
                                        &HSLA {
                                            h: ((anim.rand_value + (char_index as f32) * 0.25) * 360.0) % 360.0,
                                            s: 0.5,
                                            l: 0.5,
                                            a: 1.0 - to_center_fade,
                                        }
                                        .into(),
                                        to_box_time,
                                    )
                                }
                            }

                            for id in delete_indices.drain(..) {
                                let index = anim.char_indices.iter().position(|ci| *ci == id);

                                if let Some(id) = index {
                                    anim.font_elements.remove(id);
                                    anim.current_offset.remove(id);
                                    anim.char_indices.remove(id);
                                    anim.current_color.remove(id);
                                }
                            }

                            if anim.font_elements.len() == 0 {
                                delete_ad_indices.push(ad_index);
                            }
                        }
                        None => {
                            //anim.final_offset = Some(container_info.rect.position);
                        }
                    }
                    anim.final_offset = Some(container_info.rect.position);
                }

                for ad_id in delete_ad_indices.drain(..) {
                    anim_instances.remove(ad_id);
                }
            }
        }

        if let UIEvent::Render {
            gui_rects,
            extra_render_steps,
        } = event
        {
            render_container_background(gui_rects, &container_info);

            if contains_instance_anim(public_data, self.uuid) {
                let word_anim = get_instance_word_anim(public_data, self.uuid);
                let anim_data = word_anim.anims.borrow();
                let mut render_elements: Vec<ElementBuilder> = Vec::new();

                for data in anim_data.iter() {
                    for (font_elem, (offset, (scale, color))) in data.font_elements.iter().zip(
                        data.current_offset
                            .iter()
                            .zip(data.current_scale_mult.iter().zip(data.current_color.iter())),
                    ) {
                        let elem = ElementBuilder::new_with_rect(
                            font_elem
                                .rect
                                .offset_position(data.initial_offset + *offset)
                                .mul_size(*scale),
                        )
                        .set_sdffont(font_elem.tx_slice.into())
                        .set_color((*color).into());
                        render_elements.push(elem);
                    }
                }

                extra_render_steps.push(
                    Box::new(move |gui_rects| {
                        for elem in render_elements.drain(..) {
                            elem.build(gui_rects);
                        }
                    }),
                    container_info.depth_range.0 + depth_offset::FONT_ANIM_OFFSET,
                );
            }
        }
    }
}
