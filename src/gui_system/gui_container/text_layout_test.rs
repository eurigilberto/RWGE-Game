use super::GUIContainer;

pub struct TextLayoutTest{

}

impl TextLayoutTest{
	pub fn new()->Self{
		Self {  }
	}
}

impl GUIContainer for TextLayoutTest{
    fn get_name(&self) -> &str {
        "Text Layout"
    }

    fn handle_event(
        &mut self,
        event: &mut rwge::gui::rect_ui::event::UIEvent,
        public_data: &crate::public_data::PublicData,
        container_info: crate::gui_system::ContainerInfo,
        control_state: &mut crate::gui_system::control::ControlState,
        instance_index: usize
    ) {
        
    }
}