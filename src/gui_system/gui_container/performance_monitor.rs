use rwge::{
    color::RGBA,
    font::{font_layout::create_font_layout, font_load_gpu::FontCollection},
    glam::{vec2, Vec2},
    gui::rect_ui::{
        element::{builder::ElementBuilder, Border, Element},
        BorderRadius, GUIRects, Rect,
    },
};

use crate::{
    gui_system::gui_container::render_container_background,
    public_data::{utils::get_engine_data, PublicData},
};

use super::GUIContainer;

pub struct PerformanceMonitor {
    average_frame_times: [f32; 10],
    end_index: u32,
    frame_time_data: [f32; 10],
    last_update_frame_time: u32,
}

impl PerformanceMonitor {
    pub fn new() -> Self {
        Self {
            end_index: 0,
            average_frame_times: [0.0; 10],
            frame_time_data: [0.0; 10],
            last_update_frame_time: 0,
        }
    }

    fn compute_average(&mut self) -> f32 {
        let f_sum: f32 = self.frame_time_data.iter().sum();
        f_sum / 10.0
    }

    pub fn update_frame_data(&mut self, public_data: &PublicData) {
        let frame_count = get_engine_data(public_data).time.frame_count;

        if self.last_update_frame_time != frame_count {
            //Guard against mutliple instances of this window calling the fame updte on the same frame

            let frame_time_millis = get_engine_data(public_data).operation_time.get_total_time();
            let data_index = frame_count % 10;
            self.frame_time_data[data_index as usize] = frame_time_millis as f32;

            if data_index == 9 {
                self.average_frame_times[self.end_index as usize] = self.compute_average();
                self.end_index = (self.end_index + 1) % 10;
            }

            self.last_update_frame_time = frame_count;
        }
    }

    pub fn get_frame_averages(&self) -> [f32; 10] {
        let mut frame_averages = [0.0; 10];

        for i in 0..10 {
            let start_index = self.end_index + 1;
            let avg_index = (start_index + i) % 10;
            frame_averages[i as usize] = self.average_frame_times[avg_index as usize];
        }

        frame_averages
    }
}

impl GUIContainer for PerformanceMonitor {
    fn get_name(&self) -> &str {
        "Perf Monitor"
    }

    fn handle_event(
        &mut self,
        event: &mut rwge::gui::rect_ui::event::UIEvent,
        public_data: &crate::public_data::PublicData,
        container_info: crate::gui_system::ContainerInfo,
        control_state: &mut crate::gui_system::control::ControlState,
        instance_index: usize,
    ) {
        const COLUMN_HEIGHT: f32 = 100.0;
        const MIN_COLUMN_WIDTH: f32 = 20.0;
        const BOX_MARGIN: f32 = 5.0;
        const MARGIN: f32 = 10.0;
        const GAP: f32 = 5.0;
        const MIN_BOX_WIDTH: f32 =
            BOX_MARGIN + MIN_COLUMN_WIDTH + GAP + MIN_COLUMN_WIDTH + BOX_MARGIN;

        match event {
            rwge::gui::rect_ui::event::UIEvent::Update => {
                self.update_frame_data(public_data);
            }
            rwge::gui::rect_ui::event::UIEvent::Render {
                gui_rects,
                extra_render_steps,
            } => {
                render_container_background(gui_rects, &container_info);

                let frame_averages = self.get_frame_averages();

                let max_frame_avg = *frame_averages
                    .iter()
                    .reduce(|acc, avg| if acc > avg { acc } else { avg })
                    .unwrap();

                let mut box_width = container_info.rect.size.x - (MARGIN * 2.0);
                box_width = box_width.max(MIN_BOX_WIDTH);

                let top_left_box_pos =
                    container_info.get_top_left_position() + vec2(MARGIN, -MARGIN);

                let box_center =
                    top_left_box_pos + vec2(box_width * 0.5, -(COLUMN_HEIGHT * 0.5 + BOX_MARGIN));
                let box_size = vec2(box_width, COLUMN_HEIGHT + BOX_MARGIN * 2.0);

                ElementBuilder::new(box_center, box_size)
                    .set_round_rect(BorderRadius::ForAll(5.0).into())
                    .set_color(RGBA::rrr1(0.1).into())
                    .set_rect_mask(container_info.rect.into())
                    .set_border(Some(Border {
                        size: 1,
                        color: RGBA::rrr1(0.75).into(),
                    }))
                    .build(gui_rects);

                //FONT RENDERING
                let top_left_box_pos = top_left_box_pos + vec2(12.0, -12.0);

                ElementBuilder::new(top_left_box_pos, vec2(4.0, 4.0))
                    .set_color(RGBA::RED.into())
                    .build(gui_rects);

                let font_collection =
                    &public_data.collection.get::<Vec<FontCollection>>().unwrap()[0];

                let mask_rect = Rect {
                    position: box_center,
                    size: box_size,
                };
                let mask_rect = container_info.rect.combine_rects(&mask_rect);

                if let Some(mask) = mask_rect {
                    render_unbounded_horizontal_text(
                        &font_collection,
                        1,
                        24.0,
                        "Euri gjiIytf",
                        top_left_box_pos,
                        gui_rects,
                        &mut |elem_builder| {
                            elem_builder
                                .set_color(RGBA::WHITE.into())
                                .set_rect_mask(mask.into())
                        },
                    );

                    render_unbounded_horizontal_text(
                        &font_collection,
                        0,
                        24.0,
                        "Euri '@#$%^&*()_+-/*.' <scape> gjiIytf",
                        top_left_box_pos + vec2(0.0, -32.0),
                        gui_rects,
                        &mut |elem_builder| {
                            elem_builder
                                .set_color(RGBA::WHITE.into())
                                .set_rect_mask(mask.into())
                        },
                    );

                    render_unbounded_horizontal_text(
                        &font_collection,
                        2,
                        24.0,
                        "Euri gjiIytf",
                        top_left_box_pos + vec2(0.0, -64.0),
                        gui_rects,
                        &mut |elem_builder| {
                            elem_builder
                                .set_color(RGBA::WHITE.into())
                                .set_rect_mask(mask.into())
                        },
                    );
                }
            }
            _ => {}
        }
    }
}

fn render_unbounded_horizontal_text(
    font_collection: &FontCollection,
    collection_index: usize,
    font_size: f32,
    text: &str,
    top_left_pos: Vec2,
    gui_rects: &mut GUIRects,
    render_char: &mut dyn FnMut(ElementBuilder) -> ElementBuilder,
) {
    let (text_layout, rect) =
        create_font_layout(text, font_size, font_collection, collection_index);
    let top_left_pos = top_left_pos + vec2(0.0, -rect.size.y);
    for text_elem in text_layout {
        let t_pos = top_left_pos + text_elem.position;
        let t_size = text_elem.size;

        let char_builder =
            ElementBuilder::new(t_pos, t_size).set_sdffont(text_elem.tx_slice.into());
        render_char(char_builder).build(gui_rects);
    }

    //Debug
    /*ElementBuilder::new(top_left_pos + rect.position, rect.size)
    .set_color(RGBA::TRANSPARENT.into())
    .set_border(Some(Border {
        size: 2,
        color: RGBA::GREEN.into(),
    }))
    .build(gui_rects);*/
}
