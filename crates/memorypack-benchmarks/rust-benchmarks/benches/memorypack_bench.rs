use std::collections::HashMap;

use divan::{Bencher, black_box};
use memorypack::prelude::*;

#[global_allocator]
static ALLOC: divan::AllocProfiler = divan::AllocProfiler::system();

fn main() { divan::main(); }

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

#[derive(MemoryPackable, Clone, Copy)]
#[repr(i32)]
enum Color {
    Red = 0,
    Green = 1,
    Blue = 2
}

#[derive(MemoryPackable, Clone)]
struct FooClass {
    xyz: i32
}

#[derive(MemoryPackable, Clone)]
struct BarClass {
    opq: String
}

#[derive(MemoryPackable, Clone)]
#[memorypack(union)]
enum UnionSample {
    Foo(FooClass),
    Bar(BarClass)
}

#[derive(MemoryPackable, Clone)]
#[memorypack(zero_copy)]
struct ZeroCopyData<'a> {
    id: i32,
    name: &'a str,
    value: f64,
    is_active: bool
}

#[derive(MemoryPackable, Clone)]
#[memorypack(zero_copy)]
struct ZeroCopyDataLarge<'a> {
    id: i32,
    name: &'a str,
    description: &'a str,
    value: f64,
    is_active: bool
}

fn create_simple_data() -> SimpleData {
    SimpleData {
        id: 42,
        name: "Test Data".to_string(),
        value: 3.14159,
        is_active: true
    }
}

fn create_complex_data() -> ComplexData {
    ComplexData {
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
    }
}

fn create_simple_data_no_string() -> SimpleDataNoString {
    SimpleDataNoString {
        id: 42,
        value: 3.14159,
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

fn create_union_data() -> UnionSample { UnionSample::Foo(FooClass { xyz: 999 }) }

fn create_zero_copy_data() -> ZeroCopyData<'static> {
    ZeroCopyData {
        id: 42,
        name: "Test Data",
        value: 3.14159,
        is_active: true
    }
}

const LARGE_TEXT: &str = "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat.";

fn create_zero_copy_data_large() -> ZeroCopyDataLarge<'static> {
    ZeroCopyDataLarge {
        id: 100,
        name: "Large Test Data With A Longer Name Field",
        description: LARGE_TEXT,
        value: 2.71828,
        is_active: true
    }
}

#[divan::bench]
fn serialize_simple(bencher: Bencher) {
    let data = create_simple_data();
    bencher.bench_local(|| MemoryPackSerializer::serialize(black_box(&data)).unwrap());
}

#[divan::bench]
fn serialize_to_simple(bencher: Bencher) {
    let data = create_simple_data();
    let mut writer = MemoryPackWriter::with_capacity(data.serialized_size_hint());
    bencher.bench_local(|| {
        writer.reset();
        MemoryPackSerializer::serialize_to(black_box(&data), &mut writer).unwrap();
        black_box(writer.as_bytes());
    });
}

#[divan::bench]
fn deserialize_simple(bencher: Bencher) {
    let data = create_simple_data();
    let bytes = MemoryPackSerializer::serialize(&data).unwrap();
    bencher.bench_local(|| {
        MemoryPackSerializer::deserialize::<SimpleData>(black_box(&bytes)).unwrap()
    });
}

#[divan::bench]
fn serialize_complex(bencher: Bencher) {
    let data = create_complex_data();
    bencher.bench_local(|| MemoryPackSerializer::serialize(black_box(&data)).unwrap());
}

#[divan::bench]
fn serialize_to_complex(bencher: Bencher) {
    let data = create_complex_data();
    let mut writer = MemoryPackWriter::with_capacity(data.serialized_size_hint());
    bencher.bench_local(|| {
        writer.reset();
        MemoryPackSerializer::serialize_to(black_box(&data), &mut writer).unwrap();
        black_box(writer.as_bytes());
    });
}

#[divan::bench]
fn deserialize_complex(bencher: Bencher) {
    let data = create_complex_data();
    let bytes = MemoryPackSerializer::serialize(&data).unwrap();
    bencher.bench_local(|| {
        MemoryPackSerializer::deserialize::<ComplexData>(black_box(&bytes)).unwrap()
    });
}

#[divan::bench]
fn serialize_simple_no_string(bencher: Bencher) {
    let data = create_simple_data_no_string();
    bencher.bench_local(|| MemoryPackSerializer::serialize(black_box(&data)).unwrap());
}

#[divan::bench]
fn deserialize_simple_no_string(bencher: Bencher) {
    let data = create_simple_data_no_string();
    let bytes = MemoryPackSerializer::serialize(&data).unwrap();
    bencher.bench_local(|| {
        MemoryPackSerializer::deserialize::<SimpleDataNoString>(black_box(&bytes)).unwrap()
    });
}

#[divan::bench]
fn serialize_complex_no_string(bencher: Bencher) {
    let data = create_complex_data_no_string();
    bencher.bench_local(|| MemoryPackSerializer::serialize(black_box(&data)).unwrap());
}

#[divan::bench]
fn serialize_to_complex_no_string(bencher: Bencher) {
    let data = create_complex_data_no_string();
    let mut writer = MemoryPackWriter::with_capacity(data.serialized_size_hint());
    bencher.bench_local(|| {
        writer.reset();
        MemoryPackSerializer::serialize_to(black_box(&data), &mut writer).unwrap();
        black_box(writer.as_bytes());
    });
}

#[divan::bench]
fn deserialize_complex_no_string(bencher: Bencher) {
    let data = create_complex_data_no_string();
    let bytes = MemoryPackSerializer::serialize(&data).unwrap();
    bencher.bench_local(|| {
        MemoryPackSerializer::deserialize::<ComplexDataNoString>(black_box(&bytes)).unwrap()
    });
}

#[divan::bench]
fn serialize_version_tolerant(bencher: Bencher) {
    let data = create_version_tolerant_data();
    bencher.bench_local(|| MemoryPackSerializer::serialize(black_box(&data)).unwrap());
}

#[divan::bench]
fn deserialize_version_tolerant(bencher: Bencher) {
    let data = create_version_tolerant_data();
    let bytes = MemoryPackSerializer::serialize(&data).unwrap();
    bencher.bench_local(|| {
        MemoryPackSerializer::deserialize::<VersionTolerantData>(black_box(&bytes)).unwrap()
    });
}

#[divan::bench]
fn serialize_enum(bencher: Bencher) {
    let data = Color::Green;
    bencher.bench_local(|| MemoryPackSerializer::serialize(black_box(&data)).unwrap());
}

#[divan::bench]
fn deserialize_enum(bencher: Bencher) {
    let data = Color::Green;
    let bytes = MemoryPackSerializer::serialize(&data).unwrap();
    bencher.bench_local(|| MemoryPackSerializer::deserialize::<Color>(black_box(&bytes)).unwrap());
}

#[divan::bench]
fn serialize_union(bencher: Bencher) {
    let data = create_union_data();
    bencher.bench_local(|| MemoryPackSerializer::serialize(black_box(&data)).unwrap());
}

#[divan::bench]
fn deserialize_union(bencher: Bencher) {
    let data = create_union_data();
    let bytes = MemoryPackSerializer::serialize(&data).unwrap();
    bencher.bench_local(|| {
        MemoryPackSerializer::deserialize::<UnionSample>(black_box(&bytes)).unwrap()
    });
}

#[divan::bench]
fn serialize_zero_copy(bencher: Bencher) {
    let data = create_zero_copy_data();
    bencher.bench_local(|| MemoryPackSerializer::serialize(black_box(&data)).unwrap());
}

#[divan::bench]
fn deserialize_zero_copy(bencher: Bencher) {
    let data = create_zero_copy_data();
    let bytes = MemoryPackSerializer::serialize(&data).unwrap();
    bencher.bench_local(|| {
        MemoryPackSerializer::deserialize_zero_copy::<ZeroCopyData>(black_box(&bytes)).unwrap()
    });
}

#[divan::bench]
fn serialize_zero_copy_large(bencher: Bencher) {
    let data = create_zero_copy_data_large();
    bencher.bench_local(|| MemoryPackSerializer::serialize(black_box(&data)).unwrap());
}

#[divan::bench]
fn deserialize_zero_copy_large(bencher: Bencher) {
    let data = create_zero_copy_data_large();
    let bytes = MemoryPackSerializer::serialize(&data).unwrap();
    bencher.bench_local(|| {
        MemoryPackSerializer::deserialize_zero_copy::<ZeroCopyDataLarge>(black_box(&bytes)).unwrap()
    });
}
