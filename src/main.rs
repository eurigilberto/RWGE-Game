use core::any::TypeId;

pub use rwge::gui::rect_ui::system::GUIRects;
mod gui_system;
use gui_system::GUISystem;
use rwge::{Engine, RenderTextureSlotmap, slotmap::slotmap::Slotmap, gui::rect_ui::event::UIEvent};

pub enum DataSlotmap{
    Base(rwge::entity_component::EngineDataSlotmapTypes)
}

pub type DataSlotmaps = Slotmap<DataSlotmap>;



struct Game {
    gui_system: GUISystem,
    public_data: DataSlotmaps
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

        let gui_system = GUISystem::new(gui_rects);

        let public_data = DataSlotmaps::new_with_capacity(20);

        Self { gui_system, public_data }
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
            if !close_event_handled{
                let gui_event = rwge::gui::rect_ui::event::default_event_transformation(event);
                if let Some(e) = gui_event {
                    self.gui_system.handle_event(&e, &mut self.public_data);
                }
            }
        }
    }

    fn update(&mut self, engine: &rwge::Engine) {}

    fn render(
        &mut self,
        engine: &mut rwge::Engine,
        screen_view: &rwge::wgpu::TextureView,
        encoder: &mut rwge::wgpu::CommandEncoder,
    ) -> bool {
        println!("Render log");
        false
    }

    fn frame_end<F>(&mut self, exit_event_loop: &mut F)
    where
        F: FnMut() -> (),
    {
        println!("Frame end log");
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

    println!("The engine stopped!!");
}
