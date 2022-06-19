use rwge::{glam::{vec2, Vec2}, math_utils::lerp_f32, color::{RGBA, HSLA}, gui::rect_ui::{event::UIEvent, element::builder::ElementBuilder}, slotmap::slotmap::Slotmap};

use crate::{public_data::{self, PublicData, EngineData}, gui_system::{gui_container::GUIContainer, ContainerInfo}};

use super::{GUIContainerSlotkey, GUIContainerInfo};

pub struct TabsContainer {
    tabs: Vec<GUIContainerSlotkey>,
    active_tab: usize,
}

impl TabsContainer {
    pub fn new(mut containers: Vec<GUIContainerSlotkey>) -> Self {
        if containers.len() == 0 {
            panic!("Cannot create a tab contianer with no guicontainers")
        };

        let mut tabs = Vec::<GUIContainerSlotkey>::new();
        tabs.extend(containers.drain(..));

        Self {
            tabs,
            active_tab: 0,
        }
    }
    pub fn handle_event(
        &mut self,
        event: &mut UIEvent,
        public_data: &PublicData,
        container_info: ContainerInfo,
        gui_container_collection: &Slotmap<Box<dyn GUIContainer>>,
    ) -> GUIContainerInfo {
        const TAB_SIZE: f32 = 30.0;

        let active_tab_key = self.tabs[self.active_tab];

        let container_position = vec2(container_info.position.x, container_info.position.y - (TAB_SIZE * 0.5) + 1.0);
        let container_size = vec2(container_info.size.x, container_info.size.y - TAB_SIZE + 2.0);

        let tab_menu_size = vec2(container_info.size.x, TAB_SIZE);
        let tab_menu_position = vec2(container_info.position.x, container_info.position.y + (container_info.size.y - TAB_SIZE) * 0.5);

        match event {
            UIEvent::Render { gui_rects } => {
                let engine_data = public_data::utils::get_engine_data(public_data);
                let screen_size = engine_data.screen_size;
                let hue = lerp_f32(0.0, 360.0, engine_data.time.sin_time(0.25) * 0.5 + 0.5);
                let color: RGBA = HSLA {
                    h: hue,
                    s: 0.75,
                    l: 0.5,
                    a: 1.0,
                }
                .into();

                ElementBuilder::new(screen_size, tab_menu_position, tab_menu_size)
                    .set_color(color.into())
                    .build(gui_rects);

                GUIContainerInfo {
                    key: active_tab_key,
                    container_info: ContainerInfo{
                        position: container_position,
                        size: container_size,
                    }
                }
            }
            _ => GUIContainerInfo {
                key: active_tab_key,
                container_info: ContainerInfo{
                    position: container_position,
                    size: container_size,
                }
            },
        }
    }
}