use criterion::{black_box, criterion_group, criterion_main, Criterion};
use memorypack::prelude::*;
use hashbrown::HashMap;
use std::alloc::{GlobalAlloc, Layout, System};
use std::sync::atomic::{AtomicUsize, Ordering};

struct CountingAlloc;

static ALLOCATED: AtomicUsize = AtomicUsize::new(0);
static DEALLOCATED: AtomicUsize = AtomicUsize::new(0);

unsafe impl GlobalAlloc for CountingAlloc {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let ret = unsafe { System.alloc(layout) };
        if !ret.is_null() {
            ALLOCATED.fetch_add(layout.size(), Ordering::SeqCst);
        }
        ret
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        unsafe { System.dealloc(ptr, layout) };
        DEALLOCATED.fetch_add(layout.size(), Ordering::SeqCst);
    }
}

#[global_allocator]
static GLOBAL: CountingAlloc = CountingAlloc;

#[derive(MemoryPackable, Clone, Default)]
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

#[derive(MemoryPackable, Clone, Copy)]
#[repr(i32)]
enum Color {
    Red = 0,
    Green = 1,
    Blue = 2,
}

#[derive(MemoryPackable, Clone)]
struct FooClass {
    xyz: i32,
}

#[derive(MemoryPackable, Clone)]
struct BarClass {
    opq: String,
}

#[derive(MemoryPackable, Clone)]
#[memorypack(union)]
enum UnionSample {
    Foo(FooClass),
    Bar(BarClass),
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

fn create_union_data() -> UnionSample {
    UnionSample::Foo(FooClass { xyz: 999 })
}



fn reset_counters() {
    ALLOCATED.store(0, Ordering::SeqCst);
    DEALLOCATED.store(0, Ordering::SeqCst);
}

fn get_net_allocated() -> usize {
    let allocated = ALLOCATED.load(Ordering::SeqCst);
    let deallocated = DEALLOCATED.load(Ordering::SeqCst);
    allocated.saturating_sub(deallocated)
}

fn measure_allocations<F, T>(name: &str, f: F) -> T
where
    F: FnOnce() -> T,
{
    reset_counters();
    let result = f();
    let net_allocated = get_net_allocated();
    
    println!("{}: {} bytes allocated (net)", name, net_allocated);
    result
}

fn benchmark_serialize_simple(c: &mut Criterion) {
    let data = create_simple_data();
    
    let bytes = measure_allocations("serialize_simple", || {
        MemoryPackSerializer::serialize(&data).unwrap()
    });
    println!("serialize_simple output size: {} bytes\n", bytes.len());
    
    c.bench_function("serialize_simple", |b| {
        b.iter(|| MemoryPackSerializer::serialize(black_box(&data)).unwrap())
    });
}

fn benchmark_deserialize_simple(c: &mut Criterion) {
    let data = create_simple_data();
    let bytes = MemoryPackSerializer::serialize(&data).unwrap();
    
    measure_allocations("deserialize_simple", || {
        MemoryPackSerializer::deserialize::<SimpleData>(&bytes).unwrap()
    });
    println!();
    
    c.bench_function("deserialize_simple", |b| {
        b.iter(|| MemoryPackSerializer::deserialize::<SimpleData>(black_box(&bytes)).unwrap())
    });
}

fn benchmark_serialize_complex(c: &mut Criterion) {
    let data = create_complex_data();
    
    let bytes = measure_allocations("serialize_complex", || {
        MemoryPackSerializer::serialize(&data).unwrap()
    });
    println!("serialize_complex output size: {} bytes\n", bytes.len());
    
    c.bench_function("serialize_complex", |b| {
        b.iter(|| MemoryPackSerializer::serialize(black_box(&data)).unwrap())
    });
}

fn benchmark_deserialize_complex(c: &mut Criterion) {
    let data = create_complex_data();
    let bytes = MemoryPackSerializer::serialize(&data).unwrap();
    
    measure_allocations("deserialize_complex", || {
        MemoryPackSerializer::deserialize::<ComplexData>(&bytes).unwrap()
    });
    println!();
    
    c.bench_function("deserialize_complex", |b| {
        b.iter(|| MemoryPackSerializer::deserialize::<ComplexData>(black_box(&bytes)).unwrap())
    });
}

fn benchmark_serialize_version_tolerant(c: &mut Criterion) {
    let data = create_version_tolerant_data();
    
    let bytes = measure_allocations("serialize_version_tolerant", || {
        MemoryPackSerializer::serialize(&data).unwrap()
    });
    println!("serialize_version_tolerant output size: {} bytes\n", bytes.len());
    
    c.bench_function("serialize_version_tolerant", |b| {
        b.iter(|| MemoryPackSerializer::serialize(black_box(&data)).unwrap())
    });
}

fn benchmark_deserialize_version_tolerant(c: &mut Criterion) {
    let data = create_version_tolerant_data();
    let bytes = MemoryPackSerializer::serialize(&data).unwrap();
    
    measure_allocations("deserialize_version_tolerant", || {
        MemoryPackSerializer::deserialize::<VersionTolerantData>(&bytes).unwrap()
    });
    println!();
    
    c.bench_function("deserialize_version_tolerant", |b| {
        b.iter(|| MemoryPackSerializer::deserialize::<VersionTolerantData>(black_box(&bytes)).unwrap())
    });
}

fn benchmark_serialize_enum(c: &mut Criterion) {
    let data = Color::Green;
    
    c.bench_function("serialize_enum", |b| {
        b.iter(|| MemoryPackSerializer::serialize(black_box(&data)).unwrap())
    });
}

fn benchmark_deserialize_enum(c: &mut Criterion) {
    let data = Color::Green;
    let bytes = MemoryPackSerializer::serialize(&data).unwrap();
    
    c.bench_function("deserialize_enum", |b| {
        b.iter(|| MemoryPackSerializer::deserialize::<Color>(black_box(&bytes)).unwrap())
    });
}

fn benchmark_serialize_union(c: &mut Criterion) {
    let data = create_union_data();
    
    c.bench_function("serialize_union", |b| {
        b.iter(|| MemoryPackSerializer::serialize(black_box(&data)).unwrap())
    });
}

fn benchmark_deserialize_union(c: &mut Criterion) {
    let data = create_union_data();
    let bytes = MemoryPackSerializer::serialize(&data).unwrap();
    
    c.bench_function("deserialize_union", |b| {
        b.iter(|| MemoryPackSerializer::deserialize::<UnionSample>(black_box(&bytes)).unwrap())
    });
}



criterion_group!(
    benches,
    benchmark_serialize_simple,
    benchmark_deserialize_simple,
    benchmark_serialize_complex,
    benchmark_deserialize_complex,
    benchmark_serialize_version_tolerant,
    benchmark_deserialize_version_tolerant,
    benchmark_serialize_enum,
    benchmark_deserialize_enum,
    benchmark_serialize_union,
    benchmark_deserialize_union
);
criterion_main!(benches);
