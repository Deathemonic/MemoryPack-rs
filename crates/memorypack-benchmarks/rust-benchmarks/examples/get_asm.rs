use std::collections::HashMap;

use memorypack::prelude::*;

#[derive(MemoryPackable, Clone, Default)]
struct SimpleData {
    id: i32,
    name: String,
    value: f64,
    is_active: bool
}

#[derive(MemoryPackable, Clone)]
struct ComplexData {
    id: i32,
    name: String,
    numbers: Vec<i32>,
    properties: HashMap<String, String>,
    nested: Option<SimpleData>
}

#[derive(MemoryPackable, Clone, Default)]
struct SimpleDataNoString {
    id: i32,
    value: f64,
    is_active: bool
}

#[derive(MemoryPackable, Clone)]
struct ComplexDataNoString {
    id: i32,
    numbers: Vec<i32>,
    properties: HashMap<i32, i32>,
    nested: Option<SimpleDataNoString>
}

fn main() {
    let simple_data = SimpleData {
        id: 42,
        name: "Test Data".to_string(),
        value: 3.14159,
        is_active: true
    };

    let serialize = MemoryPackSerializer::serialize(&simple_data).unwrap();
    let _deserialize: SimpleData = MemoryPackSerializer::deserialize(&serialize).unwrap();

    let complex_data = ComplexData {
        id: 100,
        name: "Complex Test".to_string(),
        numbers: (1..=100).collect(),
        properties: (1..=50).map(|i| (format!("key{}", i), format!("value{}", i))).collect(),
        nested: Some(SimpleData {
            id: 1,
            name: "Nested".to_string(),
            value: 1.23,
            is_active: false
        })
    };

    let complex_serialize = MemoryPackSerializer::serialize(&complex_data).unwrap();
    let _complex_deserialize: ComplexData =
        MemoryPackSerializer::deserialize(&complex_serialize).unwrap();

    let simple_data_no_string = SimpleDataNoString {
        id: 42,
        value: 3.14159,
        is_active: true
    };

    let serialize_no_string = MemoryPackSerializer::serialize(&simple_data_no_string).unwrap();
    let _deserialize_no_string: SimpleDataNoString =
        MemoryPackSerializer::deserialize(&serialize_no_string).unwrap();

    let complex_data_no_string = ComplexDataNoString {
        id: 100,
        numbers: (1..=100).collect(),
        properties: (1..=50).map(|i| (i, i * 10)).collect(),
        nested: Some(SimpleDataNoString {
            id: 1,
            value: 1.23,
            is_active: false
        })
    };

    let complex_serialize_no_string =
        MemoryPackSerializer::serialize(&complex_data_no_string).unwrap();
    let _complex_deserialize_no_string: ComplexDataNoString =
        MemoryPackSerializer::deserialize(&complex_serialize_no_string).unwrap();
}
