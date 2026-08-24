use std::fs;

use memorypack::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(MemoryPackable)]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
enum Color {
    Red,
    Green,
    Blue
}

#[derive(MemoryPackable)]
#[memorypack(flags)]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
struct Permissions(i32);

impl Permissions {
    const EXECUTE: Self = Self(4);
    const READ: Self = Self(1);
    const WRITE: Self = Self(2);
}

#[derive(MemoryPackable, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct WithEnum {
    favorite_color: Color,
    user_permissions: Permissions
}

#[derive(MemoryPackable, Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
struct ExplicitOrder {
    #[memorypack(order = 2)]
    third: i32,
    #[memorypack(order = 0)]
    first: i32,
    #[memorypack(order = 1)]
    second: i32
}

#[derive(MemoryPackable, Debug, Clone, Serialize, Deserialize)]
#[memorypack(version_tolerant)]
#[serde(rename_all = "PascalCase")]
struct VersionTolerant1 {
    #[memorypack(order = 0)]
    my_property1: i32
}

#[derive(MemoryPackable, Debug, Clone, Serialize, Deserialize)]
#[memorypack(version_tolerant)]
#[serde(rename_all = "PascalCase")]
struct VersionTolerant2 {
    #[memorypack(order = 0)]
    my_property1: i32,
    #[memorypack(order = 1)]
    my_property2: i64
}

#[derive(MemoryPackable, Debug, Clone, Serialize, Deserialize)]
#[memorypack(version_tolerant)]
#[serde(rename_all = "PascalCase")]
struct VersionTolerant3 {
    #[memorypack(order = 0)]
    my_property1: i32,
    #[memorypack(order = 1)]
    my_property2: i64,
    #[memorypack(order = 2)]
    my_property3: i16
}

#[derive(MemoryPackable, Debug, Clone, Serialize, Deserialize)]
struct FooClass {
    xyz: i32
}

#[derive(MemoryPackable, Debug, Clone, Serialize, Deserialize)]
struct BarClass {
    opq: String
}

#[derive(MemoryPackable)]
#[memorypack(union)]
#[derive(Debug, Clone, Serialize, Deserialize)]
enum UnionSample {
    Foo(FooClass),
    Bar(BarClass)
}

#[derive(MemoryPackable, Debug, Clone, Serialize, Deserialize)]
struct ConcreteA {
    base_id: i32,
    a_value: String
}

#[derive(MemoryPackable, Debug, Clone, Serialize, Deserialize)]
struct ConcreteB {
    base_id: i32,
    b_value: f64
}

#[derive(MemoryPackable)]
#[memorypack(union)]
#[derive(Debug, Clone, Serialize, Deserialize)]
enum AbstractUnion {
    ConcreteA(ConcreteA),
    ConcreteB(ConcreteB)
}

#[derive(MemoryPackable, Debug, Clone, Serialize, Deserialize)]
#[memorypack(circular)]
#[serde(rename_all = "PascalCase")]
struct NodeWithCircular {
    #[memorypack(order = 0)]
    id: i32,
    #[memorypack(order = 1)]
    next: Option<Box<NodeWithCircular>>
}

fn test_case<T: MemoryPackSerialize + MemoryPackDeserialize + Serialize>(
    name: &str,
    rust_value: &T,
    csharp_file: &str,
    rust_output: &str
) -> eyre::Result<()> {
    let rust_bytes = MemoryPackSerializer::serialize(rust_value)?;
    let csharp_bytes = fs::read(format!("memorypack/examples/bytes/{}", csharp_file))?;

    fs::create_dir_all("memorypack/examples/rust_bytes")?;
    fs::write(format!("memorypack/examples/rust_bytes/{}", rust_output), &rust_bytes)?;

    let match_str = if rust_bytes == csharp_bytes { "MATCH" } else { "MISMATCH" };

    println!("{}", name);
    println!("C#:   {:02X?}", csharp_bytes);
    println!("Rust: {:02X?}", rust_bytes);
    println!("{}\n", match_str);

    Ok(())
}

fn deserialize_and_save<T: MemoryPackDeserialize + Serialize>(
    name: &str,
    csharp_file: &str
) -> eyre::Result<()> {
    let csharp_bytes = fs::read(format!("memorypack/examples/bytes/{}", csharp_file))?;
    let value: T = MemoryPackSerializer::deserialize(&csharp_bytes)?;
    let json = serde_json::to_string_pretty(&value)?;

    fs::create_dir_all("memorypack/examples/output")?;
    let output_file = format!("memorypack/examples/output/{}.json", name);
    fs::write(&output_file, json)?;

    Ok(())
}

fn main() -> eyre::Result<()> {
    println!("MEMORYPACK TEST SUITE\n");

    println!("BYTE COMPARISON (Rust vs C#)\n");

    // ============================================================================
    // 1. PRIMITIVES AND BASIC TYPES (01-23)
    // ============================================================================

    test_case("byte", &255u8, "01_byte.bytes", "01_byte.bytes")?;
    test_case("sbyte", &-128i8, "02_sbyte.bytes", "02_sbyte.bytes")?;
    test_case("short", &-32768i16, "03_short.bytes", "03_short.bytes")?;
    test_case("ushort", &65535u16, "04_ushort.bytes", "04_ushort.bytes")?;
    test_case("int", &42i32, "05_int.bytes", "05_int.bytes")?;
    test_case("uint", &4294967295u32, "06_uint.bytes", "06_uint.bytes")?;
    test_case("long", &-9223372036854775808i64, "07_long.bytes", "07_long.bytes")?;
    test_case("ulong", &18446744073709551615u64, "08_ulong.bytes", "08_ulong.bytes")?;
    test_case("float", &3.14159f32, "09_float.bytes", "09_float.bytes")?;
    test_case("double", &2.718281828459045f64, "10_double.bytes", "10_double.bytes")?;

    // Special float values
    // Note: NaN bit patterns differ between C# and Rust (both valid, different sign
    // bit) Note: Epsilon differs - C# is smallest positive, Rust is ULP at 1.0
    test_case(
        "double_positive_infinity",
        &f64::INFINITY,
        "138_double_positive_infinity.bytes",
        "138_double_positive_infinity.bytes"
    )?;
    test_case(
        "double_negative_infinity",
        &f64::NEG_INFINITY,
        "139_double_negative_infinity.bytes",
        "139_double_negative_infinity.bytes"
    )?;
    test_case("double_max", &f64::MAX, "142_double_max.bytes", "142_double_max.bytes")?;
    test_case("double_min", &f64::MIN, "143_double_min.bytes", "143_double_min.bytes")?;
    test_case("bool_true", &true, "11_bool_true.bytes", "11_bool_true.bytes")?;
    test_case("bool_false", &false, "12_bool_false.bytes", "12_bool_false.bytes")?;
    test_case("char", &'A', "13_char.bytes", "13_char.bytes")?;
    test_case("string", &"Hello, MemoryPack!".to_string(), "14_string.bytes", "14_string.bytes")?;
    test_case("string_empty", &"".to_string(), "15_string_empty.bytes", "15_string_empty.bytes")?;

    #[cfg(feature = "nightly")]
    test_case("string_null", &None::<String>, "16_string_null.bytes", "16_string_null.bytes")?;
    #[cfg(not(feature = "nightly"))]
    test_case(
        "string_null",
        &memorypack::NullableString(None),
        "16_string_null.bytes",
        "16_string_null.bytes"
    )?;

    // ============================================================================
    // EXTENDED TYPES (17-18, 21, 23)
    // ============================================================================

    #[cfg(feature = "rust_decimal")]
    {
        use rust_decimal_macros::dec;
        test_case("decimal", &dec!(123.456), "17_decimal.bytes", "17_decimal.bytes")?;
    }

    #[cfg(feature = "half")]
    {
        use half::f16;
        test_case("half", &f16::from_f32(3.14), "18_half.bytes", "18_half.bytes")?;
    }

    #[cfg(feature = "uuid")]
    {
        use uuid::Uuid;
        let guid = Uuid::from_bytes([
            0x78, 0x56, 0x34, 0x12, 0x34, 0x12, 0x34, 0x12, 0x12, 0x34, 0x12, 0x34, 0x56, 0x78,
            0x9A, 0xBC
        ]);
        test_case("guid", &guid, "21_guid.bytes", "21_guid.bytes")?;
    }

    // C# Rune (0x1F389 = 🎉) - serialized as i32
    test_case("rune", &0x1F389i32, "22_rune.bytes", "22_rune.bytes")?;

    #[cfg(feature = "num-bigint")]
    {
        use num_bigint::BigInt;
        let bigint = BigInt::parse_bytes(b"12345678901234567890", 10).unwrap();
        test_case("biginteger", &bigint, "23_biginteger.bytes", "23_biginteger.bytes")?;
    }

    #[cfg(feature = "url")]
    {
        use url::Url;
        let uri = Url::parse("https://github.com/Cysharp/MemoryPack").unwrap();
        test_case("uri", &uri, "30_uri.bytes", "30_uri.bytes")?;
    }

    // ============================================================================
    // DATE/TIME TYPES (24-28)
    // ============================================================================

    #[cfg(feature = "chrono")]
    {
        use chrono::{FixedOffset, NaiveDate, NaiveTime, TimeDelta, TimeZone, Utc};

        test_case(
            "timespan",
            &(TimeDelta::minutes(42) + TimeDelta::seconds(30)),
            "24_timespan.bytes",
            "24_timespan.bytes"
        )?;

        let datetime = Utc.with_ymd_and_hms(2025, 10, 19, 14, 30, 0).unwrap();
        test_case("datetime", &datetime, "25_datetime.bytes", "25_datetime.bytes")?;

        let offset = FixedOffset::west_opt(5 * 3600).unwrap();
        let datetime_offset = offset.with_ymd_and_hms(2025, 10, 19, 14, 30, 0).unwrap();
        test_case(
            "datetimeoffset",
            &datetime_offset,
            "26_datetimeoffset.bytes",
            "26_datetimeoffset.bytes"
        )?;

        let time = NaiveTime::from_hms_opt(14, 30, 0).unwrap();
        test_case("timeonly", &time, "27_timeonly.bytes", "27_timeonly.bytes")?;

        let date = NaiveDate::from_ymd_opt(2025, 10, 19).unwrap();
        test_case("dateonly", &date, "28_dateonly.bytes", "28_dateonly.bytes")?;
    }

    // ============================================================================
    // MATH TYPES (36-43)
    // ============================================================================

    #[cfg(feature = "num-complex")]
    {
        use num_complex::Complex;
        test_case("complex", &Complex::new(3.0, 4.0), "36_complex.bytes", "36_complex.bytes")?;
    }

    #[cfg(feature = "glam")]
    {
        use glam::{Mat3A, Mat4, Quat, Vec2, Vec3, Vec4};

        test_case("vector2", &Vec2::new(1.5, 2.5), "41_vector2.bytes", "41_vector2.bytes")?;
        test_case("vector3", &Vec3::new(1.5, 2.5, 3.5), "42_vector3.bytes", "42_vector3.bytes")?;
        test_case(
            "vector4",
            &Vec4::new(1.5, 2.5, 3.5, 4.5),
            "43_vector4.bytes",
            "43_vector4.bytes"
        )?;
        test_case(
            "quaternion",
            &Quat::from_xyzw(1.0, 2.0, 3.0, 4.0),
            "38_quaternion.bytes",
            "38_quaternion.bytes"
        )?;

        // C# Matrix3x2 stores 6 floats: m11, m12, m21, m22, m31, m32 (row-major order)
        // glam is column-major: col0=(m11,m21,m31), col1=(m12,m22,m32)
        let mat3x2 = Mat3A::from_cols(
            glam::Vec3A::new(1.0, 3.0, 5.0), // col0: m11, m21, m31
            glam::Vec3A::new(2.0, 4.0, 6.0), // col1: m12, m22, m32
            glam::Vec3A::new(0.0, 0.0, 1.0)  // col2: unused for 3x2
        );
        test_case("matrix3x2", &mat3x2, "39_matrix3x2.bytes", "39_matrix3x2.bytes")?;

        test_case("matrix4x4", &Mat4::IDENTITY, "40_matrix4x4.bytes", "40_matrix4x4.bytes")?;
    }

    // ============================================================================
    // INT128/UINT128 (19-20)
    // ============================================================================

    // C#: new Int128(12345, 67890) = (12345 << 64) | 67890
    let int128_val: i128 = ((12345i128) << 64) | 67890i128;
    test_case("int128", &int128_val, "19_int128.bytes", "19_int128.bytes")?;

    // C#: new UInt128(99999, 88888) = (99999 << 64) | 88888
    let uint128_val: u128 = ((99999u128) << 64) | 88888u128;
    test_case("uint128", &uint128_val, "20_uint128.bytes", "20_uint128.bytes")?;

    // ============================================================================
    // NOT IMPLEMENTED - Commented out types
    // ============================================================================
    // 29_timezoneinfo - TimeZoneInfo not implemented
    // 31_version - Version not implemented
    // 32_stringbuilder - StringBuilder not implemented
    // 33_type - Type not implemented
    // 34_bitarray - BitArray not implemented
    // 35_cultureinfo - CultureInfo not implemented
    // 37_plane - Plane not implemented (no glam equivalent)
    // 51-53 - Memory<T>, ReadOnlyMemory<T>, ArraySegment<T> not implemented
    // 58-59 - Reference Tuple (different format from ValueTuple)
    // 62_keyvaluepair - KeyValuePair not implemented
    // 63_lazy - Lazy<T> not implemented
    // 72_priorityqueue - PriorityQueue not implemented
    // 75_sortedlist - SortedList not implemented
    // 77_readonlydictionary - ReadOnlyDictionary not implemented
    // 78-81 - Collection/ObservableCollection not implemented
    // 90-105 - Concurrent/Immutable collections not implemented
    // 106-107 - ILookup/IGrouping not implemented
    // 108-123 - Custom C# types (Person, Point, etc.) - need Rust equivalents
    // 128-131 - Custom collections not implemented
    // 133-143 - Special values (NaN, Infinity, etc.) - can test but skipping
    // 144-145, 147-149 - Edge cases
    // 153-155 - Required members, readonly struct
    // 156, 160-164 - More version tolerant variants

    // ============================================================================
    // 46. ENUM TYPES (150-152)
    // ============================================================================

    test_case("enum_simple", &Color::Green, "150_enum_simple.bytes", "150_enum_simple.bytes")?;
    test_case(
        "enum_flags",
        &(Permissions::READ | Permissions::WRITE),
        "151_enum_flags.bytes",
        "151_enum_flags.bytes"
    )?;
    test_case(
        "class_with_enums",
        &WithEnum {
            favorite_color: Color::Blue,
            user_permissions: Permissions::READ | Permissions::EXECUTE
        },
        "152_class_with_enums.bytes",
        "152_class_with_enums.bytes"
    )?;

    // ============================================================================
    // 5. ARRAYS (44-50)
    // ============================================================================

    test_case(
        "array_1d_int",
        &vec![1, 2, 3, 4, 5],
        "44_array_1d_int.bytes",
        "44_array_1d_int.bytes"
    )?;
    test_case(
        "array_1d_string",
        &vec!["apple".to_string(), "banana".to_string(), "cherry".to_string()],
        "45_array_1d_string.bytes",
        "45_array_1d_string.bytes"
    )?;

    // Multi-dimensional arrays
    test_case(
        "array_2d",
        &memorypack::MultiDimArray::new(vec![2, 3], vec![1, 2, 3, 4, 5, 6]),
        "46_array_2d.bytes",
        "46_array_2d.bytes"
    )?;
    test_case(
        "array_3d",
        &memorypack::MultiDimArray::new(vec![2, 2, 2], vec![1, 2, 3, 4, 5, 6, 7, 8]),
        "47_array_3d.bytes",
        "47_array_3d.bytes"
    )?;
    test_case(
        "array_4d",
        &memorypack::MultiDimArray::new(vec![1, 1, 1, 1], vec![1]),
        "48_array_4d.bytes",
        "48_array_4d.bytes"
    )?;

    test_case("array_empty", &Vec::<i32>::new(), "49_array_empty.bytes", "49_array_empty.bytes")?;

    #[cfg(feature = "nightly")]
    test_case("array_null", &None::<Vec<i32>>, "50_array_null.bytes", "50_array_null.bytes")?;
    #[cfg(not(feature = "nightly"))]
    test_case(
        "array_null",
        &memorypack::NullableVec(None::<Vec<i32>>),
        "50_array_null.bytes",
        "50_array_null.bytes"
    )?;

    // ============================================================================
    // 7. NULLABLE TYPES (54-57)
    // ============================================================================

    test_case(
        "nullable_int_value",
        &Some(42i32),
        "54_nullable_int_value.bytes",
        "54_nullable_int_value.bytes"
    )?;
    test_case(
        "nullable_int_null",
        &None::<i32>,
        "55_nullable_int_null.bytes",
        "55_nullable_int_null.bytes"
    )?;

    // ============================================================================
    // 8. TUPLES (58-61)
    // ============================================================================

    test_case(
        "valuetuple3",
        &(1i32, "two".to_string(), 3.0f64),
        "60_valuetuple3.bytes",
        "60_valuetuple3.bytes"
    )?;
    test_case(
        "valuetuple8",
        &(1i32, 2i32, 3i32, 4i32, 5i32, 6i32, 7i32, 8i32),
        "61_valuetuple8.bytes",
        "61_valuetuple8.bytes"
    )?;

    // ============================================================================
    // 11. GENERIC COLLECTIONS - List (64-66)
    // ============================================================================

    test_case("list_int", &vec![1, 2, 3, 4, 5], "64_list_int.bytes", "64_list_int.bytes")?;
    test_case(
        "list_string",
        &vec!["a".to_string(), "b".to_string(), "c".to_string()],
        "65_list_string.bytes",
        "65_list_string.bytes"
    )?;
    test_case("list_empty", &Vec::<i32>::new(), "66_list_empty.bytes", "66_list_empty.bytes")?;

    // ============================================================================
    // 11b. MORE COLLECTIONS - LinkedList, Queue, Stack, HashSet, SortedSet (67-71)
    // ============================================================================

    use std::collections::{BTreeMap, BTreeSet, HashSet, LinkedList, VecDeque};

    let mut linked_list = LinkedList::new();
    linked_list.push_back(1);
    linked_list.push_back(2);
    linked_list.push_back(3);
    test_case("linkedlist", &linked_list, "67_linkedlist.bytes", "67_linkedlist.bytes")?;

    let mut queue = VecDeque::new();
    queue.push_back("first".to_string());
    queue.push_back("second".to_string());
    queue.push_back("third".to_string());
    test_case("queue", &queue, "68_queue.bytes", "68_queue.bytes")?;

    let mut stack = VecDeque::new();
    stack.push_back(1);
    stack.push_back(2);
    stack.push_back(3);
    test_case("stack", &stack, "69_stack.bytes", "69_stack.bytes")?;

    let hashset: HashSet<i32> = vec![3, 1, 4, 1, 5, 9, 2, 6].into_iter().collect();
    test_case("hashset", &hashset, "70_hashset.bytes", "70_hashset.bytes")?;

    let sortedset: BTreeSet<String> =
        vec!["zebra".to_string(), "apple".to_string(), "mango".to_string()].into_iter().collect();
    test_case("sortedset", &sortedset, "71_sortedset.bytes", "71_sortedset.bytes")?;

    // ============================================================================
    // 11c. INTERFACE COLLECTIONS (82-89)
    // These C# interfaces serialize identically to their concrete types
    // ============================================================================

    test_case("ienumerable", &vec![1, 2, 3], "82_ienumerable.bytes", "82_ienumerable.bytes")?;
    test_case("icollection", &vec![1, 2, 3], "83_icollection.bytes", "83_icollection.bytes")?;
    test_case("ilist", &vec![1, 2, 3], "84_ilist.bytes", "84_ilist.bytes")?;
    test_case(
        "ireadonlycollection",
        &vec![1, 2, 3],
        "85_ireadonlycollection.bytes",
        "85_ireadonlycollection.bytes"
    )?;
    test_case("ireadonlylist", &vec![1, 2, 3], "86_ireadonlylist.bytes", "86_ireadonlylist.bytes")?;

    let iset: HashSet<i32> = vec![1, 2, 3].into_iter().collect();
    test_case("iset", &iset, "87_iset.bytes", "87_iset.bytes")?;

    let mut idict = hashbrown::HashMap::new();
    idict.insert("a".to_string(), 1);
    test_case("idictionary", &idict, "88_idictionary.bytes", "88_idictionary.bytes")?;
    test_case(
        "ireadonlydictionary",
        &idict,
        "89_ireadonlydictionary.bytes",
        "89_ireadonlydictionary.bytes"
    )?;

    // ============================================================================
    // 12. DICTIONARY (73-74, 76, 146-147)
    // ============================================================================

    let mut dict_string_int = hashbrown::HashMap::new();
    dict_string_int.insert("one".to_string(), 1);
    dict_string_int.insert("two".to_string(), 2);
    dict_string_int.insert("three".to_string(), 3);
    test_case("dictionary", &dict_string_int, "73_dictionary.bytes", "73_dictionary.bytes")?;

    let mut dict_int_string = hashbrown::HashMap::new();
    dict_int_string.insert(1, "one".to_string());
    dict_int_string.insert(2, "two".to_string());
    test_case(
        "dictionary_int_key",
        &dict_int_string,
        "74_dictionary_int_key.bytes",
        "74_dictionary_int_key.bytes"
    )?;

    test_case(
        "dict_empty",
        &hashbrown::HashMap::<String, i32>::new(),
        "146_dict_empty.bytes",
        "146_dict_empty.bytes"
    )?;

    let mut sorted_dict = BTreeMap::new();
    sorted_dict.insert(3, "three".to_string());
    sorted_dict.insert(1, "one".to_string());
    sorted_dict.insert(2, "two".to_string());
    test_case(
        "sorteddictionary",
        &sorted_dict,
        "76_sorteddictionary.bytes",
        "76_sorteddictionary.bytes"
    )?;

    // ============================================================================
    // 27. EXPLICIT LAYOUT ORDER (115)
    // ============================================================================

    test_case(
        "explicit_order",
        &ExplicitOrder {
            third: 3,
            first: 1,
            second: 2
        },
        "115_explicit_order.bytes",
        "115_explicit_order.bytes"
    )?;

    // ============================================================================
    // 51. VERSION TOLERANT OBJECTS (157-159)
    // ============================================================================

    test_case(
        "version_tolerant_1",
        &VersionTolerant1 { my_property1: 1000 },
        "157_version_tolerant_1.bytes",
        "157_version_tolerant_1.bytes"
    )?;
    test_case(
        "version_tolerant_2",
        &VersionTolerant2 {
            my_property1: 3000,
            my_property2: 9999
        },
        "158_version_tolerant_2.bytes",
        "158_version_tolerant_2.bytes"
    )?;
    test_case(
        "version_tolerant_3",
        &VersionTolerant3 {
            my_property1: 444,
            my_property2: 2452,
            my_property3: 32
        },
        "159_version_tolerant_3.bytes",
        "159_version_tolerant_3.bytes"
    )?;

    // ============================================================================
    // 34-35. POLYMORPHISM - UNION (124-127)
    // ============================================================================

    test_case(
        "union_foo",
        &UnionSample::Foo(FooClass { xyz: 999 }),
        "124_union_foo.bytes",
        "124_union_foo.bytes"
    )?;
    test_case(
        "union_bar",
        &UnionSample::Bar(BarClass {
            opq: "bar value".to_string()
        }),
        "125_union_bar.bytes",
        "125_union_bar.bytes"
    )?;
    test_case(
        "abstract_union_a",
        &AbstractUnion::ConcreteA(ConcreteA {
            base_id: 1,
            a_value: "A".to_string()
        }),
        "126_abstract_union_a.bytes",
        "126_abstract_union_a.bytes"
    )?;
    test_case(
        "abstract_union_b",
        &AbstractUnion::ConcreteB(ConcreteB {
            base_id: 2,
            b_value: 3.14 // Using C# value instead of Rust's PI constant for exact match
        }),
        "127_abstract_union_b.bytes",
        "127_abstract_union_b.bytes"
    )?;

    println!("\nDESERIALIZING C# BYTES TO JSON\n");

    // Primitives
    deserialize_and_save::<u8>("01_byte", "01_byte.bytes")?;
    println!("01_byte.json");

    deserialize_and_save::<i8>("02_sbyte", "02_sbyte.bytes")?;
    println!("02_sbyte.json");

    deserialize_and_save::<i16>("03_short", "03_short.bytes")?;
    println!("03_short.json");

    deserialize_and_save::<u16>("04_ushort", "04_ushort.bytes")?;
    println!("04_ushort.json");

    deserialize_and_save::<i32>("05_int", "05_int.bytes")?;
    println!("05_int.json");

    deserialize_and_save::<u32>("06_uint", "06_uint.bytes")?;
    println!("06_uint.json");

    deserialize_and_save::<i64>("07_long", "07_long.bytes")?;
    println!("07_long.json");

    deserialize_and_save::<u64>("08_ulong", "08_ulong.bytes")?;
    println!("08_ulong.json");

    deserialize_and_save::<f32>("09_float", "09_float.bytes")?;
    println!("09_float.json");

    deserialize_and_save::<f64>("10_double", "10_double.bytes")?;
    println!("10_double.json");

    // Special float values - Infinity doesn't serialize to JSON, skip
    // deserialization
    deserialize_and_save::<f64>("142_double_max", "142_double_max.bytes")?;
    println!("142_double_max.json");

    deserialize_and_save::<f64>("143_double_min", "143_double_min.bytes")?;
    println!("143_double_min.json");

    deserialize_and_save::<bool>("11_bool_true", "11_bool_true.bytes")?;
    println!("11_bool_true.json");

    deserialize_and_save::<bool>("12_bool_false", "12_bool_false.bytes")?;
    println!("12_bool_false.json");

    deserialize_and_save::<char>("13_char", "13_char.bytes")?;
    println!("13_char.json");

    deserialize_and_save::<String>("14_string", "14_string.bytes")?;
    println!("14_string.json");

    deserialize_and_save::<String>("15_string_empty", "15_string_empty.bytes")?;
    println!("15_string_empty.json");

    #[cfg(feature = "nightly")]
    {
        deserialize_and_save::<Option<String>>("16_string_null", "16_string_null.bytes")?;
        println!("16_string_null.json");
    }
    #[cfg(not(feature = "nightly"))]
    {
        deserialize_and_save::<memorypack::NullableString>(
            "16_string_null",
            "16_string_null.bytes"
        )?;
        println!("16_string_null.json");
    }

    // Int128/UInt128
    deserialize_and_save::<i128>("19_int128", "19_int128.bytes")?;
    println!("19_int128.json");

    deserialize_and_save::<u128>("20_uint128", "20_uint128.bytes")?;
    println!("20_uint128.json");

    // Extended types
    #[cfg(feature = "rust_decimal")]
    {
        deserialize_and_save::<rust_decimal::Decimal>("17_decimal", "17_decimal.bytes")?;
        println!("17_decimal.json");
    }

    #[cfg(feature = "half")]
    {
        deserialize_and_save::<half::f16>("18_half", "18_half.bytes")?;
        println!("18_half.json");
    }

    #[cfg(feature = "uuid")]
    {
        deserialize_and_save::<uuid::Uuid>("21_guid", "21_guid.bytes")?;
        println!("21_guid.json");
    }

    // C# Rune as i32
    deserialize_and_save::<i32>("22_rune", "22_rune.bytes")?;
    println!("22_rune.json");

    #[cfg(feature = "num-bigint")]
    {
        deserialize_and_save::<num_bigint::BigInt>("23_biginteger", "23_biginteger.bytes")?;
        println!("23_biginteger.json");
    }

    #[cfg(feature = "url")]
    {
        deserialize_and_save::<url::Url>("30_uri", "30_uri.bytes")?;
        println!("30_uri.json");
    }

    #[cfg(feature = "chrono")]
    {
        deserialize_and_save::<chrono::TimeDelta>("24_timespan", "24_timespan.bytes")?;
        println!("24_timespan.json");

        deserialize_and_save::<chrono::DateTime<chrono::Utc>>("25_datetime", "25_datetime.bytes")?;
        println!("25_datetime.json");

        deserialize_and_save::<chrono::DateTime<chrono::FixedOffset>>(
            "26_datetimeoffset",
            "26_datetimeoffset.bytes"
        )?;
        println!("26_datetimeoffset.json");

        deserialize_and_save::<chrono::NaiveTime>("27_timeonly", "27_timeonly.bytes")?;
        println!("27_timeonly.json");

        deserialize_and_save::<chrono::NaiveDate>("28_dateonly", "28_dateonly.bytes")?;
        println!("28_dateonly.json");
    }

    // Arrays
    deserialize_and_save::<Vec<i32>>("44_array_1d_int", "44_array_1d_int.bytes")?;
    println!("44_array_1d_int.json");

    deserialize_and_save::<Vec<String>>("45_array_1d_string", "45_array_1d_string.bytes")?;
    println!("45_array_1d_string.json");

    // Multi-dimensional arrays
    deserialize_and_save::<memorypack::MultiDimArray<i32>>("46_array_2d", "46_array_2d.bytes")?;
    println!("46_array_2d.json");

    deserialize_and_save::<memorypack::MultiDimArray<i32>>("47_array_3d", "47_array_3d.bytes")?;
    println!("47_array_3d.json");

    deserialize_and_save::<memorypack::MultiDimArray<i32>>("48_array_4d", "48_array_4d.bytes")?;
    println!("48_array_4d.json");

    deserialize_and_save::<Vec<i32>>("49_array_empty", "49_array_empty.bytes")?;
    println!("49_array_empty.json");

    #[cfg(feature = "nightly")]
    {
        deserialize_and_save::<Option<Vec<i32>>>("50_array_null", "50_array_null.bytes")?;
        println!("50_array_null.json");
    }
    #[cfg(not(feature = "nightly"))]
    {
        deserialize_and_save::<memorypack::NullableVec<i32>>(
            "50_array_null",
            "50_array_null.bytes"
        )?;
        println!("50_array_null.json");
    }

    // Nullable types
    deserialize_and_save::<Option<i32>>("54_nullable_int_value", "54_nullable_int_value.bytes")?;
    println!("54_nullable_int_value.json");

    deserialize_and_save::<Option<i32>>("55_nullable_int_null", "55_nullable_int_null.bytes")?;
    println!("55_nullable_int_null.json");

    // Tuples
    deserialize_and_save::<(i32, String, f64)>("60_valuetuple3", "60_valuetuple3.bytes")?;
    println!("60_valuetuple3.json");

    deserialize_and_save::<(i32, i32, i32, i32, i32, i32, i32, i32)>(
        "61_valuetuple8",
        "61_valuetuple8.bytes"
    )?;
    println!("61_valuetuple8.json");

    // Lists
    deserialize_and_save::<Vec<i32>>("64_list_int", "64_list_int.bytes")?;
    println!("64_list_int.json");

    deserialize_and_save::<Vec<String>>("65_list_string", "65_list_string.bytes")?;
    println!("65_list_string.json");

    deserialize_and_save::<Vec<i32>>("66_list_empty", "66_list_empty.bytes")?;
    println!("66_list_empty.json");

    // More collections
    deserialize_and_save::<LinkedList<i32>>("67_linkedlist", "67_linkedlist.bytes")?;
    println!("67_linkedlist.json");

    deserialize_and_save::<VecDeque<String>>("68_queue", "68_queue.bytes")?;
    println!("68_queue.json");

    deserialize_and_save::<VecDeque<i32>>("69_stack", "69_stack.bytes")?;
    println!("69_stack.json");

    deserialize_and_save::<HashSet<i32>>("70_hashset", "70_hashset.bytes")?;
    println!("70_hashset.json");

    deserialize_and_save::<BTreeSet<String>>("71_sortedset", "71_sortedset.bytes")?;
    println!("71_sortedset.json");

    // Interface collections (deserialize to concrete types)
    deserialize_and_save::<Vec<i32>>("82_ienumerable", "82_ienumerable.bytes")?;
    println!("82_ienumerable.json");

    deserialize_and_save::<Vec<i32>>("83_icollection", "83_icollection.bytes")?;
    println!("83_icollection.json");

    deserialize_and_save::<Vec<i32>>("84_ilist", "84_ilist.bytes")?;
    println!("84_ilist.json");

    deserialize_and_save::<Vec<i32>>("85_ireadonlycollection", "85_ireadonlycollection.bytes")?;
    println!("85_ireadonlycollection.json");

    deserialize_and_save::<Vec<i32>>("86_ireadonlylist", "86_ireadonlylist.bytes")?;
    println!("86_ireadonlylist.json");

    deserialize_and_save::<HashSet<i32>>("87_iset", "87_iset.bytes")?;
    println!("87_iset.json");

    deserialize_and_save::<hashbrown::HashMap<String, i32>>(
        "88_idictionary",
        "88_idictionary.bytes"
    )?;
    println!("88_idictionary.json");

    deserialize_and_save::<hashbrown::HashMap<String, i32>>(
        "89_ireadonlydictionary",
        "89_ireadonlydictionary.bytes"
    )?;
    println!("89_ireadonlydictionary.json");

    // Dictionaries
    deserialize_and_save::<hashbrown::HashMap<String, i32>>(
        "73_dictionary",
        "73_dictionary.bytes"
    )?;
    println!("73_dictionary.json");

    deserialize_and_save::<hashbrown::HashMap<i32, String>>(
        "74_dictionary_int_key",
        "74_dictionary_int_key.bytes"
    )?;
    println!("74_dictionary_int_key.json");

    deserialize_and_save::<hashbrown::HashMap<String, i32>>(
        "146_dict_empty",
        "146_dict_empty.bytes"
    )?;
    println!("146_dict_empty.json");

    deserialize_and_save::<BTreeMap<i32, String>>(
        "76_sorteddictionary",
        "76_sorteddictionary.bytes"
    )?;
    println!("76_sorteddictionary.json");

    // Enums
    deserialize_and_save::<Color>("150_enum_simple", "150_enum_simple.bytes")?;
    println!("150_enum_simple.json");

    deserialize_and_save::<Permissions>("151_enum_flags", "151_enum_flags.bytes")?;
    println!("151_enum_flags.json");

    deserialize_and_save::<WithEnum>("152_class_with_enums", "152_class_with_enums.bytes")?;
    println!("152_class_with_enums.json");

    // Explicit order
    deserialize_and_save::<ExplicitOrder>("115_explicit_order", "115_explicit_order.bytes")?;
    println!("115_explicit_order.json");

    // Version tolerant
    deserialize_and_save::<VersionTolerant1>(
        "157_version_tolerant_1",
        "157_version_tolerant_1.bytes"
    )?;
    println!("157_version_tolerant_1.json");

    deserialize_and_save::<VersionTolerant2>(
        "158_version_tolerant_2",
        "158_version_tolerant_2.bytes"
    )?;
    println!("158_version_tolerant_2.json");

    deserialize_and_save::<VersionTolerant3>(
        "159_version_tolerant_3",
        "159_version_tolerant_3.bytes"
    )?;
    println!("159_version_tolerant_3.json");

    // Unions
    deserialize_and_save::<UnionSample>("124_union_foo", "124_union_foo.bytes")?;
    println!("124_union_foo.json");

    deserialize_and_save::<UnionSample>("125_union_bar", "125_union_bar.bytes")?;
    println!("125_union_bar.json");

    deserialize_and_save::<AbstractUnion>("126_abstract_union_a", "126_abstract_union_a.bytes")?;
    println!("126_abstract_union_a.json");

    deserialize_and_save::<AbstractUnion>("127_abstract_union_b", "127_abstract_union_b.bytes")?;
    println!("127_abstract_union_b.json");

    // Circular reference
    deserialize_and_save::<NodeWithCircular>(
        "132_circular_reference",
        "132_circular_reference.bytes"
    )?;
    println!("132_circular_reference.json");

    println!("\nALL TESTS COMPLETE");

    Ok(())
}
