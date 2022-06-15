mod test;
use std::any::{TypeId, Any};

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
        system::{BorderRadius, ExtraBufferData, GUIRects, RectMask},
    },
    render_system::{render_texture::RenderTexture, RenderSystem},
    slotmap::slotmap::{SlotKey, Slotmap},
    wgpu,
    winit::window,
    Engine,
};

use crate::{public_data::{utils::get_render_texture, PublicData, self}, anymap::Anymap};

pub struct WOne{
    pub index: u32,
    pub ans: u32
}

impl WOne{
    pub fn new()->Self{
        Self { index: 0, ans: 202 }
    }
}

pub struct WTwo{
    pub index: u32,
    pub ans: u32
}

trait AsAny {
    fn as_any(&self)->&dyn Any;
}

impl WTwo{
    pub fn new()->Self{
        Self { index: 2, ans: 859 }
    }
}

impl GUIContainer for WOne{
    fn get_name(&self)->&str {
        "WOne"
    }

    fn handle_event(&self, event: &mut UIEvent, public_data: &mut PublicData) {
        println!("handling events WOne")
    }
}

impl GUIContainer for WTwo{
    fn get_name(&self)->&str {
        "WTwo"
    }

    fn handle_event(&self, event: &mut UIEvent, public_data: &mut PublicData) {
        println!("handling events WTwo")
    }   
}

trait GUIContainer:AsAny {
    fn get_name(&self)->&str;
    fn handle_event(&self, event: &mut UIEvent, public_data: &mut PublicData);
    // When contained in a window, this is going to be used for the size if an specific one is nee
    // Not sure how this call should work yet
    //fn required_size(&self)->Option<UVec2>;
}

impl <T:GUIContainer + 'static> AsAny for T{
    fn as_any(&self)->&dyn Any {
        self
    }
}

#[test]
pub fn testing_things(){
    let mut gui_containers = Slotmap::<Box<dyn GUIContainer>>::new_with_capacity(10);
    let gui_container = WOne::new();
    let key0 = gui_containers.push(Box::new(gui_container));
    let gui_container = WTwo::new();
    let key1 = gui_containers.push(Box::new(gui_container));

    {
        let container_thing = gui_containers.get_value_mut(&key1.expect(""));
        let c_t = container_thing.unwrap();
        
        let thing: &WTwo = AsAny::as_any(c_t.as_ref()).downcast_ref().expect("imposible to cast?");
    }
}

pub struct WindowContainer {
    pub slot_key: SlotKey,
    pub position: UVec2,
    pub size: UVec2,
}

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
