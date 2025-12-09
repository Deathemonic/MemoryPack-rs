use memorypack::prelude::*;

#[derive(MemoryPackable, Clone, Default)]
struct SimpleData {
    id: i32,
    name: String,
    value: f64,
    is_active: bool,
}

fn main() {
    let simple_data = SimpleData {
        id: 42,
        name: "Test Data".to_string(),
        value: 3.14159,
        is_active: true,
    };

    let serialize = MemoryPackSerializer::serialize(&simple_data).unwrap();
    let _deserialize: SimpleData = MemoryPackSerializer::deserialize(&serialize).unwrap();
}