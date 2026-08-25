using System.Collections;
using System.Collections.Concurrent;
using System.Collections.Immutable;
using System.Collections.ObjectModel;
using System.Globalization;
using System.Numerics;
using System.Text;
using MemoryPack;

public class FixtureTests
{
    [Test]
    [MethodDataSource(nameof(GetFixtureCases))]
    public void FixtureCompatibility(FixtureCase fixture)
    {
        fixture.Run();
    }

    public static IEnumerable<Func<FixtureCase>> GetFixtureCases()
    {
        var outputDir = Environment.GetEnvironmentVariable("MEMORYPACK_TEST_FIXTURES")
                        ?? Path.GetFullPath(Path.Combine(AppContext.BaseDirectory, "..", "..", "..", "..", "..", "..",
                            "crates", "memorypack-tests", "fixtures", "c#"));
        var rustOutputDir = Environment.GetEnvironmentVariable("MEMORYPACK_RUST_FIXTURES")
                            ?? Path.GetFullPath(Path.Combine(AppContext.BaseDirectory, "..", "..", "..", "..", "..",
                                "..", "crates", "memorypack-tests", "fixtures", "rust"));
        Directory.CreateDirectory(outputDir);

        var cases = new List<FixtureCase>();
        var unorderedFixtures = new HashSet<string>(StringComparer.Ordinal)
        {
            "70_hashset.bytes",
            "73_dictionary.bytes",
            "74_dictionary_int_key.bytes",
            "87_iset.bytes",
            "88_idictionary.bytes",
            "89_ireadonlydictionary.bytes",
            "93_concurrentdictionary.bytes",
            "97_immutablehashset.bytes",
            "101_immutabledictionary.bytes",
            "104_iimmutableset.bytes",
            "105_iimmutabledictionary.bytes",
            "136_large_dict_1k.bytes"
        };

        void SaveBytes<T>(T value, string filename, MemoryPackSerializerOptions? options = null)
        {
            cases.Add(new FixtureCase(filename, () =>
            {
                var bytes = MemoryPackSerializer.Serialize(value, options);
                var path = Path.Combine(outputDir, filename);
                File.WriteAllBytes(path, bytes);

                var rustPath = Path.Combine(rustOutputDir, filename);
                if (!File.Exists(rustPath)) return;

                var rustBytes = File.ReadAllBytes(rustPath);
                if (!unorderedFixtures.Contains(filename) && !rustBytes.AsSpan().SequenceEqual(bytes))
                    throw new InvalidOperationException("C# and Rust serialization differs");

                var valueFromRust = MemoryPackSerializer.Deserialize<T>(rustBytes);
                var reserialized = MemoryPackSerializer.Serialize(valueFromRust, options);
                if (!unorderedFixtures.Contains(filename) && !rustBytes.AsSpan().SequenceEqual(reserialized))
                    throw new InvalidOperationException("Rust bytes fail C# round-trip");
            }));
        }

        SaveBytes((byte)255, "01_byte.bytes");
        SaveBytes((sbyte)-128, "02_sbyte.bytes");
        SaveBytes((short)-32768, "03_short.bytes");
        SaveBytes((ushort)65535, "04_ushort.bytes");
        SaveBytes(42, "05_int.bytes");
        SaveBytes(4294967295u, "06_uint.bytes");
        SaveBytes(-9223372036854775808L, "07_long.bytes");
        SaveBytes(18446744073709551615UL, "08_ulong.bytes");
        SaveBytes(3.14159f, "09_float.bytes");
        SaveBytes(2.718281828459045, "10_double.bytes");
        SaveBytes(true, "11_bool_true.bytes");
        SaveBytes(false, "12_bool_false.bytes");
        SaveBytes('A', "13_char.bytes");
        SaveBytes("Hello, MemoryPack!", "14_string.bytes");
        SaveBytes("", "15_string_empty.bytes");
        SaveBytes((string?)null, "16_string_null.bytes");
        SaveBytes(123.456m, "17_decimal.bytes");
        SaveBytes((Half)3.14, "18_half.bytes");
        SaveBytes(new Int128(12345, 67890), "19_int128.bytes");
        SaveBytes(new UInt128(99999, 88888), "20_uint128.bytes");
        SaveBytes(Guid.Parse("12345678-1234-1234-1234-123456789abc"), "21_guid.bytes");
        SaveBytes(new Rune(0x1F389), "22_rune.bytes");
        SaveBytes(BigInteger.Parse("12345678901234567890"), "23_biginteger.bytes");

        SaveBytes(TimeSpan.FromMinutes(42.5), "24_timespan.bytes");
        SaveBytes(new DateTime(2025, 10, 19, 14, 30, 0, DateTimeKind.Utc), "25_datetime.bytes");
        SaveBytes(new DateTimeOffset(2025, 10, 19, 14, 30, 0, TimeSpan.FromHours(-5)), "26_datetimeoffset.bytes");
        SaveBytes(new TimeOnly(14, 30, 0), "27_timeonly.bytes");
        SaveBytes(new DateOnly(2025, 10, 19), "28_dateonly.bytes");
        SaveBytes(TimeZoneInfo.Utc, "29_timezoneinfo.bytes");

        SaveBytes(new Uri("https://github.com/Cysharp/MemoryPack"), "30_uri.bytes");
        SaveBytes(new Version(1, 2, 3, 4), "31_version.bytes");
        SaveBytes(new StringBuilder("StringBuilder content"), "32_stringbuilder.bytes");
        SaveBytes(typeof(int), "33_type.bytes");
        SaveBytes(new BitArray([true, false, true, true, false]), "34_bitarray.bytes");
        SaveBytes(CultureInfo.GetCultureInfo("en-US"), "35_cultureinfo.bytes");

        SaveBytes(new Complex(3.0, 4.0), "36_complex.bytes");
        SaveBytes(new Plane(1, 2, 3, 4), "37_plane.bytes");
        SaveBytes(new Quaternion(1, 2, 3, 4), "38_quaternion.bytes");
        SaveBytes(new Matrix3x2(1, 2, 3, 4, 5, 6), "39_matrix3x2.bytes");
        SaveBytes(Matrix4x4.Identity, "40_matrix4x4.bytes");
        SaveBytes(new Vector2(1.5f, 2.5f), "41_vector2.bytes");
        SaveBytes(new Vector3(1.5f, 2.5f, 3.5f), "42_vector3.bytes");
        SaveBytes(new Vector4(1.5f, 2.5f, 3.5f, 4.5f), "43_vector4.bytes");

        SaveBytes(new[] { 1, 2, 3, 4, 5 }, "44_array_1d_int.bytes");
        SaveBytes(new[] { "apple", "banana", "cherry" }, "45_array_1d_string.bytes");
        SaveBytes(new[,] { { 1, 2, 3 }, { 4, 5, 6 } }, "46_array_2d.bytes");
        SaveBytes(new[,,] { { { 1, 2 }, { 3, 4 } }, { { 5, 6 }, { 7, 8 } } }, "47_array_3d.bytes");
        SaveBytes(new[,,,] { { { { 1 } } } }, "48_array_4d.bytes");
        SaveBytes(Array.Empty<int>(), "49_array_empty.bytes");
        SaveBytes<int[]?>(null, "50_array_null.bytes");

        SaveBytes(new Memory<byte>([1, 2, 3, 4, 5]), "51_memory.bytes");
        SaveBytes(new ReadOnlyMemory<int>([10, 20, 30]), "52_readonlymemory.bytes");
        SaveBytes(new ArraySegment<byte>([1, 2, 3, 4, 5], 1, 3), "53_arraysegment.bytes");

        SaveBytes((int?)42, "54_nullable_int_value.bytes");
        SaveBytes((int?)null, "55_nullable_int_null.bytes");
        SaveBytes((DateTime?)DateTime.UtcNow, "56_nullable_datetime_value.bytes");
        SaveBytes((DateTime?)null, "57_nullable_datetime_null.bytes");

        SaveBytes(Tuple.Create(1, "two", 3.0), "58_tuple3.bytes");
        SaveBytes(Tuple.Create(1, 2, 3, 4, 5, 6, 7), "59_tuple7.bytes");
        SaveBytes((1, "two", 3.0), "60_valuetuple3.bytes");
        SaveBytes((1, 2, 3, 4, 5, 6, 7, 8), "61_valuetuple8.bytes");

        SaveBytes(new KeyValuePair<string, int>("age", 42), "62_keyvaluepair.bytes");

        SaveBytes(new Lazy<int>(() => 42), "63_lazy.bytes");

        SaveBytes(new List<int> { 1, 2, 3, 4, 5 }, "64_list_int.bytes");
        SaveBytes(new List<string> { "a", "b", "c" }, "65_list_string.bytes");
        SaveBytes(new List<int>(), "66_list_empty.bytes");

        var linkedList = new LinkedList<int>();
        linkedList.AddLast(1);
        linkedList.AddLast(2);
        linkedList.AddLast(3);
        SaveBytes(linkedList, "67_linkedlist.bytes");

        var queue = new Queue<string>();
        queue.Enqueue("first");
        queue.Enqueue("second");
        queue.Enqueue("third");
        SaveBytes(queue, "68_queue.bytes");

        var stack = new Stack<int>();
        stack.Push(1);
        stack.Push(2);
        stack.Push(3);
        SaveBytes(stack, "69_stack.bytes");

        SaveBytes(new HashSet<int> { 3, 1, 4, 1, 5, 9, 2, 6 }, "70_hashset.bytes");
        SaveBytes(new SortedSet<string> { "zebra", "apple", "mango" }, "71_sortedset.bytes");

        var priorityQueue = new PriorityQueue<string, int>();
        priorityQueue.Enqueue("low", 10);
        priorityQueue.Enqueue("high", 1);
        priorityQueue.Enqueue("medium", 5);
        SaveBytes(priorityQueue, "72_priorityqueue.bytes");

        SaveBytes(new Dictionary<string, int> { ["one"] = 1, ["two"] = 2, ["three"] = 3 }, "73_dictionary.bytes");
        SaveBytes(new Dictionary<int, string> { [1] = "one", [2] = "two" }, "74_dictionary_int_key.bytes");
        SaveBytes(new SortedList<string, int> { ["a"] = 1, ["b"] = 2 }, "75_sortedlist.bytes");
        SaveBytes(new SortedDictionary<int, string> { [3] = "three", [1] = "one", [2] = "two" },
            "76_sorteddictionary.bytes");

        var readOnlyDict = new ReadOnlyDictionary<string, int>(new Dictionary<string, int> { ["x"] = 10 });
        SaveBytes(readOnlyDict, "77_readonlydictionary.bytes");

        SaveBytes(new Collection<int> { 1, 2, 3 }, "78_collection.bytes");
        SaveBytes(new ReadOnlyCollection<string>(["a", "b", "c"]), "79_readonlycollection.bytes");
        SaveBytes(new ObservableCollection<int> { 10, 20, 30 }, "80_observablecollection.bytes");
        SaveBytes(new ReadOnlyObservableCollection<string>(["x", "y"]),
            "81_readonlyobservablecollection.bytes");

        SaveBytes<IEnumerable<int>>([1, 2, 3], "82_ienumerable.bytes");
        SaveBytes<ICollection<int>>(new List<int> { 1, 2, 3 }, "83_icollection.bytes");
        SaveBytes<IList<int>>(new List<int> { 1, 2, 3 }, "84_ilist.bytes");
        SaveBytes<IReadOnlyCollection<int>>([1, 2, 3], "85_ireadonlycollection.bytes");
        SaveBytes<IReadOnlyList<int>>([1, 2, 3], "86_ireadonlylist.bytes");
        SaveBytes<ISet<int>>(new HashSet<int> { 1, 2, 3 }, "87_iset.bytes");

        SaveBytes<IDictionary<string, int>>(new Dictionary<string, int> { ["a"] = 1 }, "88_idictionary.bytes");
        SaveBytes<IReadOnlyDictionary<string, int>>(new Dictionary<string, int> { ["a"] = 1 },
            "89_ireadonlydictionary.bytes");

        var concurrentBag = new ConcurrentBag<int>
        {
            1,
            2,
            3
        };
        SaveBytes(concurrentBag, "90_concurrentbag.bytes");

        var concurrentQueue = new ConcurrentQueue<string>();
        concurrentQueue.Enqueue("a");
        concurrentQueue.Enqueue("b");
        concurrentQueue.Enqueue("c");
        SaveBytes(concurrentQueue, "91_concurrentqueue.bytes");

        var concurrentStack = new ConcurrentStack<int>();
        concurrentStack.Push(1);
        concurrentStack.Push(2);
        concurrentStack.Push(3);
        SaveBytes(concurrentStack, "92_concurrentstack.bytes");

        SaveBytes(new ConcurrentDictionary<string, int>(new Dictionary<string, int> { ["key"] = 42 }),
            "93_concurrentdictionary.bytes");

        var blockingCollection = new BlockingCollection<int>();
        blockingCollection.Add(1);
        blockingCollection.Add(2);
        blockingCollection.Add(3);
        SaveBytes(blockingCollection, "94_blockingcollection.bytes");

        SaveBytes(ImmutableArray.Create(1, 2, 3), "95_immutablearray.bytes");
        SaveBytes(ImmutableList.Create("a", "b", "c"), "96_immutablelist.bytes");
        SaveBytes(ImmutableHashSet.Create(1, 2, 3), "97_immutablehashset.bytes");
        SaveBytes(ImmutableSortedSet.Create("z", "a", "m"), "98_immutablesortedset.bytes");
        SaveBytes(ImmutableQueue.Create(1, 2, 3), "99_immutablequeue.bytes");
        SaveBytes(ImmutableStack.Create(1, 2, 3), "100_immutablestack.bytes");
        SaveBytes(ImmutableDictionary.Create<string, int>().Add("key", 42), "101_immutabledictionary.bytes");
        SaveBytes(ImmutableSortedDictionary.Create<string, int>().Add("key", 42),
            "102_immutablesorteddictionary.bytes");

        SaveBytes<IImmutableList<int>>(ImmutableList.Create(1, 2, 3), "103_iimmutablelist.bytes");
        SaveBytes<IImmutableSet<int>>(ImmutableHashSet.Create(1, 2, 3), "104_iimmutableset.bytes");
        SaveBytes<IImmutableDictionary<string, int>>(ImmutableDictionary.Create<string, int>().Add("k", 1),
            "105_iimmutabledictionary.bytes");

        var lookup = new[] { "apple", "apricot", "banana", "blueberry" }
            .ToLookup(s => s[0]);
        SaveBytes(lookup, "106_ilookup.bytes");

        var grouping = lookup.First();
        SaveBytes(grouping, "107_igrouping.bytes");

        SaveBytes(new Person { Age = 42, Name = "John Doe" }, "108_person_class.bytes");
        SaveBytes((Person?)null, "109_person_class_null.bytes");

        SaveBytes(new Point { X = 10, Y = 20 }, "110_point_struct.bytes");

        SaveBytes(new PersonRecord(30, "Jane Smith"), "111_person_record.bytes");
        SaveBytes(new PointRecord(15, 25), "112_point_record_struct.bytes");

        SaveBytes(new MixedMembers(100, 200)
        {
            PublicField = 1,
            PublicProperty = 2,
            InitProperty = 3
        }, "113_mixed_members.bytes");

        var includeIgnoreSample = new IncludeIgnoreSample
        {
            PublicValue = 10,
            IgnoredValue = 999
        };
        includeIgnoreSample.SetPrivateValue(42);
        SaveBytes(includeIgnoreSample, "114_include_ignore_sample.bytes");

        SaveBytes(new ExplicitOrder { First = 1, Second = 2, Third = 3 }, "115_explicit_order.bytes");

        SaveBytes(new UnmanagedStruct { A = 42, B = 3.14f, C = 255 }, "116_unmanaged_struct.bytes");

        SaveBytes(new ParameterizedConstructor(100, "test"), "117_parameterized_constructor.bytes");

        SaveBytes(new MultipleConstructors(42), "118_multiple_constructors.bytes");

        SaveBytes(new DerivedClass { BaseValue = 10, DerivedValue = "derived" }, "119_inheritance_derived.bytes");

        SaveBytes(new Outer
        {
            OuterId = 1,
            InnerObject = new Outer.Inner { InnerName = "nested" }
        }, "120_nested_types.bytes");

        SaveBytes(new GenericContainer<int> { Value = 42 }, "121_generic_int.bytes");
        SaveBytes(new GenericContainer<string> { Value = "generic" }, "122_generic_string.bytes");
        SaveBytes(new GenericContainer<List<int>> { Value = [1, 2, 3] }, "123_generic_list.bytes");

        SaveBytes<IUnionSample>(new FooClass { XYZ = 999 }, "124_union_foo.bytes");
        SaveBytes<IUnionSample>(new BarClass { OPQ = "bar value" }, "125_union_bar.bytes");

        SaveBytes<AbstractBase>(new ConcreteA { BaseId = 1, AValue = "A" }, "126_abstract_union_a.bytes");
        SaveBytes<AbstractBase>(new ConcreteB { BaseId = 2, BValue = 3.14 }, "127_abstract_union_b.bytes");

        var myList = new MyList<int> { 10, 20, 30 };
        SaveBytes(myList, "128_custom_list.bytes");

        var myDict = new MyStringDictionary<int> { ["a"] = 1, ["b"] = 2 };
        SaveBytes(myDict, "129_custom_dictionary.bytes");

        SaveBytes(new CallbackSample { Value = 100 }, "130_callbacks.bytes");

        SaveBytes(new ComplexNested
        {
            Data =
            [
                new Dictionary<string, int[]> { ["first"] = [1, 2, 3], ["second"] = [4, 5] },
                new Dictionary<string, int[]> { ["third"] = [6, 7, 8, 9] }
            ]
        }, "131_complex_nested.bytes");

        var node1 = new NodeWithCircular { Id = 1 };
        var node2 = new NodeWithCircular { Id = 2 };
        var node3 = new NodeWithCircular { Id = 3 };
        node1.Next = node2;
        node2.Next = node3;
        node3.Next = node1;

        SaveBytes(node1, "132_circular_reference.bytes");

        const string stringData = "Hello MemoryPack! こんにちは 你好 مرحبا";
        SaveBytes(stringData, "133_string_utf8.bytes", MemoryPackSerializerOptions.Utf8);
        SaveBytes(stringData, "134_string_utf16.bytes", MemoryPackSerializerOptions.Utf16);

        SaveBytes(Enumerable.Range(0, 10000).ToArray(), "135_large_array_10k.bytes");
        SaveBytes(Enumerable.Range(0, 1000).ToDictionary(x => $"key{x}", x => x), "136_large_dict_1k.bytes");

        SaveBytes(double.NaN, "137_double_nan.bytes");
        SaveBytes(double.PositiveInfinity, "138_double_positive_infinity.bytes");
        SaveBytes(double.NegativeInfinity, "139_double_negative_infinity.bytes");
        SaveBytes(float.NaN, "140_float_nan.bytes");
        SaveBytes(double.Epsilon, "141_double_epsilon.bytes");
        SaveBytes(double.MaxValue, "142_double_max.bytes");
        SaveBytes(double.MinValue, "143_double_min.bytes");

        SaveBytes(new List<int>(), "144_list_empty.bytes");
        SaveBytes((List<int>?)null, "145_list_null.bytes");
        SaveBytes(new Dictionary<string, int>(), "146_dict_empty.bytes");
        SaveBytes((Dictionary<string, int>?)null, "147_dict_null.bytes");

        SaveBytes(new NullableStruct { NullableInt = 42, NullableString = "value" },
            "148_nullable_struct_values.bytes");
        SaveBytes(new NullableStruct { NullableInt = null, NullableString = null }, "149_nullable_struct_nulls.bytes");

        SaveBytes(Color.Green, "150_enum_simple.bytes");
        SaveBytes(Permissions.Read | Permissions.Write, "151_enum_flags.bytes");
        SaveBytes(new WithEnum { FavoriteColor = Color.Blue, UserPermissions = Permissions.Read | Permissions.Execute },
            "152_class_with_enums.bytes");

        SaveBytes(new RequiredMembersSample { RequiredInt = 42, RequiredString = "required" },
            "153_required_members.bytes");

        SaveBytes(new ReadOnlyPoint(50, 60), "154_readonly_struct.bytes");

        SaveBytes(new MultiDimensionalData
        {
            Matrix2D = new[,] { { 1, 2, 3 }, { 4, 5, 6 } },
            Matrix3D = new[,,] { { { 1, 2 }, { 3, 4 } }, { { 5, 6 }, { 7, 8 } } }
        }, "155_multidimensional_arrays.bytes");

        SaveBytes(new VersionTolerant0(), "156_version_tolerant_0.bytes");
        SaveBytes(new VersionTolerant1 { MyProperty1 = 1000 }, "157_version_tolerant_1.bytes");
        SaveBytes(new VersionTolerant2 { MyProperty1 = 3000, MyProperty2 = 9999 }, "158_version_tolerant_2.bytes");
        SaveBytes(new VersionTolerant3 { MyProperty1 = 444, MyProperty2 = 2452, MyProperty3 = 32 },
            "159_version_tolerant_3.bytes");
        SaveBytes(new VersionTolerantWithSkippedField { MyProperty1 = 99, MyProperty3 = 13 },
            "160_version_tolerant_skipped_field.bytes");
        SaveBytes(new VersionTolerantSparse { MyProperty3 = 5000, MyProperty6 = [1, 10, 100] },
            "161_version_tolerant_sparse.bytes");
        SaveBytes(
            new VersionTolerantWithComplexType
                { MyProperty1 = new Version(1, 2, 3, 4), MyProperty2 = 999, MyProperty3 = 42 },
            "162_version_tolerant_complex.bytes");
        SaveBytes(
            new VersionTolerantWithNullable
                { NullableInt = 42, NullableString = "test", NullableDateTime = new DateTime(2025, 10, 19) },
            "163_version_tolerant_nullable_values.bytes");
        SaveBytes(
            new VersionTolerantWithNullable { NullableInt = null, NullableString = null, NullableDateTime = null },
            "164_version_tolerant_nullable_nulls.bytes");

        return cases.Select(x => new Func<FixtureCase>(() => x));
    }
}

public sealed record FixtureCase(string Name, Action Run)
{
    public override string ToString()
    {
        return Name;
    }
}