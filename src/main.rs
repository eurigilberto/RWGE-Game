use std::num::NonZeroU32;

pub use rwge::gui::rect_ui::system::GUIRects;
mod gui_system;
use gui_system::GUISystem;
use rwge::{
    color::RGBA,
    entity_component::{EngineDataTypeKey, PublicDataCollection},
    font,
    glam::{uvec2, Vec2},
    half,
    render_system::copy_texture_to_surface::CopyTextureToSurface,
    render_system::RenderSystem,
    slotmap::slotmap::{SlotKey, Slotmap},
    Engine, EngineDataType, RenderTextureSlotmap,
    font::font_atlas::FontAtlas
};

pub struct Other {
    pub value: f32,
}
pub enum DataType {
    Base(EngineDataType),
    OtherType(Slotmap<Other>),
}

impl From<EngineDataType> for DataType {
    fn from(key: EngineDataType) -> Self {
        DataType::Base(key)
    }
}

#[derive(Hash, PartialEq, Eq)]
pub enum DataTypeKey {
    Base(EngineDataTypeKey),
}

impl From<EngineDataTypeKey> for DataTypeKey {
    fn from(key: EngineDataTypeKey) -> Self {
        DataTypeKey::Base(key)
    }
}

pub struct DataKey {
    pub map_key: DataTypeKey,
    pub key: SlotKey,
}

struct Game {
    gui_rects: GUIRects,
    gui_copy_texture_surface: CopyTextureToSurface,
    gui_system: GUISystem,
    public_data: PublicDataCollection<DataTypeKey, DataType>,
    font_atlas_collection: Vec<FontAtlas>
}

fn create_gui_copy_texture_to_surface(
    public_data: &mut PublicDataCollection<DataTypeKey, DataType>,
    gui_rects: &GUIRects,
    engine: &Engine,
) -> Option<CopyTextureToSurface> {
    if let DataType::Base(EngineDataType::RenderTexture(render_texture_slotmap)) = public_data
        .collection
        .get(&DataTypeKey::Base(EngineDataTypeKey::RenderTexture))
        .expect("No Render Texture collection was found")
    {
        let color_rt = render_texture_slotmap
            .get_value(gui_rects.render_texture.color_texture_key.key)
            .expect("Color Render Texture not found");
        let gui_copy_texture_surface =
            CopyTextureToSurface::new(&engine.render_system, &color_rt.texture_view);
        Some(gui_copy_texture_surface)
    } else {
        None
    }
}

fn write_font_to_gpu(engine: &Engine, gui_rects: &GUIRects) -> Vec<FontAtlas> {
    let neo_san_med_data = include_bytes!("../res/fonts/Lato/Lato-Light.ttf");
    let neo_san_light_data = include_bytes!("../res/fonts/Lato/Lato-Bold.ttf");

    let neo_san_med_font_atlas = FontAtlas::new(
        neo_san_med_data,
        uvec2(1024, 1024),
        48.0,
        font::font_atlas::FontCharLimit::All,
    )
    .expect("Font Atlas could not be created");
    let neo_san_light_font_atlas = FontAtlas::new(
        neo_san_light_data,
        uvec2(1024, 1024),
        48.0,
        font::font_atlas::FontCharLimit::All,
    )
    .expect("Font Atlas could not be created");
    

    let fonts_texture_size = neo_san_med_font_atlas.font_sdf_texture.len() * 4;
    let atlas_lenght = neo_san_med_font_atlas.font_sdf_texture.len();

    let neo_san_med_data_slice = half::slice::HalfFloatSliceExt::reinterpret_cast(
        neo_san_med_font_atlas.font_sdf_texture.as_slice(),
    );
    let neo_san_light_data_slice = half::slice::HalfFloatSliceExt::reinterpret_cast(
        neo_san_light_font_atlas.font_sdf_texture.as_slice(),
    );

    let mut fonts_texture = vec![0 as u16; fonts_texture_size];

    for index in 0..atlas_lenght {
        let pixel_index = index * 4 as usize;
        fonts_texture[pixel_index] = neo_san_med_data_slice[index];
        fonts_texture[pixel_index + 1] = neo_san_light_data_slice[index];
    }

    let tx_block_size = (rwge::wgpu::TextureFormat::Rgba16Float)
        .describe()
        .block_size;
    let bytes_per_row = tx_block_size as u32 * 1024;

    engine.render_system.render_window.queue.write_texture(
        gui_rects.texture_atlas.texture.as_image_copy(),
        rwge::bytemuck::cast_slice(fonts_texture.as_slice()),
        rwge::wgpu::ImageDataLayout {
            offset: 0,
            bytes_per_row: NonZeroU32::new(bytes_per_row),
            rows_per_image: NonZeroU32::new(1024),
        },
        rwge::wgpu::Extent3d {
            width: 1024,
            height: 1024,
            depth_or_array_layers: 1,
        },
    );

    vec![neo_san_med_font_atlas, neo_san_light_font_atlas]
}

impl Game {
    fn new(engine: &Engine) -> Self {
        let size = engine.render_system.render_window.size.clone();

        let mut render_texture_slotmap = RenderTextureSlotmap::new_with_capacity(10);

        let gui_rects = GUIRects::new(
            &engine.render_system,
            &engine.system_bind_group_layout,
            size,
            &mut render_texture_slotmap,
        );
        //println!("Size: {}", size);
        let gui_system = GUISystem::new(size);

        let mut public_data = PublicDataCollection::<DataTypeKey, DataType>::new();
        public_data.collection.insert(
            EngineDataTypeKey::RenderTexture.into(),
            EngineDataType::RenderTexture(render_texture_slotmap).into(),
        );
        
        let font_atlas_collection = write_font_to_gpu(engine, &gui_rects);

        if let Some(gui_copy_texture_surface) =
            create_gui_copy_texture_to_surface(&mut public_data, &gui_rects, engine)
        {
            Self {
                gui_rects,
                gui_system,
                public_data,
                gui_copy_texture_surface,
                font_atlas_collection
            }
        } else {
            panic!("Render texture slotmap not found")
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
                    if let Some(DataType::Base(EngineDataType::RenderTexture(
                        render_texture_slotmap,
                    ))) = self
                        .public_data
                        .collection
                        .get_mut(&DataTypeKey::Base(EngineDataTypeKey::RenderTexture))
                    {
                        for render_texture in render_texture_slotmap.get_iter_mut() {
                            if let Some(scale) = render_texture.scale_size_to_surface {
                                let new_size_scaled = (new_size.as_vec2() * scale).as_uvec2();
                                render_texture
                                    .resize_texture(new_size_scaled, &mut engine.render_system);
                            }
                        }
                    }

                    self.gui_system.resize(new_size);
                    self.gui_rects.resize(new_size, &engine.render_system);

                    if let Some(DataType::Base(EngineDataType::RenderTexture(
                        render_texture_slotmap,
                    ))) = self
                        .public_data
                        .collection
                        .get(&DataTypeKey::Base(EngineDataTypeKey::RenderTexture))
                    {
                        if let Some(color_rt) = render_texture_slotmap
                            .get_value(self.gui_rects.render_texture.color_texture_key.key)
                        {
                            self.gui_copy_texture_surface
                                .update_texture_view(&color_rt.texture_view, &engine.render_system);
                        }
                    }

                    engine.render_system.render_window.resize(new_size);
                } else {
                    let gui_event = rwge::gui::rect_ui::event::default_event_transformation(event);
                    if let Some(e) = gui_event {
                        self.gui_system.handle_event(&e, &mut self.public_data);
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
            &mut self.gui_rects,
            encoder,
            &engine.system_bind_group,
            &engine.render_system,
            &mut self.public_data,
            &self.font_atlas_collection
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
