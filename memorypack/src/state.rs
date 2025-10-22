use crate::error::MemoryPackError;
use std::collections::HashMap;
use std::any::Any;

pub struct MemoryPackWriterOptionalState {
    next_id: u32,
    object_to_ref: HashMap<usize, u32>,
}

impl MemoryPackWriterOptionalState {
    pub fn new() -> Self {
        Self {
            next_id: 0,
            object_to_ref: HashMap::new(),
        }
    }

    pub fn reset(&mut self) {
        self.object_to_ref.clear();
        self.next_id = 0;
    }

    pub fn get_or_add_reference<T: ?Sized>(&mut self, value: &T) -> (bool, u32) {
        let ptr = value as *const T as *const () as usize;
        
        if let Some(&id) = self.object_to_ref.get(&ptr) {
            (true, id)
        } else {
            let id = self.next_id;
            self.next_id += 1;
            self.object_to_ref.insert(ptr, id);
            (false, id)
        }
    }
}

impl Default for MemoryPackWriterOptionalState {
    fn default() -> Self {
        Self::new()
    }
}

pub struct MemoryPackReaderOptionalState {
    ref_to_object: HashMap<u32, Box<dyn Any>>,
}

impl MemoryPackReaderOptionalState {
    pub fn new() -> Self {
        Self {
            ref_to_object: HashMap::new(),
        }
    }

    pub fn reset(&mut self) {
        self.ref_to_object.clear();
    }

    pub fn get_object_reference<T: 'static + Clone>(&self, id: u32) -> Result<T, MemoryPackError> {
        self.ref_to_object
            .get(&id)
            .and_then(|boxed| boxed.downcast_ref::<T>())
            .cloned()
            .ok_or_else(|| MemoryPackError::DeserializationError(
                format!("Object is not found in this reference id: {}", id)
            ))
    }

    pub fn add_object_reference<T: 'static>(&mut self, id: u32, value: T) -> Result<(), MemoryPackError> {
        if self.ref_to_object.contains_key(&id) {
            return Err(MemoryPackError::DeserializationError(
                format!("Object is already added, id: {}", id)
            ));
        }
        self.ref_to_object.insert(id, Box::new(value));
        Ok(())
    }

    pub fn update_object_reference<T: 'static>(&mut self, id: u32, value: T) -> Result<(), MemoryPackError> {
        if let Some(entry) = self.ref_to_object.get_mut(&id) {
            *entry = Box::new(value);
            Ok(())
        } else {
            Err(MemoryPackError::DeserializationError(
                format!("Object not found for update, id: {}", id)
            ))
        }
    }
}

impl Default for MemoryPackReaderOptionalState {
    fn default() -> Self {
        Self::new()
    }
}

