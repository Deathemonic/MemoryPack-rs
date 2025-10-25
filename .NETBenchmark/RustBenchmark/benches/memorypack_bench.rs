use criterion::{black_box, criterion_group, criterion_main, Criterion};
use memorypack::prelude::*;
use hashbrown::HashMap;

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

struct BenchmarkData {
    simple: SimpleData,
    simple_bytes: Vec<u8>,
    complex: ComplexData,
    complex_bytes: Vec<u8>,
    version_tolerant: VersionTolerantData,
    version_tolerant_bytes: Vec<u8>,
}

impl BenchmarkData {
    fn new() -> Self {
        let simple = SimpleData {
            id: 42,
            name: "Test Data".to_string(),
            value: 3.14159,
            is_active: true,
        };

        let complex = ComplexData {
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
        };

        let version_tolerant = VersionTolerantData {
            property1: 1000,
            property2: "Version Tolerant".to_string(),
            property3: 99.99,
        };

        let simple_bytes = MemoryPackSerializer::serialize(&simple).unwrap();
        let complex_bytes = MemoryPackSerializer::serialize(&complex).unwrap();
        let version_tolerant_bytes = MemoryPackSerializer::serialize(&version_tolerant).unwrap();

        println!("Simple output size: {} bytes", simple_bytes.len());
        println!("Complex output size: {} bytes", complex_bytes.len());
        println!("Version tolerant output size: {} bytes", version_tolerant_bytes.len());

        Self {
            simple,
            simple_bytes,
            complex,
            complex_bytes,
            version_tolerant,
            version_tolerant_bytes,
        }
    }
}

fn benchmark_serialize_simple(c: &mut Criterion) {
    let data = BenchmarkData::new();
    
    c.bench_function("serialize_simple", |b| {
        b.iter(|| {
            let _ = MemoryPackSerializer::serialize(black_box(&data.simple)).unwrap();
        })
    });
}

fn benchmark_deserialize_simple(c: &mut Criterion) {
    let data = BenchmarkData::new();
    
    c.bench_function("deserialize_simple", |b| {
        b.iter(|| {
            let _ = MemoryPackSerializer::deserialize::<SimpleData>(
                black_box(&data.simple_bytes)
            ).unwrap();
        })
    });
}

fn benchmark_serialize_complex(c: &mut Criterion) {
    let data = BenchmarkData::new();
    
    c.bench_function("serialize_complex", |b| {
        b.iter(|| {
            let _ = MemoryPackSerializer::serialize(black_box(&data.complex)).unwrap();
        })
    });
}

fn benchmark_deserialize_complex(c: &mut Criterion) {
    let data = BenchmarkData::new();
    
    c.bench_function("deserialize_complex", |b| {
        b.iter(|| {
            let _ = MemoryPackSerializer::deserialize::<ComplexData>(
                black_box(&data.complex_bytes)
            ).unwrap();
        })
    });
}

fn benchmark_serialize_version_tolerant(c: &mut Criterion) {
    let data = BenchmarkData::new();
    
    c.bench_function("serialize_version_tolerant", |b| {
        b.iter(|| {
            let _ = MemoryPackSerializer::serialize(black_box(&data.version_tolerant)).unwrap();
        })
    });
}

fn benchmark_deserialize_version_tolerant(c: &mut Criterion) {
    let data = BenchmarkData::new();
    
    c.bench_function("deserialize_version_tolerant", |b| {
        b.iter(|| {
            let _ = MemoryPackSerializer::deserialize::<VersionTolerantData>(
                black_box(&data.version_tolerant_bytes)
            ).unwrap();
        })
    });
}

fn configure_criterion() -> Criterion {
    Criterion::default()
        .sample_size(100)
        .measurement_time(std::time::Duration::from_secs(10))
        .warm_up_time(std::time::Duration::from_secs(3))
}

criterion_group! {
    name = benches;
    config = configure_criterion();
    targets = 
        benchmark_serialize_simple,
        benchmark_deserialize_simple,
        benchmark_serialize_complex,
        benchmark_deserialize_complex,
        benchmark_serialize_version_tolerant,
        benchmark_deserialize_version_tolerant
}

criterion_main!(benches);
