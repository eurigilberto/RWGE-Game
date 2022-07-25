use rwge::{
    color::*,
    font::{font_layout::create_single_line, font_load_gpu::FontCollection},
    glam::{vec2, Vec2},
    gui::rect_ui::{
        element::{
            builder::ElementBuilder, push_color, push_linear_gradient, push_rect_mask, Border,
            Element, LinearGradient,
        },
        event::UIEvent,
        BorderRadius, GUIRects, Rect,
    },
};

use crate::{
    gui_system::gui_container::render_container_background,
    runtime_data::{utils::get_engine_data, RuntimeData, PublicData},
};

use super::GUIContainer;

pub struct AverageTimer {
    pub average_times: [f32; 10],
    end_index: usize,
    time_data: [f32; 60],
    data_index: usize,
    most_recent: usize,
}

impl AverageTimer {
    pub fn new() -> Self {
        Self {
            end_index: 0,
            most_recent: 0,
            average_times: [0.0; 10],
            time_data: [0.0; 60],
            data_index: 0,
        }
    }

    fn compute_average(&mut self) -> f32 {
        let f_sum: f32 = self.time_data.iter().sum();
        f_sum / 60.0
    }

    pub fn get_most_recent(&self) -> f32 {
        self.average_times[self.most_recent]
    }

    pub fn get_most_recent_two_dec(&self) -> f32 {
        (self.get_most_recent() * 100.0).round() / 100.0
    }

    pub fn update_frame_data(&mut self, time: f32) {
        //Guard against mutliple instances of this window calling the fame updte on the same frame

        let new_time = time;

        self.data_index += 1;
        self.data_index %= 60;
        self.time_data[self.data_index] = new_time;

        if self.data_index == 59 {
            self.average_times[self.end_index] = self.compute_average();
            self.most_recent = self.end_index;
            self.end_index = (self.end_index + 1) % 10;
        }
    }

    pub fn get_average_times(&self) -> [f32; 10] {
        let mut times = [0.0; 10];
        for i in 0..10 {
            let index = (self.end_index + i) % 10;
            times[i] = self.average_times[index];
        }
        times
    }
}

pub struct PerformanceMonitor {
    frame_timer: AverageTimer,
    render_timer: AverageTimer,
    cpu_timer: AverageTimer,
    gpu_lock_time: AverageTimer,
}

impl PerformanceMonitor {
    pub fn new() -> Self {
        Self {
            frame_timer: AverageTimer::new(),
            render_timer: AverageTimer::new(),
            cpu_timer: AverageTimer::new(),
            gpu_lock_time: AverageTimer::new(),
        }
    }
}

impl GUIContainer for PerformanceMonitor {
    fn get_name(&self) -> &str {
        "Perf Monitor"
    }

    fn handle_event(
        &mut self,
        event: &mut rwge::gui::rect_ui::event::UIEvent,
        public_data: &PublicData,
        container_info: crate::gui_system::ContainerInfo,
        control_state: &mut crate::gui_system::control::ControlState,
    ) {
        const COLUMN_HEIGHT: f32 = 100.0;
        const MIN_COLUMN_WIDTH: f32 = 20.0;
        const BOX_MARGIN: f32 = 10.0;
        const MARGIN: f32 = 10.0;
        const GAP: f32 = 5.0;
        const MIN_BOX_WIDTH: f32 =
            BOX_MARGIN + MIN_COLUMN_WIDTH + GAP + MIN_COLUMN_WIDTH + BOX_MARGIN;
        const MAX_BOX_WIDTH: f32 = BOX_MARGIN + MIN_COLUMN_WIDTH * 12.0 + GAP * 9.0 + BOX_MARGIN;

        const CHAR_SPACING: f32 = 0.05;

        match event {
            rwge::gui::rect_ui::event::UIEvent::Update => {
                let op_time = &get_engine_data(public_data).operation_time;
                self.render_timer
                    .update_frame_data(op_time.render_time.as_millisecond().0);
                self.frame_timer
                    .update_frame_data(op_time.get_total_time().as_millisecond().0);
                self.cpu_timer.update_frame_data(
                    (op_time.update_time + op_time.event_handling_time).as_millisecond().0,
                );
                self.gpu_lock_time
                    .update_frame_data(op_time.gpu_lock_time.as_millisecond().0);
            }
            rwge::gui::rect_ui::event::UIEvent::Render {
                gui_rects,
                extra_render_steps,
            } => {
                render_container_background(gui_rects, &container_info);

                let mut top_left_box_pos =
                    container_info.top_left_position() + vec2(MARGIN, -MARGIN);

                let font_collection =
                    &public_data.get::<Vec<FontCollection>>().unwrap()[0];

                let rect_mask_data_index = push_rect_mask(container_info.rect, gui_rects) as u16;

                let (font_elems, font_rect) =
                    create_single_line("Average Frame Time (ms)", 18.0, font_collection, 0, CHAR_SPACING);

                for elem in font_elems {
                    let char_rect = elem
                        .rect
                        .offset_position(top_left_box_pos)
                        .offset_position(-vec2(0.0, font_rect.height()));
                    ElementBuilder::new_with_rect(char_rect)
                        .set_sdffont(elem.tx_slice.into())
                        .set_rect_mask(rect_mask_data_index.into())
                        .build(gui_rects);
                }

                top_left_box_pos.x += MARGIN;
                top_left_box_pos.y -= MARGIN + font_rect.height();

                let color_index = push_color(gui_rects, RGBA::rrr1(0.75)) as u16;

                {
                    let current_avg = self.render_timer.get_most_recent_two_dec();
                    let (font_elems, font_rect) = create_single_line(
                        format!("Average computed over 60 frames",).as_str(),
                        14.0,
                        font_collection,
                        1,
                        CHAR_SPACING,
                    );

                    for elem in font_elems {
                        let char_rect = elem
                            .rect
                            .offset_position(top_left_box_pos)
                            .offset_position(-vec2(0.0, font_rect.height()));
                        ElementBuilder::new_with_rect(char_rect)
                            .set_sdffont(elem.tx_slice.into())
                            .set_rect_mask(rect_mask_data_index.into())
                            .build(gui_rects);
                    }

                    top_left_box_pos.y -= MARGIN + font_rect.height();
                }

                {
                    let current_avg = self.render_timer.get_most_recent_two_dec();
                    let (font_elems, font_rect) = create_single_line(
                        format!("Render Avg. {current_avg} (ms)",).as_str(),
                        18.0,
                        font_collection,
                        0,
                        CHAR_SPACING,
                    );

                    for elem in font_elems {
                        let char_rect = elem
                            .rect
                            .offset_position(top_left_box_pos)
                            .offset_position(-vec2(0.0, font_rect.height()));
                        ElementBuilder::new_with_rect(char_rect)
                            .set_sdffont(elem.tx_slice.into())
                            .set_rect_mask(rect_mask_data_index.into())
                            .set_color(color_index.into())
                            .build(gui_rects);
                    }

                    top_left_box_pos.y -= MARGIN + font_rect.height();
                }

                {
                    let current_avg = self.gpu_lock_time.get_most_recent_two_dec();
                    let (font_elems, font_rect) = create_single_line(
                        format!("GPU lock Avg. {current_avg} (ms)",).as_str(),
                        18.0,
                        font_collection,
                        0,
                        CHAR_SPACING,
                    );

                    for elem in font_elems {
                        let char_rect = elem
                            .rect
                            .offset_position(top_left_box_pos)
                            .offset_position(-vec2(0.0, font_rect.height()));
                        ElementBuilder::new_with_rect(char_rect)
                            .set_sdffont(elem.tx_slice.into())
                            .set_rect_mask(rect_mask_data_index.into())
                            .set_color(color_index.into())
                            .build(gui_rects);
                    }

                    top_left_box_pos.y -= MARGIN + font_rect.height();
                }

                {
                    let current_avg = self.cpu_timer.get_most_recent_two_dec();
                    let (font_elems, font_rect) = create_single_line(
                        format!("Update + W Event Avg. {current_avg} (ms)",).as_str(),
                        18.0,
                        font_collection,
                        0,
                        CHAR_SPACING,
                    );

                    for elem in font_elems {
                        let char_rect = elem
                            .rect
                            .offset_position(top_left_box_pos)
                            .offset_position(-vec2(0.0, font_rect.height()));
                        ElementBuilder::new_with_rect(char_rect)
                            .set_sdffont(elem.tx_slice.into())
                            .set_rect_mask(rect_mask_data_index.into())
                            .set_color(color_index.into())
                            .build(gui_rects);
                    }

                    top_left_box_pos.y -= MARGIN + font_rect.height();
                }

                {
                    top_left_box_pos.y -= MARGIN;
                    let current_avg = self.frame_timer.get_most_recent_two_dec();
                    let (font_elems, font_rect) = create_single_line(
                        format!("Total frame time avg. {current_avg} (ms)",).as_str(),
                        20.0,
                        font_collection,
                        1,
                        CHAR_SPACING,
                    );

                    for elem in font_elems {
                        let char_rect = elem
                            .rect
                            .offset_position(top_left_box_pos)
                            .offset_position(-vec2(0.0, font_rect.height()));
                        ElementBuilder::new_with_rect(char_rect)
                            .set_sdffont(elem.tx_slice.into())
                            .set_rect_mask(rect_mask_data_index.into())
                            .build(gui_rects);
                    }

                    top_left_box_pos.y -= MARGIN + font_rect.height();
                }

                top_left_box_pos.x -= MARGIN;

                let mut box_width = container_info.rect.size.x - (MARGIN * 2.0);
                box_width = box_width.max(MIN_BOX_WIDTH).min(MAX_BOX_WIDTH);

                let box_center =
                    top_left_box_pos + vec2(box_width * 0.5, -(COLUMN_HEIGHT * 0.5 + BOX_MARGIN));
                let box_size = vec2(box_width, COLUMN_HEIGHT + BOX_MARGIN * 2.0);

                let box_rect = Rect {
                    position: box_center,
                    size: box_size,
                };

                ElementBuilder::new_with_rect(box_rect)
                    .set_round_rect(BorderRadius::ForAll(5.0).into())
                    .set_color(RGBA::rrr1(0.1).into())
                    .set_rect_mask(container_info.rect.into())
                    .set_border(Some(Border {
                        size: 1,
                        color: RGBA::rrr1(0.75).into(),
                    }))
                    .build(gui_rects);

                let bars_rect = box_rect;
                let bars_rect = bars_rect.offset_size(-Vec2::splat(BOX_MARGIN * 2.00));

                let mask_rect = box_rect;
                let mask_rect = container_info.rect.combine_rects(&mask_rect);

                let avg_iter = self.frame_timer.get_average_times();
                bar_graph(bars_rect, &avg_iter, event, mask_rect.unwrap_or_default());
            }
            _ => {}
        }
    }
}

fn bar_graph(rect: Rect, values: &[f32], event: &mut UIEvent, mask: Rect) {
    const MIN_BOX_WIDTH: f32 = 10.0;
    const GAP_SIZE: f32 = 5.0;

    //Return early if the component does not handle the current event
    match event {
        UIEvent::Render { .. } => {}
        _ => return,
    }

    //min box width = mbw
    //bar cound = bc
    //bar gap width = bgw

    let mbw = MIN_BOX_WIDTH + GAP_SIZE;
    let bc = rect.width() / mbw;
    let bc = bc.floor().min(values.len() as f32);

    let bw = (rect.width() - (bc - 1.0) * GAP_SIZE) / bc;

    let test_iter = values.iter();

    let max_value = values
        .iter()
        .reduce(|acc, next| if acc < next { next } else { acc })
        .unwrap();

    if *max_value > 0.0 && bc > 0.0 {
        let start_index = values.len() - bc as usize;
        let mut horizontal_pos = 0.0;
        let bot_left_pos = rect.bottom_left_position();

        for val in values[start_index..].iter() {
            match event {
                UIEvent::Render { gui_rects, .. } => {
                    let vertical_size = val / max_value;
                    let height = rect.height() * vertical_size;
                    let bar_rect = Rect {
                        position: bot_left_pos + vec2(horizontal_pos + bw * 0.5, height * 0.5),
                        size: vec2(bw, height),
                    };
                    horizontal_pos += bw + GAP_SIZE;
                    ElementBuilder::new_with_rect(bar_rect)
                        .set_rect_mask(mask.into())
                        .set_linear_gradient(
                            LinearGradient {
                                colors: [RGBA::WHITE, RGBA::rgb(0.95, 0.35, 0.2)],
                                start_position: vec2(0.0, -bar_rect.size.y * 0.5),
                                end_position: vec2(0.0, bar_rect.size.y * 0.5),
                            }
                            .into(),
                        )
                        .set_round_rect(
                            BorderRadius::ForTopBottom {
                                top: bw * 0.5,
                                bottom: bw * 0.15,
                            }
                            .into(),
                        )
                        .build(gui_rects);
                }
                _ => {}
            }
        }
    }
}
