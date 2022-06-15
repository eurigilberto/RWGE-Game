use std::collections::HashMap;

use rwge::{
    render_system::render_texture::RenderTexture,
    slotmap::slotmap::{SlotKey, Slotmap},
    glam::{UVec2, uvec2},
};

use crate::anymap::{self, Anymap};

pub struct PublicData{
    pub collection: Anymap
}
impl PublicData{
    pub fn new()->Self{
        Self { collection: Anymap::new()}
    }
}

pub mod utils{
    use rwge::{slotmap::slotmap::{Slotmap, SlotKey}, render_system::render_texture::RenderTexture};

    use super::PublicData;

    pub fn get_render_texture<'a>(public_data: &'a PublicData, key: &SlotKey)->Option<&'a RenderTexture>{
        let rt_slotmap = public_data.collection.get::<Slotmap<RenderTexture>>();
        rt_slotmap.expect("Render Texture Slotmap not found").get_value(key)
    }
}