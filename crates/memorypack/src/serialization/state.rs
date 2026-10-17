use std::any::Any;
use std::collections::HashMap;
use std::ptr;
use std::rc::Rc;

use crate::error::MemoryPackError;
use crate::graph::arena::{GraphArena, ReferenceKey};

#[derive(Default)]
pub struct MemoryPackWriterOptionalState {
    next_id: u32,
    object_references: HashMap<usize, u32>,
    pub(crate) graph: Option<Box<WriterGraphContext>>
}

#[derive(Default)]
pub struct MemoryPackReaderOptionalState {
    referenced_objects: HashMap<u32, Box<dyn Any>>,
    pub(crate) graph: Option<Box<GraphArena>>
}

pub(crate) struct WriterGraphContext {
    pub arena: Rc<GraphArena>,
    references: HashMap<ReferenceKey, u32>
}

impl MemoryPackWriterOptionalState {
    pub fn new() -> Self { Self::default() }

    pub fn reset(&mut self) {
        self.object_references.clear();
        self.graph = None;
        self.next_id = 0;
    }

    pub fn get_or_add_reference<T: ?Sized>(&mut self, value: &T) -> (bool, u32) {
        let ptr = ptr::from_ref::<T>(value).cast::<()>() as usize;

        if let Some(&id) = self.object_references.get(&ptr) {
            (true, id)
        } else {
            let id = self.next_id;

            self.next_id += 1;
            self.object_references.insert(ptr, id);

            (false, id)
        }
    }

    pub(crate) fn get_or_add_graph_reference(
        &mut self,
        key: ReferenceKey
    ) -> Result<(bool, u32), MemoryPackError> {
        let context = self
            .graph
            .as_mut()
            .ok_or(MemoryPackError::ObjectReferenceNotFound(key.error_id()))?;

        if let Some(&reference_id) = context.references.get(&key) {
            return Ok((true, reference_id));
        }

        let reference_id = self.next_id;
        self.next_id += 1;
        context.references.insert(key, reference_id);

        Ok((false, reference_id))
    }
}

impl MemoryPackReaderOptionalState {
    pub fn new() -> Self { Self::default() }

    pub fn reset(&mut self) {
        self.referenced_objects.clear();
        self.graph = None;
    }

    pub fn get_object_reference<T: 'static + Clone>(&self, id: u32) -> Result<T, MemoryPackError> {
        self.referenced_objects
            .get(&id)
            .and_then(|boxed: &Box<dyn Any>| boxed.downcast_ref::<T>())
            .cloned()
            .ok_or(MemoryPackError::ObjectReferenceNotFound(id))
    }

    pub fn add_object_reference<T: 'static>(
        &mut self,
        id: u32,
        value: T
    ) -> Result<(), MemoryPackError> {
        if self.referenced_objects.contains_key(&id) {
            return Err(MemoryPackError::ObjectReferenceAlreadyAdded(id));
        }

        self.referenced_objects
            .insert(id, Box::new(value));
        Ok(())
    }

    pub fn update_object_reference<T: 'static>(
        &mut self,
        id: u32,
        value: T
    ) -> Result<(), MemoryPackError> {
        self.referenced_objects.get_mut(&id).map_or_else(
            || Err(MemoryPackError::ObjectReferenceUpdateNotFound(id)),
            |entry| {
                *entry = Box::new(value);
                Ok(())
            }
        )
    }
}

impl WriterGraphContext {
    pub(crate) fn new(arena: Rc<GraphArena>) -> Self {
        Self {
            arena,
            references: HashMap::new()
        }
    }
}
