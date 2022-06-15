use std::num::NonZeroU32;
mod gui_font;
mod public_data;
use gui_font::write_font_to_gpu;
use public_data::PublicData;
pub use rwge::gui::rect_ui::system::GUIRects;
mod gui_system;
use gui_system::GUISystem;
use rwge::{
    color::RGBA,
    font,
    font::font_atlas::FontAtlas,
    glam::{uvec2, Vec2},
    half,
    render_system::copy_texture_to_surface::CopyTextureToSurface,
    render_system::{render_texture::RenderTexture, RenderSystem},
    slotmap::slotmap::{SlotKey, Slotmap},
    Engine,
};
mod anymap;
struct Game {
    gui_rects: GUIRects,
    gui_copy_texture_surface: CopyTextureToSurface,
    gui_system: GUISystem,
    public_data: PublicData,
    font_atlas_collection: Vec<FontAtlas>,
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
    fn new(engine: &Engine) -> Self {
        let size = engine.render_system.render_window.size.clone();

        let mut render_texture_slotmap = Slotmap::<RenderTexture>::new_with_capacity(10);

        let gui_rects = GUIRects::new(
            &engine.render_system,
            &engine.system_bind_group_layout,
            size,
            &mut render_texture_slotmap,
        );
        let gui_system = GUISystem::new(size);

        let mut public_data = PublicData::new();
        public_data.collection.insert(render_texture_slotmap);

        let font_atlas_collection = write_font_to_gpu(engine, &gui_rects);
        let gui_copy_texture_surface =
            create_gui_copy_texture_to_surface(&mut public_data, &gui_rects, engine);
        Self {
            gui_rects,
            gui_system,
            public_data,
            gui_copy_texture_surface,
            font_atlas_collection,
        }
    }
}

impl rwge::Runtime for Game {
    fn handle_event_queue<F>(
        &mut self,
        event_queue: &Vec<rwge::EngineEvent>,
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
                    let rt_slotmap = self
                        .public_data
                        .collection
                        .get_mut::<Slotmap<RenderTexture>>()
                        .expect("Render texture slotmap not found");
                    self.gui_system.resize(new_size);
                    self.gui_rects
                        .resize(new_size, &mut engine.render_system, rt_slotmap);
                    let gui_color_rt = &self.gui_rects.get_color_rt(rt_slotmap);
                    self.gui_copy_texture_surface
                        .update_texture_view(&gui_color_rt.texture_view, &engine.render_system);
                    engine.render_system.render_window.resize(new_size);
                } else {
                    let gui_event = rwge::gui::rect_ui::event::default_event_transformation(
                        event,
                        engine.render_system.render_window.size,
                    );
                    if let Some(mut e) = gui_event {
                        self.gui_system
                            .handle_event(&mut e, &mut self.public_data, &engine);
                    }
                }
            }
        }
    }

    fn update(&mut self, engine: &rwge::Engine) {
        self.gui_system.update(&mut self.public_data);
    }

    fn render(
        &mut self,
        engine: &mut rwge::Engine,
        screen_view: &rwge::wgpu::TextureView,
        encoder: &mut rwge::wgpu::CommandEncoder,
    ) {
        rwge::render_system::texture::clear_render_targets(
            encoder,
            screen_view,
            RGBA::BLACK.into(),
            None,
            None,
            None,
        );

        self.gui_system.render(
            engine,
            &mut self.gui_rects,
            encoder,
            &mut self.public_data,
            &self.font_atlas_collection,
        );

        self.gui_copy_texture_surface.render(encoder, screen_view);
    }

    fn frame_end<F>(&mut self, engine: &mut rwge::Engine, exit_event_loop: &mut F)
    where
        F: FnMut() -> (),
    {
        engine.render_system.destroy_queued_textures();
    }

    fn before_exit(&mut self, engine: &rwge::Engine) {
        println!("Before exit log")
    }
}

fn main() {
    let (engine, event_loop) = rwge::create_engine(950, 600, "Game A");

    let game = Game::new(&engine);

    rwge::start_engine_loop(engine, game, event_loop);
}
