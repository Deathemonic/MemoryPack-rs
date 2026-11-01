use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::sync::atomic::{AtomicUsize, Ordering};

use crate::error::MemoryPackError;

static NEXT_ARENA_ID: AtomicUsize = AtomicUsize::new(1);

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct ReferenceKey {
    pub arena: usize,
    pub index: usize
}

pub struct GraphArena {
    id: usize,
    nodes: Vec<NodeSlot>,
    references: HashMap<u32, usize>
}

struct NodeSlot {
    node_type: TypeId,
    value: Option<Box<dyn Any>>
}

impl ReferenceKey {
    pub fn error_id(self) -> u32 { u32::try_from(self.index).unwrap_or(u32::MAX) }
}

impl GraphArena {
    pub fn new() -> Self {
        Self {
            id: NEXT_ARENA_ID.fetch_add(1, Ordering::Relaxed),
            nodes: Vec::new(),
            references: HashMap::new()
        }
    }

    pub fn insert<T: 'static>(&mut self, value: T) -> ReferenceKey {
        let key = self.allocate::<T>();
        self.nodes[key.index].value = Some(Box::new(value));

        key
    }

    pub fn reserve<T: 'static>(
        &mut self,
        reference_id: u32
    ) -> Result<ReferenceKey, MemoryPackError> {
        if self.references.contains_key(&reference_id) {
            return Err(MemoryPackError::ObjectReferenceAlreadyAdded(reference_id));
        }

        let key = self.allocate::<T>();
        self.references.insert(reference_id, key.index);

        Ok(key)
    }

    pub fn resolve<T: 'static>(&self, reference_id: u32) -> Result<ReferenceKey, MemoryPackError> {
        let index = self
            .references
            .get(&reference_id)
            .copied()
            .ok_or(MemoryPackError::ObjectReferenceNotFound(reference_id))?;
        let slot = &self.nodes[index];

        if slot.node_type != TypeId::of::<T>() {
            return Err(MemoryPackError::ObjectReferenceNotFound(reference_id));
        }

        Ok(ReferenceKey { arena: self.id, index })
    }

    pub fn initialize<T: 'static>(
        &mut self,
        key: ReferenceKey,
        value: T
    ) -> Result<(), MemoryPackError> {
        self.validate(key)?;
        let slot = &mut self.nodes[key.index];

        if slot.node_type != TypeId::of::<T>() || slot.value.is_some() {
            return Err(MemoryPackError::ObjectReferenceAlreadyAdded(key.error_id()));
        }

        slot.value = Some(Box::new(value));

        Ok(())
    }

    pub fn get<T: 'static>(&self, key: ReferenceKey) -> Result<&T, MemoryPackError> {
        self.validate(key)?;

        self.nodes[key.index]
            .value
            .as_ref()
            .and_then(|value| value.downcast_ref())
            .ok_or(MemoryPackError::ObjectReferenceNotFound(key.error_id()))
    }

    pub fn get_mut<T: 'static>(&mut self, key: ReferenceKey) -> Result<&mut T, MemoryPackError> {
        self.validate(key)?;

        self.nodes[key.index]
            .value
            .as_mut()
            .and_then(|value| value.downcast_mut())
            .ok_or(MemoryPackError::ObjectReferenceNotFound(key.error_id()))
    }

    fn allocate<T: 'static>(&mut self) -> ReferenceKey {
        let key = ReferenceKey {
            arena: self.id,
            index: self.nodes.len()
        };
        self.nodes.push(NodeSlot {
            node_type: TypeId::of::<T>(),
            value: None
        });

        key
    }

    fn validate(&self, key: ReferenceKey) -> Result<(), MemoryPackError> {
        if key.arena != self.id || key.index >= self.nodes.len() {
            return Err(MemoryPackError::ObjectReferenceNotFound(key.error_id()));
        }

        Ok(())
    }
}
