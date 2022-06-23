use std::{
    any::{Any, TypeId},
    collections::HashMap,
};

pub struct Anymap {
    map: HashMap<TypeId, Box<dyn Any>>,
}

#[allow(dead_code)]
impl Anymap{
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }
    /// If data is already in the map, then it is returned if not it returns `None`
    pub fn insert<T>(&mut self, data: T) -> Option<T>
    where
        T: Sized + 'static,
    {
        if self.contains::<T>() {
            Some(data)
        } else {
            let data_any = Box::new(data) as Box<dyn Any>;
            let key = TypeId::of::<T>();
            self.map.insert(key, data_any);
            None
        }
    }

    pub fn contains<T: 'static>(&self) -> bool {
        self.map.contains_key(&TypeId::of::<T>())
    }

    pub fn remove<T: 'static>(&mut self) {
        self.map.remove(&TypeId::of::<T>());
    }

    pub fn get<T: 'static>(&self) -> Option<&T> {
        match self.map.get(&TypeId::of::<T>()) {
            Some(data) => match data.downcast_ref() {
                Some(data) => Some(data),
                None => None,
            },
            None => None,
        }
    }

    pub fn get_mut<T: 'static>(&mut self) -> Option<&mut T> {
        match self.map.get_mut(&TypeId::of::<T>()) {
            Some(data) => match data.downcast_mut() {
                Some(data) => Some(data),
                None => None,
            },
            None => None,
        }
    }

    pub fn get_any(&self, type_id: TypeId) -> Option<&Box<dyn Any>>{
        self.map.get(&type_id)
    }

    pub fn get_mut_any(&mut self, type_id: TypeId) -> Option<&mut Box<dyn Any>>{
        self.map.get_mut(&type_id)
    }
}
