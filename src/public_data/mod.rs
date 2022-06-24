use std::cell::RefCell;

use rwge::{
    engine::{engine_time::EngineTime, op_time::OperationTime},
    glam::UVec2,
    Engine,
};

use crate::anymap::Anymap;

pub struct PublicData {
    pub collection: Anymap,
    mutations: RefCell<Vec<Box<dyn FnMut(&mut Anymap) -> ()>>>,
}
impl PublicData {
    pub fn new() -> Self {
        Self {
            collection: Anymap::new(),
            mutations: RefCell::new(Vec::with_capacity(100)),
        }
    }

    pub fn push_mut(&self, mutation: Box<dyn FnMut(&mut Anymap) -> ()>) {
        self.mutations.borrow_mut().push(mutation);
    }
    pub fn apply_mut(&mut self) {
        for mut change in self.mutations.borrow_mut().drain(..) {
            change(&mut self.collection)
        }
    }
}

#[derive(Default)]
pub struct EngineTimeData {
    pub time: f32,
    pub time_millis: f32,
    pub delta_time: f32,
    pub delta_time_millis: f32,
    pub time_millis_since_start: u128,
    pub frame_count: u32,
}

#[allow(dead_code)]
impl EngineTimeData {
    pub fn sin_time(&self, mult: f32) -> f32 {
        f32::sin(self.time * mult)
    }
    pub fn sin_time_phase(&self, mult: f32, phase: f32) -> f32 {
        f32::sin(self.time * mult + phase)
    }
    pub fn cos_time(&self, mult: f32) -> f32 {
        f32::cos(self.time * mult)
    }
    pub fn cos_time_phase(&self, mult: f32, phase: f32) -> f32 {
        f32::cos(self.time * mult + phase)
    }
}

pub struct EngineData {
    pub time: EngineTimeData,
    pub operation_time: OperationTime,
    pub screen_size: UVec2,
}

impl EngineTimeData {
    pub fn update_time_data(&mut self, engine_time: &EngineTime) {
        self.time = engine_time.time_data.time;
        self.time_millis = engine_time.time_data.time_millis;
        self.delta_time = engine_time.time_data.delta_time;
        self.delta_time_millis = engine_time.time_data.delta_time_milis;
        self.time_millis_since_start = engine_time.time_since_start;
        self.frame_count = engine_time.frame_count;
    }
}

impl EngineData {
    pub fn new_from_engine(engine: &Engine) -> Self {
        let mut time = EngineTimeData::default();
        time.update_time_data(&engine.time);

        let mut operation_time = OperationTime::new();
        operation_time.copy_from(&engine.operation_time);
        
        Self {
            time,
            operation_time: operation_time,
            screen_size: engine.get_screen_size(),
        }
    }
}

#[allow(dead_code)]
pub mod utils {
    use rwge::{
        engine::engine_time::EngineTime,
        glam::UVec2,
        render_system::render_texture::RenderTexture,
        slotmap::slotmap::{SlotKey, Slotmap},
        winit, Engine,
    };

    use super::{EngineData, PublicData};

    /// Panic! if `RenderTextureSlotmap` is not present on the `PublicData` collection
    pub fn get_render_texture<'a>(
        public_data: &'a PublicData,
        key: &SlotKey,
    ) -> Option<&'a RenderTexture> {
        let rt_slotmap = public_data.collection.get::<Slotmap<RenderTexture>>();
        rt_slotmap
            .expect("Render Texture Slotmap not found")
            .get_value(key)
    }

    /// Panic! if `EngineData` is not present on the `PublicData` collection
    pub fn update_engine_time(public_data: &mut PublicData, engine: &Engine) {
        let engine_data = public_data.collection.get_mut::<EngineData>().unwrap();
        engine_data.time.update_time_data(&engine.time);
        engine_data.operation_time.copy_from(&engine.operation_time);
    }

    /// Panic! if `EngineData` is not present on the `PublicData` collection
    pub fn update_screen_size(public_data: &mut PublicData, new_size: UVec2) {
        let engine_data = public_data.collection.get_mut::<EngineData>().unwrap();
        engine_data.screen_size = new_size;
    }

    /// Panic! if `EngineData` is not present on the `PublicData` collection
    pub fn get_engine_data(public_data: &PublicData) -> &EngineData {
        public_data.collection.get().unwrap()
    }

    pub fn get_window(public_data: &PublicData) -> &winit::window::Window {
        public_data.collection.get().unwrap()
    }
}
