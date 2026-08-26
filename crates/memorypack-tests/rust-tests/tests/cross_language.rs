use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet, LinkedList, VecDeque};
use std::fs;
use std::path::PathBuf;

use chrono::TimeZone;
use memorypack::MultiDimArray;
use memorypack::prelude::*;

#[allow(unused)]
#[derive(MemoryPackable, Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
enum Color {
    Red,
    Green,
    Blue
}

#[derive(MemoryPackable, Debug, Clone, Copy, PartialEq, Eq)]
#[memorypack(flags)]
#[repr(transparent)]
struct Permissions(i32);

impl Permissions {
    const EXECUTE: Self = Self(4);
    const READ: Self = Self(1);
    const WRITE: Self = Self(2);
}

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
    my_property1: i32
}

#[derive(MemoryPackable, Debug, Clone)]
#[memorypack(version_tolerant)]
struct VersionTolerant2 {
    #[memorypack(order = 0)]
    my_property1: i32,
    #[memorypack(order = 1)]
    my_property2: i64
}

#[derive(MemoryPackable, Debug, Clone)]
#[memorypack(version_tolerant)]
struct VersionTolerant3 {
    #[memorypack(order = 0)]
    my_property1: i32,
    #[memorypack(order = 1)]
    my_property2: i64,
    #[memorypack(order = 2)]
    my_property3: i16
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
#[memorypack(union)]
enum UnionSample {
    Foo(FooClass),
    Bar(BarClass)
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

#[derive(MemoryPackable, Debug, Clone)]
#[memorypack(union)]
enum AbstractUnion {
    ConcreteA(ConcreteA),
    ConcreteB(ConcreteB)
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
struct GenericContainer<T: MemoryPackSerialize + MemoryPackDeserialize> {
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
    data: Vec<HashMap<String, Vec<i32>>>
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

impl MemoryPackSerialize for NullableStringField {
    fn serialize(&self, writer: &mut MemoryPackWriter) -> Result<(), MemoryPackError> {
        self.0.serialize(writer)
    }
}

impl MemoryPackDeserialize for NullableStringField {
    fn deserialize(reader: &mut MemoryPackReader) -> Result<Self, MemoryPackError> {
        Ok(Self(Option::<String>::deserialize(reader)?))
    }
}

#[derive(MemoryPackable, Debug, Clone)]
struct NullableStruct {
    nullable_int: Option<i32>,
    nullable_string: NullableStringField
}

#[derive(MemoryPackable, Debug, Clone)]
#[memorypack(circular)]
struct NodeWithCircular {
    #[memorypack(order = 0)]
    id: i32,
    #[memorypack(order = 1)]
    next: Option<Box<NodeWithCircular>>
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
    my_property1: i32,
    #[memorypack(order = 2)]
    my_property3: i16
}

#[derive(MemoryPackable, Debug, Clone)]
#[memorypack(version_tolerant)]
struct VersionTolerantSparse {
    #[memorypack(order = 2)]
    my_property3: i16,
    #[memorypack(order = 5)]
    my_property6: Vec<u16>
}

#[derive(MemoryPackable, Debug, Clone)]
#[memorypack(version_tolerant)]
struct VersionTolerant0 {}

#[derive(MemoryPackable, Debug, Clone)]
#[memorypack(version_tolerant)]
struct VersionTolerantWithComplexType {
    #[memorypack(order = 0)]
    my_property1: DotNetVersion,
    #[memorypack(order = 1)]
    my_property2: i64,
    #[memorypack(order = 2)]
    my_property3: i16
}

#[derive(MemoryPackable, Debug, Clone)]
#[memorypack(version_tolerant)]
struct VersionTolerantWithNullable {
    #[memorypack(order = 0)]
    nullable_int: Option<i32>,
    #[memorypack(order = 1)]
    nullable_string: NullableStringField,
    #[memorypack(order = 2)]
    nullable_datetime: Option<chrono::DateTime<chrono::Utc>>
}

fn csharp_dir() -> PathBuf {
    std::env::var_os("MEMORYPACK_DOTNET_FIXTURES").map(PathBuf::from).unwrap_or_else(|| {
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("..").join("fixtures").join("c#")
    })
}

fn rust_dir() -> PathBuf {
    std::env::var_os("MEMORYPACK_RUST_FIXTURES").map(PathBuf::from).unwrap_or_else(|| {
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("..").join("fixtures").join("rust")
    })
}

fn csharp(name: &str) -> Vec<u8> { fs::read(csharp_dir().join(name)).unwrap() }

fn write_rust(name: &str, bytes: &[u8]) {
    fs::create_dir_all(rust_dir()).unwrap();
    fs::write(rust_dir().join(name), bytes).unwrap();
}

fn exact<T: MemoryPackSerialize + MemoryPackDeserialize>(name: &str, value: &T) {
    let rust = MemoryPackSerializer::serialize(value).unwrap();
    let csharp_bytes = csharp(name);
    assert_eq!(rust, csharp_bytes, "{name}");
    let decoded = MemoryPackSerializer::deserialize::<T>(&csharp_bytes).unwrap();
    let _ = MemoryPackSerializer::serialize(&decoded).unwrap();
    write_rust(name, &rust);
}

fn exact_with_options<T: MemoryPackSerialize + MemoryPackDeserialize>(
    name: &str,
    value: &T,
    options: &memorypack::MemoryPackSerializerOptions
) {
    let rust = MemoryPackSerializer::serialize_with_options(value, options).unwrap();
    let csharp_bytes = csharp(name);
    assert_eq!(rust, csharp_bytes, "{name}");
    let decoded = MemoryPackSerializer::deserialize::<T>(&csharp_bytes).unwrap();
    let _ = MemoryPackSerializer::serialize_with_options(&decoded, options).unwrap();
    write_rust(name, &rust);
}

fn semantic<T: MemoryPackSerialize + MemoryPackDeserialize + PartialEq + std::fmt::Debug>(
    name: &str,
    value: &T
) {
    let rust = MemoryPackSerializer::serialize(value).unwrap();
    let decoded = MemoryPackSerializer::deserialize::<T>(&csharp(name)).unwrap();
    assert_eq!(&decoded, value, "{name}");
    let _ = MemoryPackSerializer::serialize(&decoded).unwrap();
    write_rust(name, &rust);
}

fn round_trip<T: MemoryPackSerialize + MemoryPackDeserialize>(name: &str) {
    let bytes = csharp(name);
    let decoded = MemoryPackSerializer::deserialize::<T>(&bytes).unwrap();
    let rust = MemoryPackSerializer::serialize(&decoded).unwrap();
    assert_eq!(rust, bytes, "{name}");
    write_rust(name, &rust);
}

macro_rules! exact_case {
    ($test:ident, $fixture:literal, $value:expr) => {
        #[test]
        fn $test() {
            let value = $value;
            exact($fixture, &value);
        }
    };
}

macro_rules! semantic_case {
    ($test:ident, $fixture:literal, $value:expr) => {
        #[test]
        fn $test() {
            let value = $value;
            semantic($fixture, &value);
        }
    };
}

macro_rules! exact_options_case {
    ($test:ident, $fixture:literal, $value:expr, $options:expr) => {
        #[test]
        fn $test() {
            let value = $value;
            exact_with_options($fixture, &value, &$options);
        }
    };
}

macro_rules! round_trip_case {
    ($test:ident, $fixture:literal, $type:ty) => {
        #[test]
        fn $test() { round_trip::<$type>($fixture); }
    };
}

macro_rules! exact_cases {
    ($( $test:ident, $fixture:literal, $value:expr; )+) => {
        $(exact_case!($test, $fixture, $value);)+
    };
}

exact_cases! {
    byte, "01_byte.bytes", 255_u8;
    sbyte, "02_sbyte.bytes", -128_i8;
    short, "03_short.bytes", -32768_i16;
    ushort, "04_ushort.bytes", 65535_u16;
    int, "05_int.bytes", 42_i32;
    uint, "06_uint.bytes", u32::MAX;
    long, "07_long.bytes", i64::MIN;
    ulong, "08_ulong.bytes", u64::MAX;
    float, "09_float.bytes", 3.14159_f32;
    double, "10_double.bytes", 2.718281828459045_f64;
    bool_true, "11_bool_true.bytes", true;
    bool_false, "12_bool_false.bytes", false;
    char, "13_char.bytes", 'A';
    string, "14_string.bytes", "Hello, MemoryPack!".to_owned();
    string_empty, "15_string_empty.bytes", String::new();
    string_null, "16_string_null.bytes", Option::<String>::None;
    decimal, "17_decimal.bytes", rust_decimal_macros::dec!(123.456);
    half, "18_half.bytes", half::f16::from_f32(3.14);
    int128, "19_int128.bytes", ((12345_i128) << 64) | 67890;
    uint128, "20_uint128.bytes", ((99999_u128) << 64) | 88888;
    guid, "21_guid.bytes", uuid::Uuid::from_bytes([0x78, 0x56, 0x34, 0x12, 0x34, 0x12, 0x34, 0x12, 0x12, 0x34, 0x12, 0x34, 0x56, 0x78, 0x9a, 0xbc]);
    rune, "22_rune.bytes", 0x1F389_i32;
    biginteger, "23_biginteger.bytes", num_bigint::BigInt::parse_bytes(b"12345678901234567890", 10).unwrap();
    timespan, "24_timespan.bytes", chrono::TimeDelta::minutes(42) + chrono::TimeDelta::seconds(30);
    datetime, "25_datetime.bytes", chrono::Utc.with_ymd_and_hms(2025, 10, 19, 14, 30, 0).unwrap();
    datetimeoffset, "26_datetimeoffset.bytes", { let offset = chrono::FixedOffset::west_opt(5 * 3600).unwrap(); offset.with_ymd_and_hms(2025, 10, 19, 14, 30, 0).unwrap() };
    timeonly, "27_timeonly.bytes", chrono::NaiveTime::from_hms_opt(14, 30, 0).unwrap();
    dateonly, "28_dateonly.bytes", chrono::NaiveDate::from_ymd_opt(2025, 10, 19).unwrap();
    uri, "30_uri.bytes", url::Url::parse("https://github.com/Cysharp/MemoryPack").unwrap();
    complex, "36_complex.bytes", num_complex::Complex::new(3.0, 4.0);
    quaternion, "38_quaternion.bytes", glam::Quat::from_xyzw(1.0, 2.0, 3.0, 4.0);
    matrix3x2, "39_matrix3x2.bytes", glam::Mat3A::from_cols(glam::Vec3A::new(1.0, 3.0, 5.0), glam::Vec3A::new(2.0, 4.0, 6.0), glam::Vec3A::new(0.0, 0.0, 1.0));
    matrix4x4, "40_matrix4x4.bytes", glam::Mat4::IDENTITY;
    vector2, "41_vector2.bytes", glam::Vec2::new(1.5, 2.5);
    vector3, "42_vector3.bytes", glam::Vec3::new(1.5, 2.5, 3.5);
    vector4, "43_vector4.bytes", glam::Vec4::new(1.5, 2.5, 3.5, 4.5);
    array_1d_int, "44_array_1d_int.bytes", vec![1, 2, 3, 4, 5];
    array_1d_string, "45_array_1d_string.bytes", vec!["apple".to_owned(), "banana".to_owned(), "cherry".to_owned()];
    array_2d, "46_array_2d.bytes", MultiDimArray::new(vec![2, 3], vec![1, 2, 3, 4, 5, 6]);
    array_3d, "47_array_3d.bytes", MultiDimArray::new(vec![2, 2, 2], vec![1, 2, 3, 4, 5, 6, 7, 8]);
    array_4d, "48_array_4d.bytes", MultiDimArray::new(vec![1, 1, 1, 1], vec![1]);
    array_empty, "49_array_empty.bytes", Vec::<i32>::new();
    array_null, "50_array_null.bytes", Option::<Vec<i32>>::None;
    nullable_int_value, "54_nullable_int_value.bytes", Some(42_i32);
    nullable_int_null, "55_nullable_int_null.bytes", None::<i32>;
    valuetuple3, "60_valuetuple3.bytes", (1_i32, "two".to_owned(), 3.0_f64);
    valuetuple8, "61_valuetuple8.bytes", (1_i32, 2, 3, 4, 5, 6, 7, 8);
    list_int, "64_list_int.bytes", vec![1, 2, 3, 4, 5];
    list_string, "65_list_string.bytes", vec!["a".to_owned(), "b".to_owned(), "c".to_owned()];
    list_empty, "66_list_empty.bytes", Vec::<i32>::new();
    linkedlist, "67_linkedlist.bytes", [1, 2, 3].into_iter().collect::<LinkedList<_>>();
    queue, "68_queue.bytes", ["first".to_owned(), "second".to_owned(), "third".to_owned()].into_iter().collect::<VecDeque<_>>();
    stack, "69_stack.bytes", [1, 2, 3].into_iter().collect::<VecDeque<_>>();
    sortedset, "71_sortedset.bytes", ["zebra", "apple", "mango"].into_iter().map(str::to_owned).collect::<BTreeSet<_>>();
    sorteddictionary, "76_sorteddictionary.bytes", [(3, "three".to_owned()), (1, "one".to_owned()), (2, "two".to_owned())].into_iter().collect::<BTreeMap<_, _>>();
    ienumerable, "82_ienumerable.bytes", vec![1, 2, 3];
    icollection, "83_icollection.bytes", vec![1, 2, 3];
    ilist, "84_ilist.bytes", vec![1, 2, 3];
    ireadonlycollection, "85_ireadonlycollection.bytes", vec![1, 2, 3];
    ireadonlylist, "86_ireadonlylist.bytes", vec![1, 2, 3];
    collection, "78_collection.bytes", vec![1, 2, 3];
    readonlycollection, "79_readonlycollection.bytes", vec!["a".to_owned(), "b".to_owned(), "c".to_owned()];
    observablecollection, "80_observablecollection.bytes", vec![10, 20, 30];
    readonlyobservablecollection, "81_readonlyobservablecollection.bytes", vec!["x".to_owned(), "y".to_owned()];
    concurrentbag, "90_concurrentbag.bytes", vec![3, 2, 1];
    concurrentqueue, "91_concurrentqueue.bytes", vec!["a".to_owned(), "b".to_owned(), "c".to_owned()];
    concurrentstack, "92_concurrentstack.bytes", memorypack::Stack(vec![3, 2, 1]);
    blockingcollection, "94_blockingcollection.bytes", vec![1, 2, 3];
    immutablearray, "95_immutablearray.bytes", vec![1, 2, 3];
    immutablelist, "96_immutablelist.bytes", vec!["a".to_owned(), "b".to_owned(), "c".to_owned()];
    immutablesortedset, "98_immutablesortedset.bytes", ["z", "a", "m"].into_iter().map(str::to_owned).collect::<BTreeSet<_>>();
    immutablequeue, "99_immutablequeue.bytes", vec![1, 2, 3];
    immutablestack, "100_immutablestack.bytes", memorypack::Stack(vec![3, 2, 1]);
    immutablesorteddictionary, "102_immutablesorteddictionary.bytes", [("key".to_owned(), 42)].into_iter().collect::<BTreeMap<_, _>>();
    iimmutablelist, "103_iimmutablelist.bytes", vec![1, 2, 3];
    person_class, "108_person_class.bytes", Person { age: 42, name: "John Doe".to_owned() };
    point_struct, "110_point_struct.bytes", Point { x: 10, y: 20 };
    person_record, "111_person_record.bytes", PersonRecord { age: 30, name: "Jane Smith".to_owned() };
    point_record_struct, "112_point_record_struct.bytes", PointRecord { x: 15, y: 25 };
    unmanaged_struct, "116_unmanaged_struct.bytes", UnmanagedStruct { a: 42, b: 3.14, c: 255 };
    parameterized_constructor, "117_parameterized_constructor.bytes", ParameterizedConstructor { x: 100, y: "test".to_owned() };
    multiple_constructors, "118_multiple_constructors.bytes", MultipleConstructors { value: 42 };
    inheritance_derived, "119_inheritance_derived.bytes", DerivedClass { base_value: 10, derived_value: "derived".to_owned() };
    nested_types, "120_nested_types.bytes", Outer { outer_id: 1, inner_object: Inner { inner_name: "nested".to_owned() } };
    generic_int, "121_generic_int.bytes", GenericContainer { value: 42_i32 };
    generic_string, "122_generic_string.bytes", GenericContainer { value: "generic".to_owned() };
    generic_list, "123_generic_list.bytes", GenericContainer { value: vec![1_i32, 2, 3] };
    mixed_members, "113_mixed_members.bytes", MixedMembers { public_field: 1, public_read_only_field: 100, public_property: 2, private_set_public_property: 200, read_only_public_property: 0, init_property: 3 };
    include_ignore_sample, "114_include_ignore_sample.bytes", IncludeIgnoreSample { public_value: 10, private_value: 42 };
    dotnet_version, "31_version.bytes", DotNetVersion { major: 1, minor: 2, build: 3, revision: 4 };
    callbacks, "130_callbacks.bytes", CallbackSample { value: 100, call_count: 1 };
    complex_nested, "131_complex_nested.bytes", ComplexNested { data: vec![[("first".to_owned(), vec![1, 2, 3]), ("second".to_owned(), vec![4, 5])].into_iter().collect(), [("third".to_owned(), vec![6, 7, 8, 9])].into_iter().collect()] };
    version_tolerant_0, "156_version_tolerant_0.bytes", VersionTolerant0 {};
    version_tolerant_complex, "162_version_tolerant_complex.bytes", VersionTolerantWithComplexType { my_property1: DotNetVersion { major: 1, minor: 2, build: 3, revision: 4 }, my_property2: 999, my_property3: 42 };
    circular_reference, "132_circular_reference.bytes", NodeWithCircular { id: 1, next: Some(Box::new(NodeWithCircular { id: 2, next: Some(Box::new(NodeWithCircular { id: 3, next: None })) })) };
    required_members, "153_required_members.bytes", RequiredMembersSample { required_int: 42, required_string: "required".to_owned(), optional_int: 0 };
    readonly_struct, "154_readonly_struct.bytes", ReadOnlyPoint { x: 50, y: 60 };
    multidimensional_arrays, "155_multidimensional_arrays.bytes", MultiDimensionalData { matrix_2d: MultiDimArray::new(vec![2, 3], vec![1, 2, 3, 4, 5, 6]), matrix_3d: MultiDimArray::new(vec![2, 2, 2], vec![1, 2, 3, 4, 5, 6, 7, 8]) };
    version_tolerant_skipped_field, "160_version_tolerant_skipped_field.bytes", VersionTolerantSkippedField { my_property1: 99, my_property3: 13 };
    version_tolerant_sparse, "161_version_tolerant_sparse.bytes", VersionTolerantSparse { my_property3: 5000, my_property6: vec![1, 10, 100] };
    explicit_order, "115_explicit_order.bytes", ExplicitOrder { third: 3, first: 1, second: 2 };
    union_foo, "124_union_foo.bytes", UnionSample::Foo(FooClass { xyz: 999 });
    union_bar, "125_union_bar.bytes", UnionSample::Bar(BarClass { opq: "bar value".to_owned() });
    abstract_union_a, "126_abstract_union_a.bytes", AbstractUnion::ConcreteA(ConcreteA { base_id: 1, a_value: "A".to_owned() });
    abstract_union_b, "127_abstract_union_b.bytes", AbstractUnion::ConcreteB(ConcreteB { base_id: 2, b_value: 3.14 });
    enum_simple, "150_enum_simple.bytes", Color::Green;
    enum_flags, "151_enum_flags.bytes", Permissions::READ | Permissions::WRITE;
    class_with_enums, "152_class_with_enums.bytes", WithEnum { favorite_color: Color::Blue, user_permissions: Permissions::READ | Permissions::EXECUTE };
    version_tolerant_1, "157_version_tolerant_1.bytes", VersionTolerant1 { my_property1: 1000 };
    version_tolerant_2, "158_version_tolerant_2.bytes", VersionTolerant2 { my_property1: 3000, my_property2: 9999 };
    version_tolerant_3, "159_version_tolerant_3.bytes", VersionTolerant3 { my_property1: 444, my_property2: 2452, my_property3: 32 };
    double_positive_infinity, "138_double_positive_infinity.bytes", f64::INFINITY;
    double_negative_infinity, "139_double_negative_infinity.bytes", f64::NEG_INFINITY;
    double_max, "142_double_max.bytes", f64::MAX;
    double_min, "143_double_min.bytes", f64::MIN;
    double_nan, "137_double_nan.bytes", f64::from_bits(0xfff8_0000_0000_0000);
    float_nan, "140_float_nan.bytes", f32::from_bits(0xffc0_0000);
    double_epsilon, "141_double_epsilon.bytes", f64::from_bits(1);
    large_array, "135_large_array_10k.bytes", (0..10000).collect::<Vec<i32>>();
    list_empty_duplicate, "144_list_empty.bytes", Vec::<i32>::new();
    string_utf8, "133_string_utf8.bytes", "Hello MemoryPack! こんにちは 你好 مرحبا".to_owned();
    person_class_null, "109_person_class_null.bytes", Option::<Person>::None;
    list_null, "145_list_null.bytes", Option::<Vec<i32>>::None;
    dict_null, "147_dict_null.bytes", Option::<Vec<(String, i32)>>::None;
    tuple3, "58_tuple3.bytes", memorypack::Tuple((1_i32, "two".to_owned(), 3.0_f64));
    tuple7, "59_tuple7.bytes", memorypack::Tuple((1_i32, 2_i32, 3_i32, 4_i32, 5_i32, 6_i32, 7_i32));
    keyvaluepair, "62_keyvaluepair.bytes", ("age".to_owned(), 42_i32);
    lazy, "63_lazy.bytes", memorypack::Lazy(42_i32);
    priorityqueue, "72_priorityqueue.bytes", vec![("high".to_owned(), 1_i32), ("low".to_owned(), 10_i32), ("medium".to_owned(), 5_i32)];
    sortedlist, "75_sortedlist.bytes", [("a".to_owned(), 1_i32), ("b".to_owned(), 2_i32)].into_iter().collect::<BTreeMap<_, _>>();
    custom_list, "128_custom_list.bytes", vec![10, 20, 30];
    memory, "51_memory.bytes", vec![1_u8, 2, 3, 4, 5];
    readonlymemory, "52_readonlymemory.bytes", vec![10_i32, 20, 30];
    arraysegment, "53_arraysegment.bytes", vec![2_u8, 3, 4];
    bitarray, "34_bitarray.bytes", BitArrayValue { length: 5, values: vec![13] };
    cultureinfo, "35_cultureinfo.bytes", "en-US".to_owned();
    plane, "37_plane.bytes", glam::Vec4::new(1.0, 2.0, 3.0, 4.0);
    ilookup, "106_ilookup.bytes", vec![Grouping { key: 'a', values: vec!["apple".to_owned(), "apricot".to_owned()] }, Grouping { key: 'b', values: vec!["banana".to_owned(), "blueberry".to_owned()] }];
    igrouping, "107_igrouping.bytes", Grouping { key: 'a', values: vec!["apple".to_owned(), "apricot".to_owned()] };
    nullable_struct_values, "148_nullable_struct_values.bytes", NullableStruct { nullable_int: Some(42), nullable_string: NullableStringField(Some("value".to_owned())) };
    nullable_struct_nulls, "149_nullable_struct_nulls.bytes", NullableStruct { nullable_int: None, nullable_string: NullableStringField(None) };
    version_tolerant_nullable_values, "163_version_tolerant_nullable_values.bytes", VersionTolerantWithNullable { nullable_int: Some(42), nullable_string: NullableStringField(Some("test".to_owned())), nullable_datetime: Some(chrono::Utc.with_ymd_and_hms(2025, 10, 19, 0, 0, 0).unwrap()) };
    version_tolerant_nullable_nulls, "164_version_tolerant_nullable_nulls.bytes", VersionTolerantWithNullable { nullable_int: None, nullable_string: NullableStringField(None), nullable_datetime: None };
}

exact_options_case!(
    string_utf16,
    "134_string_utf16.bytes",
    "Hello MemoryPack! こんにちは 你好 مرحبا".to_owned(),
    memorypack::MemoryPackSerializerOptions::UTF16
);

semantic_case!(
    hashset,
    "70_hashset.bytes",
    [3, 1, 4, 1, 5, 9, 2, 6].into_iter().collect::<HashSet<_>>()
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
    [(1, "one".to_owned()), (2, "two".to_owned())].into_iter().collect::<HashMap<_, _>>()
);
semantic_case!(iset, "87_iset.bytes", [1, 2, 3].into_iter().collect::<HashSet<_>>());
semantic_case!(
    idictionary,
    "88_idictionary.bytes",
    [("a".to_owned(), 1)].into_iter().collect::<HashMap<_, _>>()
);
semantic_case!(
    ireadonlydictionary,
    "89_ireadonlydictionary.bytes",
    [("a".to_owned(), 1)].into_iter().collect::<HashMap<_, _>>()
);
semantic_case!(dict_empty, "146_dict_empty.bytes", HashMap::<String, i32>::new());
semantic_case!(
    concurrentdictionary,
    "93_concurrentdictionary.bytes",
    [("key".to_owned(), 42)].into_iter().collect::<HashMap<_, _>>()
);
semantic_case!(
    immutablehashset,
    "97_immutablehashset.bytes",
    [1, 2, 3].into_iter().collect::<HashSet<_>>()
);
semantic_case!(
    immutabledictionary,
    "101_immutabledictionary.bytes",
    [("key".to_owned(), 42)].into_iter().collect::<HashMap<_, _>>()
);
semantic_case!(
    iimmutableset,
    "104_iimmutableset.bytes",
    [1, 2, 3].into_iter().collect::<HashSet<_>>()
);
semantic_case!(
    iimmutabledictionary,
    "105_iimmutabledictionary.bytes",
    [("k".to_owned(), 1)].into_iter().collect::<HashMap<_, _>>()
);
semantic_case!(
    large_dictionary,
    "136_large_dict_1k.bytes",
    (0..1000).map(|x| (format!("key{x}"), x)).collect::<HashMap<_, _>>()
);
semantic_case!(
    custom_dictionary,
    "129_custom_dictionary.bytes",
    [("a".to_owned(), 1_i32), ("b".to_owned(), 2_i32)].into_iter().collect::<HashMap<_, _>>()
);
round_trip_case!(
    nullable_datetime_value,
    "56_nullable_datetime_value.bytes",
    Option<chrono::DateTime<chrono::Utc>>
);
round_trip_case!(
    nullable_datetime_null,
    "57_nullable_datetime_null.bytes",
    Option<chrono::DateTime<chrono::Utc>>
);
