use criterion::{black_box, criterion_group, criterion_main, Criterion};
use memorypack::prelude::*;
use std::collections::HashMap;

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

fn benchmark_serialize_simple(c: &mut Criterion) {
    let data = create_simple_data();
    c.bench_function("serialize_simple", |b| {
        b.iter(|| MemoryPackSerializer::serialize(black_box(&data)).unwrap())
    });
}

fn benchmark_deserialize_simple(c: &mut Criterion) {
    let data = create_simple_data();
    let bytes = MemoryPackSerializer::serialize(&data).unwrap();
    c.bench_function("deserialize_simple", |b| {
        b.iter(|| MemoryPackSerializer::deserialize::<SimpleData>(black_box(&bytes)).unwrap())
    });
}

fn benchmark_serialize_complex(c: &mut Criterion) {
    let data = create_complex_data();
    c.bench_function("serialize_complex", |b| {
        b.iter(|| MemoryPackSerializer::serialize(black_box(&data)).unwrap())
    });
}

fn benchmark_deserialize_complex(c: &mut Criterion) {
    let data = create_complex_data();
    let bytes = MemoryPackSerializer::serialize(&data).unwrap();
    c.bench_function("deserialize_complex", |b| {
        b.iter(|| MemoryPackSerializer::deserialize::<ComplexData>(black_box(&bytes)).unwrap())
    });
}

fn benchmark_serialize_version_tolerant(c: &mut Criterion) {
    let data = create_version_tolerant_data();
    c.bench_function("serialize_version_tolerant", |b| {
        b.iter(|| MemoryPackSerializer::serialize(black_box(&data)).unwrap())
    });
}

fn benchmark_deserialize_version_tolerant(c: &mut Criterion) {
    let data = create_version_tolerant_data();
    let bytes = MemoryPackSerializer::serialize(&data).unwrap();
    c.bench_function("deserialize_version_tolerant", |b| {
        b.iter(|| MemoryPackSerializer::deserialize::<VersionTolerantData>(black_box(&bytes)).unwrap())
    });
}

criterion_group!(
    benches,
    benchmark_serialize_simple,
    benchmark_deserialize_simple,
    benchmark_serialize_complex,
    benchmark_deserialize_complex,
    benchmark_serialize_version_tolerant,
    benchmark_deserialize_version_tolerant
);
criterion_main!(benches);

