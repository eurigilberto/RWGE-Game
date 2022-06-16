mod test;
use std::any::{TypeId, Any};

mod window_layout;
mod gui_container;
use test::test_screen;

use rwge::{
    color::RGBA,
    engine,
    font::font_atlas::FontAtlas,
    glam::{uvec2, UVec2},
    gui::rect_ui::{
        element::{create_new_rect_element, ColoringType, MaskType},
        event::UIEvent,
        graphic::RectGraphic,
        GUIRects,
    },
    Engine,
};

use crate::{public_data::{utils::get_render_texture, PublicData}};

/// This version of the window system is only going to work with windowed spaces. This needs to be refactored in the future to support docking.
pub struct GUISystem {
    pub screen_size: UVec2,
}

impl GUISystem {
    pub fn new(screen_size: UVec2) -> Self {
        Self {
            //active_window_collection,
            //window_collection,
            screen_size,
        }
    }

    pub fn handle_event(
        &mut self,
        event: &mut UIEvent,
        public_data: &mut PublicData,
        engine: &Engine,
    ) {
        // Handle Any event FGUI
    }

    pub fn update(&mut self, public_data: &mut PublicData) {
        /* Nothing yet - The UIEvent to be sent to the GUI containers is going to be created here */
    }

    pub fn resize(&mut self, new_size: UVec2) {
        self.screen_size = new_size;
    }

    pub fn render(
        &mut self,
        engine: &Engine,
        gui_rects: &mut GUIRects,
        encoder: &mut rwge::wgpu::CommandEncoder,
        public_data: &mut PublicData,
        font_atlas_collection: &Vec<FontAtlas>,
    ) {
        gui_rects.rect_collection.clear_buffers();

        {
            test_screen(
                &engine.time,
                gui_rects,
                font_atlas_collection,
                self.screen_size,
            );
        }

        gui_rects
            .rect_collection
            .update_gpu_buffers(&engine.render_system);

        {
            let color_rt =
                get_render_texture(&public_data, &gui_rects.render_texture.color_texture_key)
                    .expect("GUI Color render target was not present");
            let mask_rt =
                get_render_texture(&public_data, &gui_rects.render_texture.mask_texture_key)
                    .expect("GUI Masking render target was not present");

            rwge::gui::rect_ui::render_pass::render_gui(
                encoder,
                &gui_rects,
                &engine.system_bind_group,
                &color_rt.texture_view,
                &mask_rt.texture_view,
            );
        }
    }
}
