use rwge::{glam::{vec2, Vec2}, math_utils::lerp_f32, color::{RGBA, HSLA}, gui::rect_ui::{event::UIEvent, element::builder::ElementBuilder, RectMask, BorderRadius}, slotmap::slotmap::Slotmap};

use crate::{public_data::{self, PublicData, EngineData}, gui_system::{gui_container::GUIContainer, ContainerInfo, control::ControlState}};

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
        control_state: &mut ControlState
    ) -> GUIContainerInfo {
        const TAB_SIZE: f32 = 30.0;

        let active_tab_key = self.tabs[self.active_tab];

        let container_position = vec2(container_info.position.x, container_info.position.y - (TAB_SIZE * 0.5));
        let container_size = vec2(container_info.size.x, container_info.size.y - TAB_SIZE);

        let tab_menu_size = vec2(container_info.size.x, TAB_SIZE);
        let tab_menu_position = vec2(container_info.position.x, container_info.position.y + (container_info.size.y - TAB_SIZE) * 0.5);

        match event {
            UIEvent::Render { gui_rects } => {
                let engine_data = public_data::utils::get_engine_data(public_data);
                let screen_size = engine_data.screen_size;
                
                let color:RGBA = RGBA::rrr1(0.30).into();

                let rect_mask = RectMask{
                    position: tab_menu_position,
                    size: tab_menu_size,
                };

                ElementBuilder::new(screen_size, tab_menu_position, tab_menu_size)
                    .set_color(color.into())
                    .build(gui_rects);

                const TAB_WIDTH: f32 = 100.0;
                const TAB_GAP: f32 = 5.0;
                let left_position = tab_menu_position - vec2(tab_menu_size.x * 0.5, 0.0);
                let mut current_pos = left_position;
                for index in 0..self.tabs.len() {
                    let mut tab_btn_pos = current_pos + vec2(TAB_GAP + TAB_WIDTH * 0.5 , 0.0);
                    let mut tab_btn_size = vec2(TAB_WIDTH, tab_menu_size.y - TAB_GAP * 2.0);
                    current_pos += vec2(TAB_GAP + TAB_WIDTH, 0.0);

                    let (round_rect, color) = if index == self.active_tab {
                        tab_btn_pos -= vec2(0.0, TAB_GAP * 0.5);
                        tab_btn_size += vec2(0.0, TAB_GAP);
                        (
                            BorderRadius::ForTopBottom { top: tab_menu_size.y * 0.5 - TAB_GAP, bottom: 0.0 },
                            RGBA::rrr1(0.55)
                        )
                    }else{
                        (
                            BorderRadius::ForAll(tab_menu_size.y * 0.5 - TAB_GAP),
                            RGBA::rrr1(0.40)
                        )
                    };

                    ElementBuilder::new(screen_size, tab_btn_pos, tab_btn_size)
                        .set_round_rect(round_rect.into())
                        .set_color(color.into())
                        .set_rect_mask(rect_mask.into())
                        .build(gui_rects);
                }

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