use std::{
    collections::HashMap,
    iter::Map,
    ops::{Deref, DerefMut},
};

use rwge::{
    slotmap::slotmap::{SlotKey, Slotmap},
    uuid::Uuid,
};

pub struct EntitySlot<T> {
    pub entity_id: Uuid,
    pub data: T,
}

impl<T> Deref for EntitySlot<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<T> DerefMut for EntitySlot<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

pub struct EntitySlotmap<T> {
    slotmap: Slotmap<EntitySlot<T>>,
    entity_map: HashMap<Uuid, SlotKey>,
}

impl<T: Sized> EntitySlotmap<T> {
    pub fn new(capacity: usize) -> Self {
        Self {
            slotmap: Slotmap::with_capacity(capacity),
            entity_map: HashMap::with_capacity(capacity),
        }
    }

    pub fn push(&mut self, data: T, entity_id: Uuid) -> Result<SlotKey, ()> {
        if !self.entity_map.contains_key(&entity_id) {
            match self.slotmap.push(EntitySlot {
                entity_id: entity_id,
                data,
            }) {
                Some(slot) => {
                    self.entity_map.insert(entity_id, slot);
                    Ok(slot)
                }
                None => Err(()),
            }
        } else {
            Err(())
        }
    }

    pub fn contains_entity(&self, entity_id: &Uuid) -> bool {
        self.entity_map.contains_key(entity_id)
    }

    pub fn get_iter(&self) -> std::slice::Iter<EntitySlot<T>> {
        self.slotmap.get_iter()
    }

    pub fn iter_mut_callback(&mut self, callback: &mut dyn Fn(Uuid, &mut T) -> ()) {
        for data in self
            .slotmap
            .get_iter_mut()
            .map(|entity_slot: &mut EntitySlot<T>| (entity_slot.entity_id, &mut entity_slot.data))
        {
            callback(data.0, data.1)
        }
    }

    pub fn iter_mut_vec(&mut self) -> Vec<(Uuid, &mut T)> {
        self.slotmap
            .get_iter_mut()
            .map(|entity_slot: &mut EntitySlot<T>| (entity_slot.entity_id, &mut entity_slot.data))
            .collect()
    }

    pub fn get_entity(&self, entity_id: &Uuid) -> Option<&T> {
        match self.entity_map.get(entity_id) {
            Some(slot) => match self.slotmap.get_value(slot) {
                Some(entity_slot) => Some(&entity_slot.data),
                None => None,
            },
            None => None,
        }
    }

    pub fn get_mut_entity(&mut self, entity_id: &Uuid) -> Option<&mut T> {
        match self.entity_map.get(entity_id) {
            Some(slot) => match self.slotmap.get_value_mut(slot) {
                Some(entity_slot) => Some(&mut entity_slot.data),
                None => None,
            },
            None => None,
        }
    }

    pub fn remove_entity(&mut self, entity_id: &Uuid) {
        if self.entity_map.contains_key(entity_id) {
            match self.entity_map.remove(entity_id) {
                Some(slot) => {
                    self.slotmap.remove(slot);
                }
                None => { /* No Op */ }
            }
        }
    }
}