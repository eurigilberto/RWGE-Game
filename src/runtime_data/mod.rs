use std::{
    cell::RefCell,
    ops::{Deref, DerefMut},
};

use rwge::{
    glam::UVec2,
    Engine, engine::{operation_timer::OperationTimer, engine_timer::EngineTimer, time::{Second, Millisecond, Microsecond, FrameNumber}},
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

impl Deref for PublicData {
    type Target = Anymap;

    fn deref(&self) -> &Self::Target {
        &self.collection
    }
}

impl DerefMut for PublicData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.collection
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

    pub fn apply_pub_mut(&mut self) {
        self.public_data.apply_mut();
    }
}

#[derive(Default)]
pub struct EngineTimeData {
    pub time: Second,
    pub time_millis: Millisecond,
    pub delta_time: Second,
    pub delta_time_millis: Millisecond,
    pub time_millis_since_start: Microsecond,
    pub frame_count: FrameNumber,
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
    pub operation_time: OperationTimer,
    pub screen_size: UVec2,
}

impl EngineTimeData {
    pub fn update_time_data(&mut self, engine_time: &EngineTimer) {
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
        time.update_time_data(&engine.timer);

        let mut operation_time = OperationTimer::new();
        operation_time.copy_from(&engine.operation_timer);

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
        font::font_load_gpu::FontCollection,
        glam::UVec2,
        graphics::render_texture::RenderTexture,
        slotmap::prelude::*,
        winit, Engine,
    };

    use super::{EngineData, EngineTimeData, PublicData};

    /// Panic! if `RenderTextureSlotmap` is not present on the `PublicData` collection
    pub fn get_render_texture<'a>(
        public_data: &'a PublicData,
        key: &SlotKey,
    ) -> Option<&'a RenderTexture> {
        let rt_slotmap = public_data.get::<Slotmap<RenderTexture>>();
        rt_slotmap
            .expect("Render Texture Slotmap not found")
            .get_value(key)
    }

    /// Panic! if `EngineData` is not present on the `PublicData` collection
    pub fn update_engine_time(public_data: &mut PublicData, engine: &Engine) {
        let engine_data = public_data.get_mut::<EngineData>().unwrap();
        engine_data.time.update_time_data(&engine.timer);
        engine_data.operation_time.copy_from(&engine.operation_timer);
    }

    /// Panic! if `EngineData` is not present on the `PublicData` collection
    pub fn update_screen_size(public_data: &mut PublicData, new_size: UVec2) {
        let engine_data = public_data.get_mut::<EngineData>().unwrap();
        engine_data.screen_size = new_size;
    }

    /// Panic! if `EngineData` is not present on the `PublicData` collection
    pub fn get_engine_data(public_data: &PublicData) -> &EngineData {
        public_data.get().unwrap()
    }

    pub fn get_time(public_data: &PublicData) -> &EngineTimeData {
        &public_data.get::<EngineData>().unwrap().time
    }

    pub fn get_font_collections(public_data: &PublicData) -> &Vec<FontCollection> {
        &public_data.get::<Vec<FontCollection>>().unwrap()
    }

    pub fn get_window(public_data: &PublicData) -> &winit::window::Window {
        public_data.get().unwrap()
    }
}
