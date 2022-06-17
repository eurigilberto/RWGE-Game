use rwge::glam::UVec2;

use crate::anymap::{Anymap};

pub struct PublicData{
    pub collection: Anymap
}
impl PublicData{
    pub fn new()->Self{
        Self { collection: Anymap::new()}
    }
}

pub struct EngineData{
    pub time: f32,
    pub time_millis: f32,
    pub screen_size: UVec2
}

pub mod utils{
    use rwge::{slotmap::slotmap::{Slotmap, SlotKey}, render_system::render_texture::RenderTexture};

    use super::PublicData;

    pub fn get_render_texture<'a>(public_data: &'a PublicData, key: &SlotKey)->Option<&'a RenderTexture>{
        let rt_slotmap = public_data.collection.get::<Slotmap<RenderTexture>>();
        rt_slotmap.expect("Render Texture Slotmap not found").get_value(key)
    }
}