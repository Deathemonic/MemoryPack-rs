#![cfg(test)]

use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet, LinkedList, VecDeque};
use std::error::Error;
use std::fmt::Debug;
use std::net::Ipv4Addr;
use std::path::PathBuf;
use std::{env, fs, iter};

use jiff::civil::{Date, DateTime, Time};
use jiff::tz::{Offset, TimeZone};
use jiff::{SignedDuration, Timestamp, Zoned};
use memorypack::prelude::*;
use memorypack::{MemoryPackGraph, MemoryPackReference, MultiDimArray};

const SAMPLE_FLOAT: f32 = 314_159.0 / 100_000.0;
const IPV4_OCTET_COUNT: usize = 4;

type TestResult<T = ()> = Result<T, Box<dyn Error>>;

#[derive(MemoryPackable, Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
enum Color {
    Red,
    Green,
    Blue
}

#[derive(MemoryPackable, Debug, Clone)]
#[memorypack(union)]
enum UnionSample {
    Foo(FooClass),
    Bar(BarClass)
}

#[derive(MemoryPackable, Debug, Clone)]
#[memorypack(union)]
enum AbstractUnion {
    ConcreteA(ConcreteA),
    ConcreteB(ConcreteB)
}

#[derive(MemoryPackable, Debug, Clone, Copy, PartialEq, Eq)]
#[memorypack(flags)]
#[repr(transparent)]
struct Permissions(i32);

#[derive(MemoryPackable, Debug, Clone)]
struct WithEnum {
    favorite_color: Color,
    user_permissions: Permissions
}

#[derive(MemoryPackable, Debug, Clone, PartialEq)]
struct ExplicitOrder {
    #[memorypack(order = 2)]
    third: i32,
    #[memorypack(order = 0)]
    first: i32,
    #[memorypack(order = 1)]
    second: i32
}

#[derive(MemoryPackable, Debug, Clone)]
#[memorypack(version_tolerant)]
struct VersionTolerant1 {
    #[memorypack(order = 0)]
    first: i32
}

#[derive(MemoryPackable, Debug, Clone)]
#[memorypack(version_tolerant)]
struct VersionTolerant2 {
    #[memorypack(order = 0)]
    first: i32,
    #[memorypack(order = 1)]
    second: i64
}

#[derive(MemoryPackable, Debug, Clone)]
#[memorypack(version_tolerant)]
struct VersionTolerant3 {
    #[memorypack(order = 0)]
    first: i32,
    #[memorypack(order = 1)]
    second: i64,
    #[memorypack(order = 2)]
    third: i16
}

#[derive(MemoryPackable, Debug, Clone)]
struct FooClass {
    xyz: i32
}

#[derive(MemoryPackable, Debug, Clone)]
struct BarClass {
    opq: String
}

#[derive(MemoryPackable, Debug, Clone)]
struct ConcreteA {
    base_id: i32,
    a_value: String
}

#[derive(MemoryPackable, Debug, Clone)]
struct ConcreteB {
    base_id: i32,
    b_value: f64
}

#[derive(MemoryPackable, Debug, Clone, Default)]
struct Person {
    age: i32,
    name: String
}

#[derive(MemoryPackable, Debug, Clone)]
#[repr(C)]
struct Point {
    x: i32,
    y: i32
}

#[derive(MemoryPackable, Debug, Clone)]
struct PersonRecord {
    age: i32,
    name: String
}

#[derive(MemoryPackable, Debug, Clone)]
#[repr(C)]
struct PointRecord {
    x: i32,
    y: i32
}

#[derive(MemoryPackable, Debug, Clone)]
struct MixedMembers {
    public_field: i32,
    public_read_only_field: i32,
    public_property: i32,
    private_set_public_property: i32,
    read_only_public_property: i32,
    init_property: i32
}

#[derive(MemoryPackable, Debug, Clone)]
struct IncludeIgnoreSample {
    public_value: i32,
    private_value: i32
}

#[derive(MemoryPackable, Debug, Clone)]
#[repr(C)]
struct UnmanagedStruct {
    a: i32,
    b: f32,
    c: u8
}

#[derive(MemoryPackable, Debug, Clone)]
struct ParameterizedConstructor {
    x: i32,
    y: String
}

#[derive(MemoryPackable, Debug, Clone)]
struct MultipleConstructors {
    value: i32
}

#[derive(MemoryPackable, Debug, Clone)]
struct DerivedClass {
    base_value: i32,
    derived_value: String
}

#[derive(MemoryPackable, Debug, Clone)]
struct Outer {
    outer_id: i32,
    inner_object: Inner
}

#[derive(MemoryPackable, Debug, Clone)]
struct Inner {
    inner_name: String
}

#[derive(MemoryPackable, Debug, Clone)]
struct GenericContainer<T> {
    value: T
}

#[derive(MemoryPackable, Debug, Clone, Default)]
struct DotNetVersion {
    major: i32,
    minor: i32,
    build: i32,
    revision: i32
}

#[derive(MemoryPackable, Debug, Clone)]
struct CallbackSample {
    value: i32,
    call_count: i32
}

#[derive(MemoryPackable, Debug, Clone)]
struct ComplexNested {
    data: Vec<BTreeMap<String, Vec<i32>>>
}

#[derive(MemoryPackable, Debug, Clone)]
struct BitArrayValue {
    length: i32,
    values: Vec<i32>
}

#[derive(MemoryPackable, Debug, Clone)]
struct Grouping {
    key: char,
    values: Vec<String>
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
struct NullableStringField(Option<String>);

#[derive(MemoryPackable, Debug, Clone)]
struct NullableStruct {
    integer: Option<i32>,
    text: NullableStringField
}

#[derive(MemoryPackable, Debug, Clone)]
#[memorypack(circular)]
struct NodeWithCircular {
    #[memorypack(order = 0)]
    id: i32,
    #[memorypack(order = 1)]
    next: Option<MemoryPackReference<Self>>
}

#[derive(MemoryPackable, Debug, Clone)]
#[memorypack(circular)]
struct SharedCircularRoot {
    left: Option<MemoryPackReference<NodeWithCircular>>,
    right: Option<MemoryPackReference<NodeWithCircular>>
}

#[derive(MemoryPackable, Debug, Clone)]
struct RequiredMembersSample {
    required_int: i32,
    required_string: String,
    optional_int: i32
}

#[derive(MemoryPackable, Debug, Clone)]
#[repr(C)]
struct ReadOnlyPoint {
    x: i32,
    y: i32
}

#[derive(MemoryPackable, Debug, Clone)]
struct MultiDimensionalData {
    matrix_2d: MultiDimArray<i32>,
    matrix_3d: MultiDimArray<i32>
}

#[derive(MemoryPackable, Debug, Clone)]
#[memorypack(version_tolerant)]
struct VersionTolerantSkippedField {
    #[memorypack(order = 0)]
    first: i32,
    #[memorypack(order = 2)]
    third: i16
}

#[derive(MemoryPackable, Debug, Clone)]
#[memorypack(version_tolerant)]
struct VersionTolerantSparse {
    #[memorypack(order = 2)]
    third: i16,
    #[memorypack(order = 5)]
    sixth: Vec<u16>
}

#[derive(MemoryPackable, Debug, Clone)]
#[memorypack(version_tolerant)]
struct VersionTolerant0 {}

#[derive(MemoryPackable, Debug, Clone)]
#[memorypack(version_tolerant)]
struct VersionTolerantWithComplexType {
    #[memorypack(order = 0)]
    first: DotNetVersion,
    #[memorypack(order = 1)]
    second: i64,
    #[memorypack(order = 2)]
    third: i16
}

#[derive(MemoryPackable, Debug, Clone)]
#[memorypack(version_tolerant)]
struct VersionTolerantWithNullable {
    #[memorypack(order = 0)]
    integer: Option<i32>,
    #[memorypack(order = 1)]
    text: NullableStringField,
    #[memorypack(order = 2)]
    timestamp: Option<DateTime>
}

#[derive(MemoryPackable, Debug, PartialEq, Eq)]
struct CustomFormattedEndpoint {
    id: u32,
    #[memorypack(formatter = Ipv4Formatter)]
    address: Ipv4Addr,
    port: u16
}

#[derive(MemoryPackable)]
struct CustomFormattedTuple(u32, #[memorypack(formatter = self::Ipv4Formatter)] Ipv4Addr, u16);

#[derive(MemoryPackable)]
struct CustomFormattedGeneric<T> {
    id: u32,
    #[memorypack(formatter = Ipv4Formatter)]
    address: T,
    port: u16
}

struct Ipv4Formatter;

macro_rules! exact_case {
    ($test:ident, $fixture:literal, $value:expr) => {
        #[test]
        fn $test() -> TestResult {
            let value = $value;
            exact($fixture, &value)
        }
    };
}

macro_rules! semantic_case {
    ($test:ident, $fixture:literal, $value:expr) => {
        #[test]
        fn $test() -> TestResult {
            let value = $value;
            semantic($fixture, &value)
        }
    };
}

macro_rules! exact_options_case {
    ($test:ident, $fixture:literal, $value:expr, $options:expr) => {
        #[test]
        fn $test() -> TestResult {
            let value = $value;
            exact_with_options($fixture, &value, &$options)
        }
    };
}

macro_rules! round_trip_case {
    ($test:ident, $fixture:literal, $type:ty) => {
        #[test]
        fn $test() -> TestResult { round_trip::<$type>($fixture) }
    };
}

impl Permissions {
    const EXECUTE: Self = Self(4);
    const READ: Self = Self(1);
    const WRITE: Self = Self(2);
}

impl MemoryPackSerialize for NullableStringField {
    fn serialize(&self, writer: &mut MemoryPackWriter) -> Result<(), MemoryPackError> {
        self.0.serialize(writer)
    }
}

impl<'de> MemoryPackDeserialize<'de> for NullableStringField {
    fn deserialize(reader: &mut MemoryPackReader<'de>) -> Result<Self, MemoryPackError> {
        Ok(Self(Option::<String>::deserialize(reader)?))
    }
}

impl MemoryPackFormatter<Ipv4Addr> for Ipv4Formatter {
    fn serialize(value: &Ipv4Addr, writer: &mut MemoryPackWriter) -> Result<(), MemoryPackError> {
        value
            .octets()
            .into_iter()
            .try_for_each(|octet| writer.write_u8(octet))
    }

    fn deserialize(reader: &mut MemoryPackReader<'_>) -> Result<Ipv4Addr, MemoryPackError> {
        Ok(Ipv4Addr::new(
            reader.read_u8()?,
            reader.read_u8()?,
            reader.read_u8()?,
            reader.read_u8()?
        ))
    }

    fn serialized_size_hint(_value: &Ipv4Addr) -> usize { IPV4_OCTET_COUNT }
}

#[test]
fn custom_formatter() -> TestResult {
    let fixture = "167_custom_formatter.bytes";
    let endpoint = CustomFormattedEndpoint {
        id: 42,
        address: Ipv4Addr::new(192, 168, 1, 10),
        port: 8080
    };

    exact(fixture, &endpoint)?;
    let decoded = MemoryPackSerializer::deserialize::<CustomFormattedEndpoint>(&csharp(fixture)?)?;

    assert_eq!(decoded, endpoint);

    Ok(())
}

exact_case!(
    custom_formatter_tuple,
    "167_custom_formatter.bytes",
    CustomFormattedTuple(42, Ipv4Addr::new(192, 168, 1, 10), 8080)
);

exact_case!(custom_formatter_generic, "167_custom_formatter.bytes", CustomFormattedGeneric {
    id: 42,
    address: Ipv4Addr::new(192, 168, 1, 10),
    port: 8080
});

#[test]
fn circular_reference() -> TestResult {
    let mut graph = MemoryPackGraph::new(NodeWithCircular { id: 1, next: None });
    let root = graph.root;
    let second = graph.insert(NodeWithCircular { id: 2, next: None })?;
    let third = graph.insert(NodeWithCircular { id: 3, next: None })?;
    graph.get_mut(root)?.next = Some(second);
    graph.get_mut(second)?.next = Some(third);
    graph.get_mut(third)?.next = Some(root);

    exact("132_circular_reference.bytes", &graph)?;
    let decoded = MemoryPackSerializer::deserialize::<MemoryPackGraph<NodeWithCircular>>(&csharp(
        "132_circular_reference.bytes"
    )?)?;
    let second = decoded
        .get(decoded.root)?
        .next
        .ok_or("missing second node")?;
    let third = decoded
        .get(second)?
        .next
        .ok_or("missing third node")?;

    assert_eq!(
        (decoded.get(decoded.root)?.id, decoded.get(second)?.id, decoded.get(third)?.id),
        (1, 2, 3)
    );
    assert_eq!(decoded.get(third)?.next, Some(decoded.root));

    Ok(())
}

#[test]
fn self_reference() -> TestResult {
    let mut graph = MemoryPackGraph::new(NodeWithCircular { id: 1, next: None });
    let root = graph.root;
    graph.get_mut(root)?.next = Some(root);

    exact("165_self_reference.bytes", &graph)?;
    let decoded = MemoryPackSerializer::deserialize::<MemoryPackGraph<NodeWithCircular>>(&csharp(
        "165_self_reference.bytes"
    )?)?;

    assert_eq!(decoded.get(decoded.root)?.next, Some(decoded.root));

    Ok(())
}

#[test]
fn shared_reference() -> TestResult {
    let mut graph = MemoryPackGraph::new(SharedCircularRoot { left: None, right: None });
    let child = graph.insert(NodeWithCircular { id: 7, next: None })?;
    let root = graph.root;
    graph.get_mut(root)?.left = Some(child);
    graph.get_mut(root)?.right = Some(child);

    exact("166_shared_reference.bytes", &graph)?;
    let mut decoded = MemoryPackSerializer::deserialize::<MemoryPackGraph<SharedCircularRoot>>(
        &csharp("166_shared_reference.bytes")?
    )?;
    let left = decoded
        .get(decoded.root)?
        .left
        .ok_or("missing left node")?;
    let right = decoded
        .get(decoded.root)?
        .right
        .ok_or("missing right node")?;
    decoded.get_mut(left)?.id = 42;

    assert_eq!(left, right);
    assert_eq!(decoded.get(right)?.id, 42);

    Ok(())
}

#[test]
fn bitarray() -> TestResult {
    let fixture_name = "34_bitarray.bytes";
    let csharp_bytes = csharp(fixture_name)?;
    let decoded = MemoryPackSerializer::deserialize::<BitArrayValue>(&csharp_bytes)?;
    let expected_bits = [true, false, true, true, false];

    assert_eq!(decoded.length, expected_bits.len() as i32);

    let decoded_bits = decoded
        .values
        .iter()
        .flat_map(|word| (0..i32::BITS).map(move |offset| (word >> offset) & 1 != 0))
        .take(expected_bits.len())
        .collect::<Vec<_>>();

    assert_eq!(decoded_bits, expected_bits);

    let rust_bytes = MemoryPackSerializer::serialize(&decoded)?;

    assert_eq!(rust_bytes, csharp_bytes);

    write_rust(fixture_name, &rust_bytes)
}

fn csharp_dir() -> PathBuf {
    env::var_os("MEMORYPACK_DOTNET_FIXTURES").map_or_else(
        || {
            PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("..")
                .join("fixtures")
                .join("c#")
        },
        PathBuf::from
    )
}

fn rust_dir() -> PathBuf {
    env::var_os("MEMORYPACK_RUST_FIXTURES").map_or_else(
        || {
            PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("..")
                .join("fixtures")
                .join("rust")
        },
        PathBuf::from
    )
}

fn csharp(name: &str) -> TestResult<Vec<u8>> { Ok(fs::read(csharp_dir().join(name))?) }

fn write_rust(name: &str, bytes: &[u8]) -> TestResult {
    fs::create_dir_all(rust_dir())?;
    fs::write(rust_dir().join(name), bytes)?;

    Ok(())
}

fn exact<T: MemoryPackSerialize + MemoryPackDeserializeOwned>(name: &str, value: &T) -> TestResult {
    let rust = MemoryPackSerializer::serialize(value)?;
    let csharp_bytes = csharp(name)?;

    assert_eq!(rust, csharp_bytes, "{name}");
    let decoded = MemoryPackSerializer::deserialize::<T>(&csharp_bytes)?;

    assert_eq!(
        MemoryPackSerializer::serialize(&decoded)?,
        csharp_bytes,
        "{name} decoded round-trip"
    );

    write_rust(name, &rust)
}

fn exact_with_options<T: MemoryPackSerialize + MemoryPackDeserializeOwned>(
    name: &str,
    value: &T,
    options: &memorypack::MemoryPackSerializerOptions
) -> TestResult {
    let rust = MemoryPackSerializer::serialize_with_options(value, options)?;
    let csharp_bytes = csharp(name)?;

    assert_eq!(rust, csharp_bytes, "{name}");
    let decoded = MemoryPackSerializer::deserialize::<T>(&csharp_bytes)?;
    let _ = MemoryPackSerializer::serialize_with_options(&decoded, options)?;

    write_rust(name, &rust)
}

fn semantic<T: MemoryPackSerialize + MemoryPackDeserializeOwned + PartialEq + Debug>(
    name: &str,
    value: &T
) -> TestResult {
    let rust = MemoryPackSerializer::serialize(value)?;
    let decoded = MemoryPackSerializer::deserialize::<T>(&csharp(name)?)?;

    assert_eq!(&decoded, value, "{name}");
    let _ = MemoryPackSerializer::serialize(&decoded)?;

    write_rust(name, &rust)
}

fn round_trip<T: MemoryPackSerialize + MemoryPackDeserializeOwned>(name: &str) -> TestResult {
    let bytes = csharp(name)?;
    let decoded = MemoryPackSerializer::deserialize::<T>(&bytes)?;
    let rust = MemoryPackSerializer::serialize(&decoded)?;

    assert_eq!(rust, bytes, "{name}");

    write_rust(name, &rust)
}

exact_case!(byte, "01_byte.bytes", 255_u8);

exact_case!(sbyte, "02_sbyte.bytes", -128_i8);

exact_case!(short, "03_short.bytes", -32768_i16);

exact_case!(ushort, "04_ushort.bytes", 65535_u16);

exact_case!(int, "05_int.bytes", 42_i32);

exact_case!(uint, "06_uint.bytes", u32::MAX);

exact_case!(long, "07_long.bytes", i64::MIN);

exact_case!(ulong, "08_ulong.bytes", u64::MAX);

exact_case!(float, "09_float.bytes", SAMPLE_FLOAT);

exact_case!(double, "10_double.bytes", 2_718_281_828_459_045_f64 / 1_000_000_000_000_000.0);

exact_case!(bool_true, "11_bool_true.bytes", true);

exact_case!(bool_false, "12_bool_false.bytes", false);

exact_case!(char, "13_char.bytes", 'A');

exact_case!(string, "14_string.bytes", "Hello, MemoryPack!".to_owned());

exact_case!(string_empty, "15_string_empty.bytes", String::new());

exact_case!(string_null, "16_string_null.bytes", Option::<String>::None);

exact_case!(decimal, "17_decimal.bytes", rust_decimal_macros::dec!(123.456));

exact_case!(half, "18_half.bytes", half::f16::from_f32(314.0 / 100.0));

exact_case!(int128, "19_int128.bytes", ((12345_i128) << 64) | 67890);

exact_case!(uint128, "20_uint128.bytes", ((99999_u128) << 64) | 88888);

exact_case!(
    guid,
    "21_guid.bytes",
    uuid::Uuid::from_bytes([
        0x78, 0x56, 0x34, 0x12, 0x34, 0x12, 0x34, 0x12, 0x12, 0x34, 0x12, 0x34, 0x56, 0x78, 0x9A,
        0xBC
    ])
);

exact_case!(rune, "22_rune.bytes", 0x1F389_i32);

exact_case!(
    biginteger,
    "23_biginteger.bytes",
    "12345678901234567890".parse::<num_bigint::BigInt>()?
);

exact_case!(
    timespan,
    "24_timespan.bytes",
    SignedDuration::from_mins(42) + SignedDuration::from_secs(30)
);

exact_case!(datetime, "25_datetime.bytes", "2025-10-19T14:30:00Z".parse::<Timestamp>()?);

exact_case!(
    datetimeoffset,
    "26_datetimeoffset.bytes",
    Zoned::new(
        "2025-10-19T19:30:00Z".parse::<Timestamp>()?,
        TimeZone::fixed(Offset::from_hours(-5)?)
    )
);

exact_case!(timeonly, "27_timeonly.bytes", Time::new(14, 30, 0, 0)?);

exact_case!(dateonly, "28_dateonly.bytes", Date::new(2025, 10, 19)?);

exact_case!(uri, "30_uri.bytes", url::Url::parse("https://github.com/Cysharp/MemoryPack")?);

exact_case!(complex, "36_complex.bytes", num_complex::Complex::new(3.0, 4.0));

exact_case!(quaternion, "38_quaternion.bytes", glam::Quat::from_xyzw(1.0, 2.0, 3.0, 4.0));

exact_case!(
    matrix3x2,
    "39_matrix3x2.bytes",
    glam::Mat3A::from_cols(
        glam::Vec3A::new(1.0, 3.0, 5.0),
        glam::Vec3A::new(2.0, 4.0, 6.0),
        glam::Vec3A::new(0.0, 0.0, 1.0)
    )
);

exact_case!(matrix4x4, "40_matrix4x4.bytes", glam::Mat4::IDENTITY);

exact_case!(vector2, "41_vector2.bytes", glam::Vec2::new(1.5, 2.5));

exact_case!(vector3, "42_vector3.bytes", glam::Vec3::new(1.5, 2.5, 3.5));

exact_case!(vector4, "43_vector4.bytes", glam::Vec4::new(1.5, 2.5, 3.5, 4.5));

exact_case!(array_1d_int, "44_array_1d_int.bytes", vec![1, 2, 3, 4, 5]);

exact_case!(array_1d_string, "45_array_1d_string.bytes", vec![
    "apple".to_owned(),
    "banana".to_owned(),
    "cherry".to_owned()
]);

exact_case!(array_2d, "46_array_2d.bytes", MultiDimArray::new(vec![2, 3], vec![1, 2, 3, 4, 5, 6]));

exact_case!(
    array_3d,
    "47_array_3d.bytes",
    MultiDimArray::new(vec![2, 2, 2], vec![1, 2, 3, 4, 5, 6, 7, 8])
);

exact_case!(array_4d, "48_array_4d.bytes", MultiDimArray::new(vec![1, 1, 1, 1], vec![1]));

exact_case!(array_empty, "49_array_empty.bytes", Vec::<i32>::new());

exact_case!(array_null, "50_array_null.bytes", Option::<Vec<i32>>::None);

exact_case!(nullable_int_value, "54_nullable_int_value.bytes", Some(42_i32));

exact_case!(nullable_int_null, "55_nullable_int_null.bytes", None::<i32>);

exact_case!(valuetuple3, "60_valuetuple3.bytes", (1_i32, "two".to_owned(), 3.0_f64));

exact_case!(valuetuple8, "61_valuetuple8.bytes", (1_i32, 2, 3, 4, 5, 6, 7, 8));

exact_case!(list_int, "64_list_int.bytes", vec![1, 2, 3, 4, 5]);

exact_case!(list_string, "65_list_string.bytes", vec![
    "a".to_owned(),
    "b".to_owned(),
    "c".to_owned()
]);

exact_case!(list_empty, "66_list_empty.bytes", Vec::<i32>::new());

exact_case!(linkedlist, "67_linkedlist.bytes", [1, 2, 3].into_iter().collect::<LinkedList<_>>());

exact_case!(
    queue,
    "68_queue.bytes",
    ["first".to_owned(), "second".to_owned(), "third".to_owned()]
        .into_iter()
        .collect::<VecDeque<_>>()
);

exact_case!(stack, "69_stack.bytes", [1, 2, 3].into_iter().collect::<VecDeque<_>>());

exact_case!(
    sortedset,
    "71_sortedset.bytes",
    ["zebra", "apple", "mango"]
        .into_iter()
        .map(str::to_owned)
        .collect::<BTreeSet<_>>()
);

exact_case!(
    sorteddictionary,
    "76_sorteddictionary.bytes",
    [(3, "three".to_owned()), (1, "one".to_owned()), (2, "two".to_owned())]
        .into_iter()
        .collect::<BTreeMap<_, _>>()
);

exact_case!(ienumerable, "82_ienumerable.bytes", vec![1, 2, 3]);

exact_case!(icollection, "83_icollection.bytes", vec![1, 2, 3]);

exact_case!(ilist, "84_ilist.bytes", vec![1, 2, 3]);

exact_case!(ireadonlycollection, "85_ireadonlycollection.bytes", vec![1, 2, 3]);

exact_case!(ireadonlylist, "86_ireadonlylist.bytes", vec![1, 2, 3]);

exact_case!(collection, "78_collection.bytes", vec![1, 2, 3]);

exact_case!(readonlycollection, "79_readonlycollection.bytes", vec![
    "a".to_owned(),
    "b".to_owned(),
    "c".to_owned()
]);

exact_case!(observablecollection, "80_observablecollection.bytes", vec![10, 20, 30]);

exact_case!(readonlyobservablecollection, "81_readonlyobservablecollection.bytes", vec![
    "x".to_owned(),
    "y".to_owned()
]);

exact_case!(concurrentbag, "90_concurrentbag.bytes", vec![3, 2, 1]);

exact_case!(concurrentqueue, "91_concurrentqueue.bytes", vec![
    "a".to_owned(),
    "b".to_owned(),
    "c".to_owned()
]);

exact_case!(concurrentstack, "92_concurrentstack.bytes", memorypack::Stack(vec![3, 2, 1]));

exact_case!(blockingcollection, "94_blockingcollection.bytes", vec![1, 2, 3]);

exact_case!(immutablearray, "95_immutablearray.bytes", vec![1, 2, 3]);

exact_case!(immutablelist, "96_immutablelist.bytes", vec![
    "a".to_owned(),
    "b".to_owned(),
    "c".to_owned()
]);

exact_case!(
    immutablesortedset,
    "98_immutablesortedset.bytes",
    ["z", "a", "m"]
        .into_iter()
        .map(str::to_owned)
        .collect::<BTreeSet<_>>()
);

exact_case!(immutablequeue, "99_immutablequeue.bytes", vec![1, 2, 3]);

exact_case!(immutablestack, "100_immutablestack.bytes", memorypack::Stack(vec![3, 2, 1]));

exact_case!(
    immutablesorteddictionary,
    "102_immutablesorteddictionary.bytes",
    iter::once(("key".to_owned(), 42)).collect::<BTreeMap<_, _>>()
);

exact_case!(iimmutablelist, "103_iimmutablelist.bytes", vec![1, 2, 3]);

exact_case!(person_class, "108_person_class.bytes", Person {
    age: 42,
    name: "John Doe".to_owned()
});

exact_case!(point_struct, "110_point_struct.bytes", Point { x: 10, y: 20 });

exact_case!(person_record, "111_person_record.bytes", PersonRecord {
    age: 30,
    name: "Jane Smith".to_owned()
});

exact_case!(point_record_struct, "112_point_record_struct.bytes", PointRecord { x: 15, y: 25 });

exact_case!(unmanaged_struct, "116_unmanaged_struct.bytes", UnmanagedStruct {
    a: 42,
    b: 314.0 / 100.0,
    c: 255
});

exact_case!(
    parameterized_constructor,
    "117_parameterized_constructor.bytes",
    ParameterizedConstructor {
        x: 100,
        y: "test".to_owned()
    }
);

exact_case!(multiple_constructors, "118_multiple_constructors.bytes", MultipleConstructors {
    value: 42
});

exact_case!(inheritance_derived, "119_inheritance_derived.bytes", DerivedClass {
    base_value: 10,
    derived_value: "derived".to_owned()
});

exact_case!(nested_types, "120_nested_types.bytes", Outer {
    outer_id: 1,
    inner_object: Inner {
        inner_name: "nested".to_owned()
    }
});

exact_case!(generic_int, "121_generic_int.bytes", GenericContainer { value: 42_i32 });

exact_case!(generic_string, "122_generic_string.bytes", GenericContainer {
    value: "generic".to_owned()
});

exact_case!(generic_list, "123_generic_list.bytes", GenericContainer { value: vec![1_i32, 2, 3] });

exact_case!(mixed_members, "113_mixed_members.bytes", MixedMembers {
    public_field: 1,
    public_read_only_field: 100,
    public_property: 2,
    private_set_public_property: 0,
    read_only_public_property: 200,
    init_property: 3
});

exact_case!(include_ignore_sample, "114_include_ignore_sample.bytes", IncludeIgnoreSample {
    public_value: 10,
    private_value: 42
});

exact_case!(dotnet_version, "31_version.bytes", DotNetVersion {
    major: 1,
    minor: 2,
    build: 3,
    revision: 4
});

exact_case!(callbacks, "130_callbacks.bytes", CallbackSample { value: 100, call_count: 1 });

exact_case!(complex_nested, "131_complex_nested.bytes", ComplexNested {
    data: vec![
        [("first".to_owned(), vec![1, 2, 3]), ("second".to_owned(), vec![4, 5])]
            .into_iter()
            .collect(),
        iter::once(("third".to_owned(), vec![6, 7, 8, 9])).collect()
    ]
});

exact_case!(version_tolerant_0, "156_version_tolerant_0.bytes", VersionTolerant0 {});

exact_case!(
    version_tolerant_complex,
    "162_version_tolerant_complex.bytes",
    VersionTolerantWithComplexType {
        first: DotNetVersion {
            major: 1,
            minor: 2,
            build: 3,
            revision: 4
        },
        second: 999,
        third: 42
    }
);

exact_case!(required_members, "153_required_members.bytes", RequiredMembersSample {
    required_int: 42,
    required_string: "required".to_owned(),
    optional_int: 0
});

exact_case!(readonly_struct, "154_readonly_struct.bytes", ReadOnlyPoint { x: 50, y: 60 });

exact_case!(multidimensional_arrays, "155_multidimensional_arrays.bytes", MultiDimensionalData {
    matrix_2d: MultiDimArray::new(vec![2, 3], vec![1, 2, 3, 4, 5, 6]),
    matrix_3d: MultiDimArray::new(vec![2, 2, 2], vec![1, 2, 3, 4, 5, 6, 7, 8])
});

exact_case!(
    version_tolerant_skipped_field,
    "160_version_tolerant_skipped_field.bytes",
    VersionTolerantSkippedField { first: 99, third: 13 }
);

exact_case!(version_tolerant_sparse, "161_version_tolerant_sparse.bytes", VersionTolerantSparse {
    third: 5000,
    sixth: vec![1, 10, 100]
});

exact_case!(explicit_order, "115_explicit_order.bytes", ExplicitOrder {
    third: 3,
    first: 1,
    second: 2
});

exact_case!(union_foo, "124_union_foo.bytes", UnionSample::Foo(FooClass { xyz: 999 }));

exact_case!(
    union_bar,
    "125_union_bar.bytes",
    UnionSample::Bar(BarClass {
        opq: "bar value".to_owned()
    })
);

exact_case!(
    abstract_union_a,
    "126_abstract_union_a.bytes",
    AbstractUnion::ConcreteA(ConcreteA {
        base_id: 1,
        a_value: "A".to_owned()
    })
);

exact_case!(
    abstract_union_b,
    "127_abstract_union_b.bytes",
    AbstractUnion::ConcreteB(ConcreteB {
        base_id: 2,
        b_value: 314.0 / 100.0
    })
);

exact_case!(enum_simple, "150_enum_simple.bytes", Color::Green);

exact_case!(enum_flags, "151_enum_flags.bytes", Permissions::READ | Permissions::WRITE);

exact_case!(class_with_enums, "152_class_with_enums.bytes", WithEnum {
    favorite_color: Color::Blue,
    user_permissions: Permissions::READ | Permissions::EXECUTE
});

exact_case!(version_tolerant_1, "157_version_tolerant_1.bytes", VersionTolerant1 { first: 1000 });

exact_case!(version_tolerant_2, "158_version_tolerant_2.bytes", VersionTolerant2 {
    first: 3000,
    second: 9999
});

exact_case!(version_tolerant_3, "159_version_tolerant_3.bytes", VersionTolerant3 {
    first: 444,
    second: 2452,
    third: 32
});

exact_case!(double_positive_infinity, "138_double_positive_infinity.bytes", f64::INFINITY);

exact_case!(double_negative_infinity, "139_double_negative_infinity.bytes", f64::NEG_INFINITY);

exact_case!(double_max, "142_double_max.bytes", f64::MAX);

exact_case!(double_min, "143_double_min.bytes", f64::MIN);

exact_case!(double_nan, "137_double_nan.bytes", f64::from_bits(0xFFF8_0000_0000_0000));

exact_case!(float_nan, "140_float_nan.bytes", f32::from_bits(0xFFC0_0000));

exact_case!(double_epsilon, "141_double_epsilon.bytes", f64::from_bits(1));

exact_case!(large_array, "135_large_array_10k.bytes", (0..10000).collect::<Vec<i32>>());

exact_case!(list_empty_duplicate, "144_list_empty.bytes", Vec::<i32>::new());

exact_case!(
    string_utf8,
    "133_string_utf8.bytes",
    "Hello MemoryPack! こんにちは 你好 مرحبا".to_owned()
);

exact_case!(person_class_null, "109_person_class_null.bytes", Option::<Person>::None);

exact_case!(list_null, "145_list_null.bytes", Option::<Vec<i32>>::None);

exact_case!(dict_null, "147_dict_null.bytes", Option::<Vec<(String, i32)>>::None);

exact_case!(tuple3, "58_tuple3.bytes", memorypack::Tuple((1_i32, "two".to_owned(), 3.0_f64)));

exact_case!(
    tuple7,
    "59_tuple7.bytes",
    memorypack::Tuple((1_i32, 2_i32, 3_i32, 4_i32, 5_i32, 6_i32, 7_i32))
);

exact_case!(keyvaluepair, "62_keyvaluepair.bytes", ("age".to_owned(), 42_i32));

exact_case!(lazy, "63_lazy.bytes", memorypack::Lazy(42_i32));

exact_case!(priorityqueue, "72_priorityqueue.bytes", vec![
    ("high".to_owned(), 1_i32),
    ("low".to_owned(), 10_i32),
    ("medium".to_owned(), 5_i32)
]);

exact_case!(
    sortedlist,
    "75_sortedlist.bytes",
    [("a".to_owned(), 1_i32), ("b".to_owned(), 2_i32)]
        .into_iter()
        .collect::<BTreeMap<_, _>>()
);

exact_case!(custom_list, "128_custom_list.bytes", vec![10, 20, 30]);

exact_case!(memory, "51_memory.bytes", vec![1_u8, 2, 3, 4, 5]);

exact_case!(readonlymemory, "52_readonlymemory.bytes", vec![10_i32, 20, 30]);

exact_case!(arraysegment, "53_arraysegment.bytes", vec![2_u8, 3, 4]);

exact_case!(cultureinfo, "35_cultureinfo.bytes", "en-US".to_owned());

exact_case!(plane, "37_plane.bytes", glam::Vec4::new(1.0, 2.0, 3.0, 4.0));

exact_case!(ilookup, "106_ilookup.bytes", vec![
    Grouping {
        key: 'a',
        values: vec!["apple".to_owned(), "apricot".to_owned()]
    },
    Grouping {
        key: 'b',
        values: vec!["banana".to_owned(), "blueberry".to_owned()]
    }
]);

exact_case!(igrouping, "107_igrouping.bytes", Grouping {
    key: 'a',
    values: vec!["apple".to_owned(), "apricot".to_owned()]
});

exact_case!(nullable_struct_values, "148_nullable_struct_values.bytes", NullableStruct {
    integer: Some(42),
    text: NullableStringField(Some("value".to_owned()))
});

exact_case!(nullable_struct_nulls, "149_nullable_struct_nulls.bytes", NullableStruct {
    integer: None,
    text: NullableStringField(None)
});

exact_case!(
    version_tolerant_nullable_values,
    "163_version_tolerant_nullable_values.bytes",
    VersionTolerantWithNullable {
        integer: Some(42),
        text: NullableStringField(Some("test".to_owned())),
        timestamp: Some(DateTime::new(2025, 10, 19, 0, 0, 0, 0)?)
    }
);

exact_case!(
    version_tolerant_nullable_nulls,
    "164_version_tolerant_nullable_nulls.bytes",
    VersionTolerantWithNullable {
        integer: None,
        text: NullableStringField(None),
        timestamp: None
    }
);
exact_options_case!(
    string_utf16,
    "134_string_utf16.bytes",
    "Hello MemoryPack! こんにちは 你好 مرحبا".to_owned(),
    memorypack::MemoryPackSerializerOptions::UTF16
);

semantic_case!(
    hashset,
    "70_hashset.bytes",
    [3, 1, 4, 1, 5, 9, 2, 6]
        .into_iter()
        .collect::<HashSet<_>>()
);

semantic_case!(
    dictionary,
    "73_dictionary.bytes",
    [("one".to_owned(), 1), ("two".to_owned(), 2), ("three".to_owned(), 3)]
        .into_iter()
        .collect::<HashMap<_, _>>()
);

semantic_case!(
    dictionary_int_key,
    "74_dictionary_int_key.bytes",
    [(1, "one".to_owned()), (2, "two".to_owned())]
        .into_iter()
        .collect::<HashMap<_, _>>()
);

semantic_case!(iset, "87_iset.bytes", [1, 2, 3].into_iter().collect::<HashSet<_>>());

semantic_case!(
    idictionary,
    "88_idictionary.bytes",
    iter::once(("a".to_owned(), 1)).collect::<HashMap<_, _>>()
);

semantic_case!(
    ireadonlydictionary,
    "89_ireadonlydictionary.bytes",
    iter::once(("a".to_owned(), 1)).collect::<HashMap<_, _>>()
);

semantic_case!(dict_empty, "146_dict_empty.bytes", HashMap::<String, i32>::new());

semantic_case!(
    concurrentdictionary,
    "93_concurrentdictionary.bytes",
    iter::once(("key".to_owned(), 42)).collect::<HashMap<_, _>>()
);

semantic_case!(
    immutablehashset,
    "97_immutablehashset.bytes",
    [1, 2, 3].into_iter().collect::<HashSet<_>>()
);

semantic_case!(
    immutabledictionary,
    "101_immutabledictionary.bytes",
    iter::once(("key".to_owned(), 42)).collect::<HashMap<_, _>>()
);

semantic_case!(
    iimmutableset,
    "104_iimmutableset.bytes",
    [1, 2, 3].into_iter().collect::<HashSet<_>>()
);

semantic_case!(
    iimmutabledictionary,
    "105_iimmutabledictionary.bytes",
    iter::once(("k".to_owned(), 1)).collect::<HashMap<_, _>>()
);

semantic_case!(
    large_dictionary,
    "136_large_dict_1k.bytes",
    (0..1000)
        .map(|x| (format!("key{x}"), x))
        .collect::<HashMap<_, _>>()
);

semantic_case!(
    custom_dictionary,
    "129_custom_dictionary.bytes",
    [("a".to_owned(), 1_i32), ("b".to_owned(), 2_i32)]
        .into_iter()
        .collect::<HashMap<_, _>>()
);

round_trip_case!(nullable_datetime_value, "56_nullable_datetime_value.bytes", Option<Timestamp>);

round_trip_case!(nullable_datetime_null, "57_nullable_datetime_null.bytes", Option<Timestamp>);

semantic_case!(
    readonlydictionary,
    "77_readonlydictionary.bytes",
    iter::once(("x".to_owned(), 10_i32)).collect::<HashMap<_, _>>()
);
