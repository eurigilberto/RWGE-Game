pub enum State {
    Inactive,
    Hovered,
    Active,
}

#[derive(PartialEq, Default, Clone, Copy)]
pub struct Uiid {
    id: u32,
    depth: u32,
}

pub enum ControlId {
    Active(Uuid),
    Control(Uiid),
}

use rwge::glam::{Vec2, vec2};
use rwge::gui::rect_ui::event::UIEvent;
use rwge::gui::rect_ui::RectMask;
use rwge::uuid::Uuid;

pub struct ControlState {
    current_ui_id: Option<Uiid>,

    hot: Option<Uiid>,
    pub hovered: Option<Uiid>,

    pub max_depth: u32,

    hold_active: bool,
    active: Uuid,

    pub last_cursor_position: Vec2,
}

impl ControlState {
    pub fn new() -> Self {
        Self {
            last_cursor_position: Vec2::ZERO,

            current_ui_id: None,

            hot: None,
            hovered: None,
            max_depth: 0,

            active: Uuid::nil(),
            hold_active: false,
        }
    }

    pub fn get_id(&mut self) -> Uiid {
        self.current_ui_id
            .expect("GUI Control state was not initialized properly")
            .id += 1;
        self.current_ui_id.unwrap()
    }

    /// Returs true if hot was changed
    fn set_hot(&mut self, id: Uiid) -> bool {
        if self.active.is_nil() {
            if let Some(hot) = &mut self.hot {
                if hot.depth <= id.depth {
                    *hot = id;
                    return true;
                }
            } else {
                self.hot = Some(id);
                return true;
            }
        }
        return false;
    }

    pub fn increase_depth(&mut self) {
        let current_id = &mut self
            .current_ui_id
            .expect("GUI Control state was not initialized properly");
        current_id.depth += 1;
        if self.max_depth < current_id.depth {
            self.max_depth = current_id.depth;
        }
    }
    pub fn reduce_depth(&mut self) {
        self.current_ui_id
            .expect("GUI Control state was not initialized properly")
            .depth -= 1;
    }

    /// Returns true if the element is hot now
    pub fn update_hot_hovered(&mut self, id: Uiid, control_rect: &RectMask) -> bool {
        //self.mouse_position
        if control_rect.inside_rect(self.last_cursor_position) {
            self.set_hot(id)
        } else {
            if let Some(ref hovered) = self.hovered {
                if *hovered == id {
                    self.hovered = None;
                }
            }
            false
        }
    }

    pub fn get_control_state(&self, id: ControlId) -> State {
        match id {
            ControlId::Active(id) => {
                if self.active == id {
                    State::Active
                } else {
                    State::Inactive
                }
            }
            ControlId::Control(id) => {
                if let Some(ref hovered) = self.hovered {
                    if *hovered == id {
                        State::Hovered
                    } else {
                        State::Inactive
                    }
                } else {
                    State::Inactive
                }
            }
        }
    }

    pub fn set_active(&mut self, id: Uiid) -> Option<Uuid>{
        if let Some(hovered) = self.hovered{
            if self.active.is_nil() && hovered == id {
                self.active = Uuid::new_v4();
                Some(self.active)
            }else{
                None
            }
        }else{
            None
        }
    }

    pub fn remove_active(&mut self, active_id: Uuid) -> Result<(),()>{
        if self.active == active_id {
            self.active = Uuid::nil();
            Ok(())
        }else{
            Err(())
        }
    }

    pub fn hold_active_state(&mut self, active_id: Uuid) {
        if self.active == active_id {
            self.hold_active = true;
        }
    }

    /// Called at the start of the frame
    pub fn on_gui_start(&mut self) {
        self.hold_active = false;
        self.current_ui_id = Some(Uiid::default());
    }

    pub fn on_gui_end(&mut self) -> State {
        // Remove active because it did not update its active status this frame

        if self.active.is_nil() {
            if self.hot.is_some() {
                self.hovered = self.hot;
                self.hot = None;
            }

            if !self.active.is_nil() {
                State::Active
            } else if self.hovered.is_some() {
                State::Hovered
            } else {
                State::Inactive
            }
        } else {
            self.hovered = None;
            State::Inactive
        }
    }

    pub fn on_cursor_exit(&mut self){
        self.active = Uuid::nil();
        self.current_ui_id = None;
        self.hot = None;
        self.hold_active = false;
        self.max_depth = 0;
        self.last_cursor_position = vec2(-1.0, -1.0);
    }

    pub fn on_frame_end(&mut self){
        if !self.hold_active {
            self.active = Uuid::nil();
        }
    }
}
