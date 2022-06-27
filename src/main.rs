use std::{collections::VecDeque, num::NonZeroU32};
mod gui_font;
mod public_data;
use gui_font::load_default_font_data;
use public_data::{EngineData, PublicData};
pub use rwge::gui::rect_ui::GUIRects;
mod gui_system;
use gui_system::GUISystem;

use rwge::{
    color::RGBA,
    font::font_load_gpu::write_font_to_gpu,
    glam::uvec2,
    gui::rect_ui::event::UIEvent,
    render_system::copy_texture_to_surface::CopyTextureToSurface,
    render_system::{render_texture::RenderTexture, RenderSystem},
    slotmap::slotmap::Slotmap,
    winit::window::Window,
    Engine,
};
mod anymap;
mod as_any;

struct Game {
    gui_rects: GUIRects,
    gui_copy_texture_surface: CopyTextureToSurface,
    gui_system: GUISystem,
    public_data: PublicData,
    //font_atlas_collection: Vec<FontAtlas>
}

fn create_gui_copy_texture_to_surface(
    public_data: &mut PublicData,
    gui_rects: &GUIRects,
    engine: &Engine,
) -> CopyTextureToSurface {
    let color_rt = public_data::utils::get_render_texture(
        &public_data,
        &gui_rects.render_texture.color_texture_key,
    )
    .expect("GUI Color render texture not found");
    CopyTextureToSurface::new(&engine.render_system, &color_rt.texture_view)
}

impl Game {
    fn new(engine: &Engine, window: Window) -> Self {
        let size = engine.render_system.render_window.size.clone();

        let mut render_texture_slotmap = Slotmap::<RenderTexture>::new_with_capacity(10);

        let gui_rects = GUIRects::new(
            &engine.render_system,
            &engine.system_bind_group_layout,
            size,
            &mut render_texture_slotmap,
            4000,
        );
        let gui_system = GUISystem::new(size);

        let mut public_data = PublicData::new();
        public_data.collection.insert(render_texture_slotmap);

        let engine_data = EngineData::new_from_engine(engine);
        public_data.collection.insert(engine_data);

        public_data.collection.insert(window);

        let default_fonts = load_default_font_data();
        let font_slices = write_font_to_gpu(
            &engine.render_system.render_window.queue,
            &gui_rects.texture_atlas.texture,
            &default_fonts,
            uvec2(1024, 1024),
            0,
        )
        .unwrap();

        let mut font_collections = Vec::new();
        font_collections.push(font_slices);

        public_data.collection.insert(font_collections);

        let gui_copy_texture_surface =
            create_gui_copy_texture_to_surface(&mut public_data, &gui_rects, engine);

        Self {
            gui_rects,
            gui_system,
            public_data,
            gui_copy_texture_surface,
            //font_atlas_collection
        }
    }
}

impl rwge::Runtime for Game {
    fn frame_start(&mut self, engine: &Engine) {
        public_data::utils::update_engine_time(&mut self.public_data, &engine);
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
                let size_event = RenderSystem::resize_event_transformation(event);
                if let Some(new_size) = size_event {
                    //Resize event
                    public_data::utils::update_screen_size(&mut self.public_data, new_size);

                    let rt_slotmap = self
                        .public_data
                        .collection
                        .get_mut::<Slotmap<RenderTexture>>()
                        .expect("Render texture slotmap not found");

                    self.gui_rects
                        .resize(new_size, &engine.render_system, rt_slotmap);

                    let gui_color_rt = &self.gui_rects.get_color_rt(rt_slotmap);
                    self.gui_copy_texture_surface
                        .update_texture_view(&gui_color_rt.texture_view, &engine.render_system);
                    engine.render_system.render_window.resize(new_size);

                    let mut resize_ui_event = UIEvent::Resize(new_size);
                    self.gui_system
                        .handle_event(&mut resize_ui_event, &mut self.public_data);
                } else {
                    let gui_event = rwge::gui::rect_ui::event::default_event_transformation(
                        event,
                        engine.render_system.render_window.size,
                    );
                    if let Some(mut e) = gui_event {
                        self.gui_system.handle_event(&mut e, &mut self.public_data);
                    }
                }
            }
        }
    }

    fn update(&mut self, engine: &rwge::Engine, exit_event_loop: &mut dyn FnMut() -> ()) {
        self.gui_system.update(&self.public_data);
    }

    fn render(
        &mut self,
        engine: &rwge::Engine,
        screen_view: &rwge::wgpu::TextureView,
        encoder: &mut rwge::wgpu::CommandEncoder,
    ) {
        ///////// APPLY PUBLIC DATA CHANGES
        self.public_data.apply_mut();
        ///////// APPLY PUBLIC DATA CHANGES

        rwge::render_system::texture::clear_render_targets(
            encoder,
            screen_view,
            RGBA::rrr1((0.12 as f32).powf(2.2)).into(),
            None,
            None,
            None,
        );

        self.gui_system
            .render(engine, &mut self.gui_rects, encoder, &mut self.public_data);

        self.gui_copy_texture_surface.render(encoder, screen_view);
    }

    fn frame_end<F>(&mut self, engine: &mut rwge::Engine, exit_event_loop: &mut F)
    where
        F: FnMut() -> (),
    {
        self.gui_system
            .window_layouting
            .control_state
            .on_frame_end();
        engine.render_system.destroy_queued_textures();
    }

    fn before_exit(&mut self, engine: &rwge::Engine) {
        println!("Before exit log")
    }

    fn get_window_id(&self) -> rwge::winit::window::WindowId {
        self.public_data
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
        .build(&event_loop)
        .expect("Window could not be created");

    let engine = Engine::new(&window);

    let game = Game::new(&engine, window);

    rwge::start_engine_loop(engine, game, event_loop);
}
