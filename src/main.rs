pub use rwge::gui::rect_ui::system::GUIRects;
mod gui_system;
use gui_system::GUISystem;
use rwge::{
    color::RGBA,
    entity_component::{EngineDataTypeKey, PublicDataCollection},
    slotmap::slotmap::{SlotKey, Slotmap},
    Engine, EngineDataType, RenderTextureSlotmap,
};

pub struct Other{
    pub value: f32
}
pub enum DataType {
    Base(EngineDataType),
    OtherType(Slotmap<Other>)
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
    gui_system: GUISystem,
    public_data: PublicDataCollection<DataTypeKey, DataType>,
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

        let gui_system = GUISystem::new();

        let mut public_data = PublicDataCollection::<DataTypeKey, DataType>::new();
        public_data.collection.insert(
            EngineDataTypeKey::RenderTexture.into(),
            EngineDataType::RenderTexture(render_texture_slotmap).into(),
        );
        Self {
            gui_rects,
            gui_system,
            public_data,
        }
    }
}

impl rwge::Runtime for Game {
    fn handle_event_queue<F>(
        &mut self,
        event_queue: &Vec<rwge::EngineEvent>,
        exit_event_loop: &mut F,
    ) where
        F: FnMut() -> (),
    {
        println!("Handle event queue log");
        for event in event_queue {
            let close_event_handled = rwge::default_close_event_handler(event, exit_event_loop);
            if !close_event_handled {
                let gui_event = rwge::gui::rect_ui::event::default_event_transformation(event);
                if let Some(e) = gui_event {
                    self.gui_system.handle_event(&e, &mut self.public_data);
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
        rwge::render_system::utils::texture_utils::clear_render_targets(
            encoder,
            screen_view,
            RGBA::GREEN.into(),
            None,
            None,
            None,
        );

        

        self.gui_system.render(&mut self.gui_rects, encoder, &engine.system_bind_group, &mut self.public_data);
    }

    fn frame_end<F>(&mut self, exit_event_loop: &mut F)
    where
        F: FnMut() -> (),
    {
        //println!("Frame end log");
        //exit_event_loop();
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
