#![cfg(test)]

use std::cell::Cell;
use std::rc::Rc;

use memorypack::prelude::*;
use memorypack::{MemoryPackGraph, MemoryPackReference};

const NATIVE_SELF_REFERENCE: &[u8] = &[2, 4, 2, 0, 1, 0, 0, 0, 250, 0];
const DUPLICATE_REFERENCE: &[u8] = &[2, 4, 9, 0, 1, 0, 0, 0, 2, 4, 1, 0, 2, 0, 0, 0, 255];

#[derive(MemoryPackable, Clone)]
#[memorypack(circular)]
struct Node {
    value: i32,
    next: Option<MemoryPackReference<Self>>
}

#[derive(MemoryPackable, Clone)]
#[memorypack(circular)]
struct CountedNode {
    next: Option<MemoryPackReference<Self>>,
    #[memorypack(skip)]
    drops: Rc<Cell<usize>>
}

#[derive(MemoryPackable, Clone)]
#[memorypack(circular)]
struct NamedNode {
    name: String,
    next: Option<MemoryPackReference<Self>>
}

#[derive(MemoryPackable, Clone)]
#[memorypack(circular)]
struct TypedRoot {
    left: Option<MemoryPackReference<Node>>,
    right: Option<MemoryPackReference<NamedNode>>
}

impl Drop for CountedNode {
    fn drop(&mut self) { self.drops.set(self.drops.get() + 1); }
}

#[test]
fn rejects_unknown_reference() {
    let mut bytes = NATIVE_SELF_REFERENCE.to_vec();
    let last = bytes.len() - 1;
    bytes[last] = 42;

    assert!(matches!(
        MemoryPackSerializer::deserialize::<MemoryPackGraph<Node>>(&bytes),
        Err(MemoryPackError::ObjectReferenceNotFound(42))
    ));
}

#[test]
fn rejects_duplicate_reference() {
    assert!(matches!(
        MemoryPackSerializer::deserialize::<MemoryPackGraph<Node>>(DUPLICATE_REFERENCE),
        Err(MemoryPackError::ObjectReferenceAlreadyAdded(0))
    ));
}

#[test]
fn rejects_truncated_graph_without_panicking() {
    for end in 0..NATIVE_SELF_REFERENCE.len() {
        assert!(
            MemoryPackSerializer::deserialize::<MemoryPackGraph<Node>>(
                &NATIVE_SELF_REFERENCE[..end]
            )
            .is_err()
        );
    }
}

#[test]
fn rejects_references_from_another_graph() {
    let first = MemoryPackGraph::new(Node { value: 1, next: None });
    let second = MemoryPackGraph::new(Node {
        value: 2,
        next: Some(first.root)
    });

    assert!(second.get(first.root).is_err());
    assert!(MemoryPackSerializer::serialize(&second).is_err());
}

#[test]
fn drops_nodes_even_when_they_form_a_cycle() -> Result<(), MemoryPackError> {
    let drops = Rc::new(Cell::new(0));
    let mut graph = MemoryPackGraph::new(CountedNode {
        next: None,
        drops: Rc::clone(&drops)
    });
    let root = graph.root;
    graph.get_mut(root)?.next = Some(root);

    drop(graph);

    assert_eq!(drops.get(), 1);

    Ok(())
}

#[test]
fn restores_reader_graph_context_after_an_error() -> Result<(), MemoryPackError> {
    let mut reader = MemoryPackReader::new(DUPLICATE_REFERENCE);
    let result = MemoryPackGraph::<Node>::deserialize(&mut reader);

    assert!(result.is_err());
    reader.rewind(reader.position() as usize)?;
    let second_result = MemoryPackGraph::<Node>::deserialize(&mut reader);

    assert!(matches!(second_result, Err(MemoryPackError::ObjectReferenceAlreadyAdded(0))));

    Ok(())
}

#[test]
fn rejects_a_reference_to_the_wrong_node_type() {
    let bytes = [2, 9, 2, 0, 2, 4, 1, 1, 7, 0, 0, 0, 255, 250, 1];

    assert!(matches!(
        MemoryPackSerializer::deserialize::<MemoryPackGraph<TypedRoot>>(&bytes),
        Err(MemoryPackError::ObjectReferenceNotFound(1))
    ));
}

#[test]
fn preserves_reference_ids_when_field_lengths_need_varints() -> Result<(), MemoryPackError> {
    let name = "x".repeat(256);
    let mut graph = MemoryPackGraph::new(NamedNode {
        name: name.clone(),
        next: None
    });
    let root = graph.root;
    graph.get_mut(root)?.next = Some(root);
    let bytes = MemoryPackSerializer::serialize(&graph)?;
    let decoded = MemoryPackSerializer::deserialize::<MemoryPackGraph<NamedNode>>(&bytes)?;

    assert_eq!(decoded.get(decoded.root)?.name, name);
    assert_eq!(decoded.get(decoded.root)?.next, Some(decoded.root));
    assert_eq!(MemoryPackSerializer::serialize(&decoded)?, bytes);

    Ok(())
}

#[test]
fn graph_contexts_are_scoped_to_each_serialized_graph() -> Result<(), MemoryPackError> {
    let mut graph = MemoryPackGraph::new(Node { value: 1, next: None });
    let root = graph.root;
    graph.get_mut(root)?.next = Some(root);
    let mut writer = MemoryPackWriter::new();
    graph.serialize(&mut writer)?;
    graph.serialize(&mut writer)?;
    let bytes = writer.into_bytes();
    let mut reader = MemoryPackReader::new(&bytes);
    let first = MemoryPackGraph::<Node>::deserialize(&mut reader)?;
    let second = MemoryPackGraph::<Node>::deserialize(&mut reader)?;

    assert_eq!(first.get(first.root)?.next, Some(first.root));
    assert_eq!(second.get(second.root)?.next, Some(second.root));
    assert_ne!(first.root, second.root);
    assert_eq!(reader.position() as usize, bytes.len());

    Ok(())
}
