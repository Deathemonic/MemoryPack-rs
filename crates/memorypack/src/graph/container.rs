use std::rc::Rc;

use crate::error::MemoryPackError;
use crate::graph::arena::GraphArena;
use crate::graph::{MemoryPackCircular, MemoryPackReference};
use crate::reader::MemoryPackReader;
use crate::state::{
    MemoryPackReaderOptionalState,
    MemoryPackWriterOptionalState,
    WriterGraphContext
};
use crate::traits::{MemoryPackDeserialize, MemoryPackSerialize};
use crate::writer::MemoryPackWriter;

pub struct MemoryPackGraph<T> {
    pub root: MemoryPackReference<T>,
    arena: Rc<GraphArena>
}

impl<T: 'static> MemoryPackGraph<T> {
    pub fn new(value: T) -> Self {
        let mut arena = GraphArena::new();
        let root = MemoryPackReference::from_key(arena.insert(value));

        Self {
            root,
            arena: Rc::new(arena)
        }
    }

    pub fn insert<N: 'static>(
        &mut self,
        value: N
    ) -> Result<MemoryPackReference<N>, MemoryPackError> {
        let arena = Rc::get_mut(&mut self.arena)
            .ok_or(MemoryPackError::ObjectReferenceUpdateNotFound(self.root.key.error_id()))?;

        Ok(MemoryPackReference::from_key(arena.insert(value)))
    }

    pub fn get<N: 'static>(
        &self,
        reference: MemoryPackReference<N>
    ) -> Result<&N, MemoryPackError> {
        self.arena.get(reference.key)
    }

    pub fn get_mut<N: 'static>(
        &mut self,
        reference: MemoryPackReference<N>
    ) -> Result<&mut N, MemoryPackError> {
        let arena = Rc::get_mut(&mut self.arena)
            .ok_or(MemoryPackError::ObjectReferenceUpdateNotFound(reference.key.error_id()))?;

        arena.get_mut(reference.key)
    }
}

impl<T: MemoryPackCircular + 'static> MemoryPackSerialize for MemoryPackGraph<T> {
    fn serialize(&self, writer: &mut MemoryPackWriter) -> Result<(), MemoryPackError> {
        let state = writer
            .optional_state
            .get_or_insert_with(MemoryPackWriterOptionalState::new);
        let previous = state
            .graph
            .replace(Box::new(WriterGraphContext::new(Rc::clone(&self.arena))));
        let result = self.root.serialize(writer);
        let state = writer
            .optional_state
            .get_or_insert_with(MemoryPackWriterOptionalState::new);
        state.graph = previous;

        result
    }
}

impl<'de, T: MemoryPackCircular + 'static> MemoryPackDeserialize<'de> for MemoryPackGraph<T> {
    fn deserialize(reader: &mut MemoryPackReader<'de>) -> Result<Self, MemoryPackError> {
        let previous = reader
            .optional_state
            .get_or_insert_with(MemoryPackReaderOptionalState::new)
            .graph
            .replace(Box::new(GraphArena::new()));
        let result = MemoryPackReference::<T>::deserialize(reader);
        let arena = reader
            .optional_state
            .get_or_insert_with(MemoryPackReaderOptionalState::new)
            .graph
            .take();
        reader
            .optional_state
            .get_or_insert_with(MemoryPackReaderOptionalState::new)
            .graph = previous;
        let root = result?;
        let arena = arena.ok_or(MemoryPackError::ObjectReferenceNotFound(root.key.error_id()))?;

        Ok(Self {
            root,
            arena: Rc::from(arena)
        })
    }
}
