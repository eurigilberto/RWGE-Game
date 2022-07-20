use std::cell::RefCell;

use rwge::{
    engine::{engine_time::EngineTime, op_time::OperationTime},
    glam::UVec2,
    Engine,
};

mod entity_slotmap;

use crate::anymap::Anymap;

pub type DataMutFn = Box<dyn FnMut(&mut Anymap) -> ()>;

pub struct PublicData {
    pub collection: Anymap,
    mutations: RefCell<Vec<DataMutFn>>,
}

impl PublicData {
    pub fn new() -> Self {
        Self {
            collection: Anymap::new(),
            mutations: RefCell::new(Vec::with_capacity(100)),
        }
    }
    pub fn insert<T>(&mut self, data: T) -> Option<T>
    where
        T: Sized + 'static,
    {
        self.collection.insert(data)
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

pub struct RuntimeData {
    pub public_data: PublicData,
}
impl RuntimeData {
    pub fn new() -> Self {
        Self {
            public_data: PublicData::new(),
        }
    }

    pub fn insert_pub<T>(&mut self, data: T) -> Option<T>
    where
        T: Sized + 'static,
    {
        self.public_data.insert(data)
    }

    pub fn push_pub_mut(&self, mutation: Box<dyn FnMut(&mut Anymap) -> ()>){
        self.public_data.push_mut(mutation);
    }

    pub fn apply_pub_mut(&mut self) {
        self.public_data.apply_mut();
    }

    pub fn get_pub<T: 'static>(&self) -> Option<&T> {
        self.public_data.collection.get::<T>()
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
        font::font_load_gpu::FontCollection,
        glam::UVec2,
        render_system::render_texture::RenderTexture,
        slotmap::slotmap::{SlotKey, Slotmap},
        winit, Engine,
    };

    use super::{EngineData, EngineTimeData, RuntimeData};

    /// Panic! if `RenderTextureSlotmap` is not present on the `PublicData` collection
    pub fn get_render_texture<'a>(
        runtime_data: &'a RuntimeData,
        key: &SlotKey,
    ) -> Option<&'a RenderTexture> {
        let rt_slotmap = runtime_data
            .public_data
            .collection
            .get::<Slotmap<RenderTexture>>();
        rt_slotmap
            .expect("Render Texture Slotmap not found")
            .get_value(key)
    }

    /// Panic! if `EngineData` is not present on the `PublicData` collection
    pub fn update_engine_time(runtime_data: &mut RuntimeData, engine: &Engine) {
        let engine_data = runtime_data
            .public_data
            .collection
            .get_mut::<EngineData>()
            .unwrap();
        engine_data.time.update_time_data(&engine.time);
        engine_data.operation_time.copy_from(&engine.operation_time);
    }

    /// Panic! if `EngineData` is not present on the `PublicData` collection
    pub fn update_screen_size(runtime_data: &mut RuntimeData, new_size: UVec2) {
        let engine_data = runtime_data.public_data.collection.get_mut::<EngineData>().unwrap();
        engine_data.screen_size = new_size;
    }

    /// Panic! if `EngineData` is not present on the `PublicData` collection
    pub fn get_engine_data(runtime_data: &RuntimeData) -> &EngineData {
        runtime_data.get_pub().unwrap()
    }

    pub fn get_time(runtime_data: &RuntimeData) -> &EngineTimeData {
        &runtime_data
            .public_data
            .collection
            .get::<EngineData>()
            .unwrap()
            .time
    }

    pub fn get_font_collections(runtime_data: &RuntimeData) -> &Vec<FontCollection> {
        &runtime_data
            .public_data
            .collection
            .get::<Vec<FontCollection>>()
            .unwrap()
    }

    pub fn get_window(runtime_data: &RuntimeData) -> &winit::window::Window {
        runtime_data.get_pub().unwrap()
    }
}
