#![windows_subsystem = "windows"]

use std::{collections::VecDeque, num::NonZeroU32};
mod gui_font;
mod runtime_data;
use gui_font::load_default_font_data;
use runtime_data::{utils::get_render_texture, EngineData, RuntimeData};
pub use rwge::gui::rect_ui::GUIRects;
mod gui_system;
use gui_system::{gui_container::text_animation::TextAnimationData, GUISystem};

use rwge::{
    color::*,
    font::font_load_gpu::write_font_to_gpu,
    glam::*,
    gui::rect_ui::event::UIEvent,
    graphics::copy_texture_to_surface::CopyTextureToSurface,
    graphics::{render_texture::RenderTexture, Graphics},
    slotmap::prelude::*,
    winit::window::Window,
    Engine, engine::time::Microsecond,
};
mod anymap;
mod as_any;

struct Game {
    gui_rects: GUIRects,
    gui_copy_texture_surface: CopyTextureToSurface,
    gui_system: GUISystem,
    runtime_data: RuntimeData,
    //font_atlas_collection: Vec<FontAtlas>
}

fn create_gui_copy_texture_to_surface(
    public_data: &mut runtime_data::PublicData,
    gui_rects: &GUIRects,
    engine: &Engine,
) -> CopyTextureToSurface {
    let color_rt = runtime_data::utils::get_render_texture(
        &public_data,
        &gui_rects.render_texture.color_texture_key,
    )
    .expect("GUI Color render texture not found");
    CopyTextureToSurface::new(&engine.graphics, &color_rt.texture_view)
}

impl Game {
    fn new(engine: &Engine, window: Window) -> Self {
        let size = engine.graphics.render_window.size.clone();

        let mut render_texture_slotmap = Slotmap::<RenderTexture>::with_capacity(10);

        let gui_rects = GUIRects::new(
            &engine.graphics,
            &engine.system_bind_group_layout,
            size,
            &mut render_texture_slotmap,
            8000,
        );
        let gui_system = GUISystem::new(size);

        let mut runtime_data = RuntimeData::new();
        runtime_data.insert_pub(render_texture_slotmap);

        let engine_data = EngineData::new_from_engine(engine);
        runtime_data.insert_pub(engine_data);

        runtime_data.insert_pub(window);

        let default_fonts = load_default_font_data();
        let font_slices = write_font_to_gpu(
            &engine.graphics.queue,
            &gui_rects.texture_atlas.texture,
            &default_fonts,
            uvec2(1024, 1024),
            0,
        )
        .unwrap();

        let mut font_collections = Vec::new();
        font_collections.push(font_slices);

        runtime_data.insert_pub(font_collections);

        let gui_copy_texture_surface =
            create_gui_copy_texture_to_surface(&mut runtime_data.public_data, &gui_rects, engine);

        runtime_data.insert_pub(TextAnimationData::new());

        Self {
            gui_rects,
            gui_system,
            runtime_data,
            gui_copy_texture_surface,
            //font_atlas_collection
        }
    }
}

impl rwge::Runtime for Game {
    fn frame_start(&mut self, engine: &Engine) {
        runtime_data::utils::update_engine_time(&mut self.runtime_data.public_data, &engine);
    }

    fn handle_event_queue<F>(
        &mut self,
        event_queue: &VecDeque<rwge::EngineEvent>,
        engine: &mut Engine,
        exit_event_loop: &mut F,
    ) where
        F: FnMut() -> (),
    {
        for event in event_queue {
            let close_event_handled = rwge::default_close_event_handler(event, exit_event_loop);

            if !close_event_handled {
                let size_event = Graphics::resize_event_transformation(event);
                if let Some(new_size) = size_event {
                    //Resize event
                    runtime_data::utils::update_screen_size(&mut self.runtime_data.public_data, new_size);

                    {
                        //Update GUI texture size
                        let rt_slotmap = self
                            .runtime_data
                            .public_data
                            .collection
                            .get_mut::<Slotmap<RenderTexture>>()
                            .expect("Render texture slotmap not found");

                        self.gui_rects
                            .resize(new_size, &engine.graphics, rt_slotmap);
                    }
                    
                    engine.graphics.resize(new_size);

                    let mut resize_ui_event = UIEvent::Resize(new_size);
                    self.gui_system
                        .handle_event(&mut resize_ui_event, &mut self.runtime_data.public_data);
                } else {
                    let gui_event = rwge::gui::rect_ui::event::default_event_transformation(
                        event,
                        engine.graphics.render_window.size,
                    );
                    if let Some(mut e) = gui_event {
                        self.gui_system.handle_event(&mut e, &mut self.runtime_data.public_data);
                    }
                }
            }
        }
    }

    fn update(&mut self, engine: &rwge::Engine, exit_event_loop: &mut dyn FnMut() -> ()) {
        self.gui_system.update(&self.runtime_data.public_data);
    }

    fn render(
        &mut self,
        engine: &rwge::Engine,
        screen_view: &rwge::wgpu::TextureView,
        encoder: &mut rwge::wgpu::CommandEncoder,
    ) {
        ///////// APPLY PUBLIC DATA CHANGES
        self.runtime_data.apply_pub_mut();
        ///////// APPLY PUBLIC DATA CHANGES

        rwge::graphics::texture::clear_render_targets(
            encoder,
            screen_view,
            RGBA::rrr1((0.12 as f32).powf(2.2)).into(),
            None,
            None,
            None,
        );

        self.gui_system
            .render(engine, &mut self.gui_rects, encoder, &mut self.runtime_data.public_data);

        let color_rt = get_render_texture(
            &self.runtime_data.public_data,
            &self.gui_rects.render_texture.color_texture_key,
        )
        .unwrap();

        self.gui_copy_texture_surface.render(
            encoder,
            &engine.graphics.device,
            screen_view,
            &color_rt.texture_view,
        );
    }

    fn frame_end<F>(&mut self, engine: &mut rwge::Engine, exit_event_loop: &mut F)
    where
        F: FnMut() -> (),
    {
        self.gui_system
            .window_layouting
            .control_state
            .on_frame_end();
        engine.graphics.destroy_queued_textures();
    }

    fn before_exit(&mut self, engine: &rwge::Engine) {
        println!("Before exit log")
    }

    fn get_window_id(&self) -> rwge::winit::window::WindowId {
        self.runtime_data
            .public_data
            .collection
            .get::<rwge::winit::window::Window>()
            .unwrap()
            .id()
    }
}

fn main() {
    let event_loop = rwge::winit::event_loop::EventLoop::new();

    let window = rwge::winit::window::WindowBuilder::new()
        .with_inner_size(rwge::winit::dpi::LogicalSize::<f32>::new(1128.0, 740.0))
        .with_decorations(false)
        .with_resizable(true)
        //.with_transparent(true)
        .with_maximized(true)
        .build(&event_loop)
        .expect("Window could not be created");

    let engine = Engine::new(&window, Microsecond(16666));

    let game = Game::new(&engine, window);

    rwge::start_engine_loop(engine, game, event_loop);
}
