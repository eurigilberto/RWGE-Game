#[derive(Debug)]
pub enum State {
    Inactive,
    Hovered,
    Active,
}

#[derive(Default, Clone, Copy, Debug)]
pub struct Uiid {
    id: u32,
    depth: u32,
}

impl Into<ControlId> for Uiid{
    fn into(self) -> ControlId {
        ControlId::Control(self)
    }
}

impl PartialEq for Uiid {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.depth == other.depth
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ControlId {
    Active(Uuid),
    Control(Uiid),
}

use rwge::glam::{vec2, Vec2};
use rwge::gui::rect_ui::event::UIEvent;
use rwge::gui::rect_ui::Rect;
use rwge::uuid::Uuid;

pub mod drag_element;
pub mod main_window_top_bar;

pub struct ControlState {
    current_ui_id: Option<Uiid>,

    hot: Option<Uiid>,
    hold_hover: bool,
    pub hovered: Option<Uiid>,

    hold_active: bool,
    active: Uuid,

    pub last_cursor_position: Option<Vec2>,
    pub depth_stack: Vec<u32>,
}

impl ControlState {
    pub fn new() -> Self {
        Self {
            last_cursor_position: None,

            current_ui_id: None,

            hot: None,
            hold_hover: false,
            hovered: None,

            active: Uuid::nil(),
            hold_active: false,
            depth_stack: Vec::with_capacity(25),
        }
    }

    pub fn get_id(&mut self) -> Uiid {
        self.current_ui_id
            .as_mut()
            .expect("GUI Control state was not initialized properly")
            .id += 1;
        self.current_ui_id.unwrap()
    }

    /// Returs true if hot was changed
    pub fn set_hot(&mut self, id: Uiid) -> bool {
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

    fn unset_hovered(&mut self, id: Uiid) -> bool {
        if let Some(hovered) = self.hovered {
            if hovered == id {
                self.hovered = None;
                return true
            }
        }
        return false
    }

    pub fn set_depth(&mut self, depth: u32) {
        let current_id = &mut self
            .current_ui_id
            .expect("GUI Control state was not initialized properly");
        current_id.depth = depth;
    }

    pub fn set_depth_and_save(&mut self, depth: u32) {
        self.depth_stack.push(self.get_current_depth());
        self.set_depth(depth);
    }

    pub fn restore_depth(&mut self) {
        let pop_depth = self.depth_stack.pop();
        if let Some(depth) = pop_depth {
            self.set_depth(depth);
        }
    }

    pub fn get_current_depth(&self) -> u32 {
        match self.current_ui_id {
            Some(ui_id) => ui_id.depth,
            None => 0,
        }
    }

    /// Returns true if the element is hot now
    pub fn update_hot_with_rect(&mut self, id: Uiid, control_rect: &Rect) -> bool {
        if let Some(cursor_pos) = self.last_cursor_position {
            if control_rect.inside_rect(cursor_pos) {
                self.set_hot(id)
            } else {
                self.unset_hovered(id);
                false
            }
        } else {
            self.unset_hovered(id);
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

    pub fn set_active(&mut self, id: Uiid) -> Option<Uuid> {
        if let Some(hovered) = self.hovered {
            if self.active.is_nil() && hovered == id {
                self.active = Uuid::new_v4();
                Some(self.active)
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn remove_active(&mut self, active_id: Uuid) -> Result<(), ()> {
        if self.active == active_id {
            self.active = Uuid::nil();
            Ok(())
        } else {
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
        self.current_ui_id = Some(Uiid { id: 0, depth: 0 });
    }

    pub fn on_gui_end(&mut self) -> State {
        // Remove active because it did not update its active status this frame
        assert_eq!(self.depth_stack.len(), 0, "The depth stack should be empty. If it is not empty it might inadvertently change the state of other controls.");
        if self.active.is_nil() {
            if self.hot.is_some(){
                self.hovered = self.hot;
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

    pub fn on_cursor_exit(&mut self) {
        self.active = Uuid::nil();
        self.current_ui_id = None;
        self.hot = None;
        self.hold_active = false;
        self.last_cursor_position = None;
    }

    pub fn on_after_update(&mut self) {
        if !self.hold_active {
            self.active = Uuid::nil();
        }
        if self.hot.is_some() {
            self.hold_hover = true;
        }else{
            self.hold_hover = false;
        }
        self.hot = None;
    }

    pub fn on_frame_end(&mut self) {
        if !self.hold_hover {
            self.hovered = None;
        }
    }
}
