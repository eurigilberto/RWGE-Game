use rwge::glam::Vec2;

/// All positions should be computed from the same coordinate system
pub struct DragElement {
    start_mouse_position: Vec2,
    start_element_position: Vec2,
    current_mouse_position: Vec2,
    is_draggin: bool,
}

impl DragElement {
    pub fn new() -> Self {
        Self {
            start_mouse_position: Vec2::ZERO,
            start_element_position: Vec2::ZERO,
            current_mouse_position: Vec2::ZERO,
            is_draggin: false,
        }
    }
    pub fn start_dragging(&mut self, element_position: Vec2) {
        self.start_element_position = element_position;
        self.is_draggin = true;
    }
    pub fn stop_dragging(&mut self) {
        self.start_mouse_position = self.current_mouse_position;
        self.is_draggin = false;
    }
    pub fn update_position(&mut self, current_position: Vec2) {
        if !self.is_draggin {
            self.start_mouse_position = current_position;
        }
        self.current_mouse_position = current_position;
    }
    pub fn compute_element_position(&self) -> Vec2 {
        let delta_mouse_pos = self.current_mouse_position - self.start_mouse_position;
        self.start_element_position + delta_mouse_pos
    }
}