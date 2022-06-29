use rwge::uuid::Uuid;

use crate::gui_system::window_layout::tabs_container;

use super::Orientation;

pub struct ActiveDivider {
    pub active_id: Uuid,
    pub index: usize,
    pub drag_divider: LayoutDragDivider,
}

impl ActiveDivider {
    pub fn new(
        active_id: Uuid,
        index: usize,
        div_data: [DivData; 2],
        start_cursor_position: f32,
    ) -> Self {
        Self {
            active_id,
            index,
            drag_divider: LayoutDragDivider::new(div_data, start_cursor_position),
        }
    }
}

pub struct DivData {
    pub div_size: f32,
    pub div_px_size: f32,
}

pub struct LayoutDragDivider {
    div_data: [DivData; 2],
    start_cursor_position: f32,
    current_cursor_position: f32,
    total_px_size: f32,
    total_div_size: f32,
}

impl LayoutDragDivider {
    pub fn new(div_data: [DivData; 2], start_cursor_position: f32) -> Self {
        let total_px_size = div_data[0].div_px_size + div_data[1].div_px_size;
        let total_div_size = div_data[0].div_size + div_data[1].div_size;
        Self {
            div_data,
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
                .min(self.div_data[0].div_px_size - tabs_container::TAB_SIZE);
            (
                self.div_data[0].div_px_size - clamped_movement,
                self.div_data[1].div_px_size + clamped_movement,
            )
        } else {
            let clamped_movement = cursor_movement
                .max(0.0)
                .min(self.div_data[1].div_px_size - tabs_container::TAB_SIZE);
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