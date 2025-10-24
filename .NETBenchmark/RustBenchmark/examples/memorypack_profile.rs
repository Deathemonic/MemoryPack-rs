use memorypack::prelude::*;
use hashbrown::HashMap;

#[cfg(feature = "dhat-heap")]
#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

#[derive(MemoryPackable, Clone)]
struct SimpleData {
    id: i32,
    name: String,
    value: f64,
    is_active: bool,
}

#[derive(MemoryPackable, Clone)]
struct ComplexData {
    id: i32,
    name: String,
    numbers: Vec<i32>,
    properties: HashMap<String, String>,
    nested: Option<SimpleData>,
}

#[derive(MemoryPackable, Clone)]
#[memorypack(version_tolerant)]
struct VersionTolerantData {
    #[memorypack(order = 0)]
    property1: i32,
    #[memorypack(order = 1)]
    property2: String,
    #[memorypack(order = 2)]
    property3: f64,
}

#[derive(MemoryPackable)]
#[memorypack(zero_copy)]
struct ZeroCopyData<'a> {
    id: i32,
    name: &'a str,
    description: &'a str,
}

fn create_simple_data() -> SimpleData {
    SimpleData {
        id: 42,
        name: "Test Data".to_string(),
        value: 3.14159,
        is_active: true,
    }
}

fn create_complex_data() -> ComplexData {
    ComplexData {
        id: 100,
        name: "Complex Test".to_string(),
        numbers: (1..=100).collect(),
        properties: (1..=50)
            .map(|i| (format!("key{}", i), format!("value{}", i)))
            .collect(),
        nested: Some(SimpleData {
            id: 1,
            name: "Nested".to_string(),
            value: 1.23,
            is_active: false,
        }),
    }
}

fn create_version_tolerant_data() -> VersionTolerantData {
    VersionTolerantData {
        property1: 1000,
        property2: "Version Tolerant".to_string(),
        property3: 99.99,
    }
}

fn main() {
    #[cfg(feature = "dhat-heap")]
    let _profiler = dhat::Profiler::new_heap();
    
    let simple_data = create_simple_data();
    for _ in 0..100_000 {
        let _bytes = MemoryPackSerializer::serialize(&simple_data).unwrap();
    }
    
    let simple_bytes = MemoryPackSerializer::serialize(&simple_data).unwrap();
    for _ in 0..100_000 {
        let _data: SimpleData = MemoryPackSerializer::deserialize(&simple_bytes).unwrap();
    }
    
    let complex_data = create_complex_data();
    for _ in 0..10_000 {
        let _bytes = MemoryPackSerializer::serialize(&complex_data).unwrap();
    }
    
    let complex_bytes = MemoryPackSerializer::serialize(&complex_data).unwrap();
    for _ in 0..10_000 {
        let _data: ComplexData = MemoryPackSerializer::deserialize(&complex_bytes).unwrap();
    }
    
    let vt_data = create_version_tolerant_data();
    for _ in 0..100_000 {
        let _bytes = MemoryPackSerializer::serialize(&vt_data).unwrap();
    }
    
    let vt_bytes = MemoryPackSerializer::serialize(&vt_data).unwrap();
    for _ in 0..100_000 {
        let _data: VersionTolerantData = MemoryPackSerializer::deserialize(&vt_bytes).unwrap();
    }
    
    let zc_owned = SimpleData {
        id: 42,
        name: "Zero Copy Test".to_string(),
        value: 0.0,
        is_active: true,
    };
    let zc_bytes = MemoryPackSerializer::serialize(&zc_owned).unwrap();
    for _ in 0..100_000 {
        let _data: ZeroCopyData = MemoryPackSerializer::deserialize(&zc_bytes).unwrap();
    }
}

