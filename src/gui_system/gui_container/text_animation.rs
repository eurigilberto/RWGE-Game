use rwge::gui::rect_ui::event::UIEvent;

use super::{render_container_background, GUIContainer};



pub struct TextAnimation {
    first_update: bool,
}

impl TextAnimation {
    pub fn new() -> Self {
        Self {
            first_update: false,
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
        instance_index: usize,
    ) {

        //initialize public data struct
        if self.first_update == false {
            if let UIEvent::Update = event {
                public_data.push_mut(Box::new(|public_data|{
                    public_data.contains()
                }));
                self.first_update = true;
            }
        }

        if let UIEvent::Render { gui_rects, .. } = event {
            render_container_background(gui_rects, &container_info);
        }
    }
}
