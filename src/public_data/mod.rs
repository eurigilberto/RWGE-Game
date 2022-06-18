use rwge::{glam::UVec2, Engine, engine_time::EngineTime};

use crate::anymap::{Anymap};

pub struct PublicData{
    pub collection: Anymap
}
impl PublicData{
    pub fn new()->Self{
        Self { collection: Anymap::new()}
    }
}

#[derive(Default)]
pub struct EngineTimeData{
    pub time: f32,
    pub time_millis: f32,
    pub delta_time: f32,
    pub delta_time_milis: f32,
    pub time_millis_since_start: u128,
}

impl EngineTimeData {
    pub fn sin_time(&self, mult: f32)->f32{
        f32::sin(self.time * mult)
    }
    pub fn cos_time(&self, mult: f32)->f32{
        f32::cos(self.time * mult)
    }
}

pub struct EngineData{
    pub time: EngineTimeData,
    pub screen_size: UVec2
}

impl EngineTimeData{
    pub fn update_time_data(&mut self, engine_time: &EngineTime){
        self.time = engine_time.time_data.time;
        self.time_millis = engine_time.time_data.time_millis; 
        self.delta_time = engine_time.time_data.delta_time;
        self.delta_time_milis = engine_time.time_data.delta_time_milis;
        self.time_millis_since_start = engine_time.time_since_start;
    }
}

impl EngineData {
    pub fn new_from_engine(engine: &Engine)->Self{
        let mut time = EngineTimeData::default();
        time.update_time_data(&engine.time);
        Self {
            time,
            screen_size: engine.get_screen_size() }
    }
}

pub mod utils{
    use rwge::{slotmap::slotmap::{Slotmap, SlotKey}, render_system::render_texture::RenderTexture, engine_time::EngineTime, glam::UVec2};

    use super::{PublicData, EngineData};

    /// Panic! if `RenderTextureSlotmap` is not present on the `PublicData` collection
    pub fn get_render_texture<'a>(public_data: &'a PublicData, key: &SlotKey)->Option<&'a RenderTexture>{
        let rt_slotmap = public_data.collection.get::<Slotmap<RenderTexture>>();
        rt_slotmap.expect("Render Texture Slotmap not found").get_value(key)
    }

    /// Panic! if `EngineData` is not present on the `PublicData` collection
    pub fn update_engine_time(public_data: &mut PublicData, engine_time: &EngineTime){
        let engine_data = public_data.collection.get_mut::<EngineData>().unwrap();
        engine_data.time.update_time_data(engine_time);
    }

    /// Panic! if `EngineData` is not present on the `PublicData` collection
    pub fn update_screen_size(public_data: &mut PublicData, new_size: UVec2){
        let engine_data = public_data.collection.get_mut::<EngineData>().unwrap();
        engine_data.screen_size = new_size;
    }

    /// Panic! if `EngineData` is not present on the `PublicData` collection
    pub fn get_engine_data(public_data: &PublicData)->&EngineData{
        public_data.collection.get().unwrap()
    }
}