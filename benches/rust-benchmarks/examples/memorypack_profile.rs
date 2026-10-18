use std::collections::HashMap;

use memorypack::prelude::*;

const SAMPLE_DOUBLE: f64 = 314_159.0 / 100_000.0;

#[cfg(all(feature = "dhat-heap", not(feature = "mimalloc")))]
#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

#[derive(MemoryPackable, Clone, Copy)]
#[repr(i32)]
enum Color {
    Red = 0,
    Green = 1,
    Blue = 2
}

#[derive(MemoryPackable, Clone)]
#[memorypack(union)]
enum UnionSample {
    Foo(FooClass),
    Bar(BarClass)
}

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

#[derive(MemoryPackable, Clone)]
#[memorypack(version_tolerant)]
struct VersionTolerantData {
    #[memorypack(order = 0)]
    property1: i32,
    #[memorypack(order = 1)]
    property2: String,
    #[memorypack(order = 2)]
    property3: f64
}

#[derive(MemoryPackable)]
struct ZeroCopyData<'a> {
    id: i32,
    name: &'a str,
    description: &'a str
}

#[derive(MemoryPackable, Clone)]
struct FooClass {
    xyz: i32
}

#[derive(MemoryPackable, Clone)]
struct BarClass {
    opq: String
}

fn create_simple_data() -> SimpleData {
    SimpleData {
        id: 42,
        name: "Test Data".to_string(),
        value: SAMPLE_DOUBLE,
        is_active: true
    }
}

fn create_complex_data() -> ComplexData {
    ComplexData {
        id: 100,
        name: "Complex Test".to_string(),
        numbers: (1..=100).collect(),
        properties: (1..=50)
            .map(|i| (format!("key{i}"), format!("value{i}")))
            .collect(),
        nested: Some(SimpleData {
            id: 1,
            name: "Nested".to_string(),
            value: 1.23,
            is_active: false
        })
    }
}

const fn create_simple_data_no_string() -> SimpleDataNoString {
    SimpleDataNoString {
        id: 42,
        value: SAMPLE_DOUBLE,
        is_active: true
    }
}

fn create_complex_data_no_string() -> ComplexDataNoString {
    ComplexDataNoString {
        id: 100,
        numbers: (1..=100).collect(),
        properties: (1..=50).map(|i| (i, i * 10)).collect(),
        nested: Some(SimpleDataNoString {
            id: 1,
            value: 1.23,
            is_active: false
        })
    }
}

fn create_version_tolerant_data() -> VersionTolerantData {
    VersionTolerantData {
        property1: 1000,
        property2: "Version Tolerant".to_string(),
        property3: 99.99
    }
}

fn main() -> Result<(), MemoryPackError> {
    #[cfg(all(feature = "dhat-heap", not(feature = "mimalloc")))]
    let _profiler = dhat::Profiler::new_heap();

    let simple_data = create_simple_data();

    for _ in 0..100_000 {
        let _bytes = MemoryPackSerializer::serialize(&simple_data)?;
    }

    let simple_bytes = MemoryPackSerializer::serialize(&simple_data)?;

    for _ in 0..100_000 {
        let _data: SimpleData = MemoryPackSerializer::deserialize(&simple_bytes)?;
    }

    let complex_data = create_complex_data();

    for _ in 0..10_000 {
        let _bytes = MemoryPackSerializer::serialize(&complex_data)?;
    }

    let complex_bytes = MemoryPackSerializer::serialize(&complex_data)?;

    for _ in 0..10_000 {
        let _data: ComplexData = MemoryPackSerializer::deserialize(&complex_bytes)?;
    }

    let simple_data_no_string = create_simple_data_no_string();

    for _ in 0..100_000 {
        let _bytes = MemoryPackSerializer::serialize(&simple_data_no_string)?;
    }

    let simple_no_string_bytes = MemoryPackSerializer::serialize(&simple_data_no_string)?;

    for _ in 0..100_000 {
        let _data: SimpleDataNoString = MemoryPackSerializer::deserialize(&simple_no_string_bytes)?;
    }

    let complex_data_no_string = create_complex_data_no_string();

    for _ in 0..10_000 {
        let _bytes = MemoryPackSerializer::serialize(&complex_data_no_string)?;
    }

    let complex_no_string_bytes = MemoryPackSerializer::serialize(&complex_data_no_string)?;

    for _ in 0..10_000 {
        let _data: ComplexDataNoString =
            MemoryPackSerializer::deserialize(&complex_no_string_bytes)?;
    }

    let vt_data = create_version_tolerant_data();

    for _ in 0..100_000 {
        let _bytes = MemoryPackSerializer::serialize(&vt_data)?;
    }

    let vt_bytes = MemoryPackSerializer::serialize(&vt_data)?;

    for _ in 0..100_000 {
        let _data: VersionTolerantData = MemoryPackSerializer::deserialize(&vt_bytes)?;
    }

    let zc_owned = SimpleData {
        id: 42,
        name: "Zero Copy Test".to_string(),
        value: 0.0,
        is_active: true
    };
    let zc_bytes = MemoryPackSerializer::serialize(&zc_owned)?;

    for _ in 0..100_000 {
        let _data: ZeroCopyData = MemoryPackSerializer::deserialize(&zc_bytes)?;
    }

    let enum_data = Color::Green;

    for _ in 0..100_000 {
        let _bytes = MemoryPackSerializer::serialize(&enum_data)?;
    }

    let enum_bytes = MemoryPackSerializer::serialize(&enum_data)?;

    for _ in 0..100_000 {
        let _data: Color = MemoryPackSerializer::deserialize(&enum_bytes)?;
    }

    let union_data = UnionSample::Foo(FooClass { xyz: 999 });

    for _ in 0..100_000 {
        let _bytes = MemoryPackSerializer::serialize(&union_data)?;
    }

    let union_bytes = MemoryPackSerializer::serialize(&union_data)?;

    for _ in 0..100_000 {
        let _data: UnionSample = MemoryPackSerializer::deserialize(&union_bytes)?;
    }

    Ok(())
}
