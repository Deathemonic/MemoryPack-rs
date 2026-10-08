using MemoryPack;

[MemoryPackable]
partial class Person
{
    public int Age { get; set; }
    public string Name { get; set; } = "";
}

[MemoryPackable]
partial struct Point
{
    public int X;
    public int Y;
}

[MemoryPackable]
partial record PersonRecord(int Age, string Name);

[MemoryPackable]
partial record struct PointRecord(int X, int Y);

[MemoryPackable]
partial class MixedMembers
{
    public int PublicField;
    public readonly int PublicReadOnlyField;
    public int PublicProperty { get; set; }
    public int PrivateSetPublicProperty { get; private set; }
    public int ReadOnlyPublicProperty { get; }
    public int InitProperty { get; init; }

    public MixedMembers(int publicReadOnlyField, int readOnlyPublicProperty)
    {
        PublicReadOnlyField = publicReadOnlyField;
        ReadOnlyPublicProperty = readOnlyPublicProperty;
    }
}

[MemoryPackable]
partial class IncludeIgnoreSample
{
    public int PublicValue { get; set; }

    [MemoryPackIgnore]
    public int IgnoredValue { get; set; }

    [MemoryPackInclude]
    private int privateValue;

    public void SetPrivateValue(int value) => privateValue = value;
}

[MemoryPackable(SerializeLayout.Explicit)]
partial class ExplicitOrder
{
    [MemoryPackOrder(2)]
    public int Third { get; set; }

    [MemoryPackOrder(0)]
    public int First { get; set; }

    [MemoryPackOrder(1)]
    public int Second { get; set; }
}

[MemoryPackable]
partial struct UnmanagedStruct
{
    public int A;
    public float B;
    public byte C;
}

[MemoryPackable]
partial class ParameterizedConstructor
{
    public readonly int X;
    public readonly string Y;

    public ParameterizedConstructor(int x, string y)
    {
        X = x;
        Y = y;
    }
}

[MemoryPackable]
partial class MultipleConstructors
{
    public int Value { get; set; }

    public MultipleConstructors()
    {
        Value = 0;
    }

    [MemoryPackConstructor]
    public MultipleConstructors(int value)
    {
        Value = value;
    }
}

[MemoryPackable]
partial class BaseClass
{
    public int BaseValue { get; set; }
}

[MemoryPackable]
partial class DerivedClass : BaseClass
{
    public string DerivedValue { get; set; } = "";
}

[MemoryPackable]
partial class Outer
{
    public int OuterId { get; set; }
    public Inner? InnerObject { get; set; }

    [MemoryPackable]
    public partial class Inner
    {
        public string InnerName { get; set; } = "";
    }
}

[MemoryPackable]
partial class GenericContainer<T>
{
    public T? Value { get; set; }
}

[MemoryPackable]
[MemoryPackUnion(0, typeof(FooClass))]
[MemoryPackUnion(1, typeof(BarClass))]
partial interface IUnionSample
{
}

[MemoryPackable]
partial class FooClass : IUnionSample
{
    public int XYZ { get; set; }
}

[MemoryPackable]
partial class BarClass : IUnionSample
{
    public string? OPQ { get; set; }
}

[MemoryPackable]
[MemoryPackUnion(0, typeof(ConcreteA))]
[MemoryPackUnion(1, typeof(ConcreteB))]
abstract partial class AbstractBase
{
    public int BaseId { get; set; }
}

[MemoryPackable]
partial class ConcreteA : AbstractBase
{
    public string AValue { get; set; } = "";
}

[MemoryPackable]
partial class ConcreteB : AbstractBase
{
    public double BValue { get; set; }
}

[MemoryPackable(GenerateType.Collection)]
partial class MyList<T> : List<T>
{
}

[MemoryPackable(GenerateType.Collection)]
partial class MyStringDictionary<TValue> : Dictionary<string, TValue>
{
}

[MemoryPackable]
partial class CallbackSample
{
    public int Value { get; set; }
    public int CallCount { get; set; }

    [MemoryPackOnSerializing]
    void OnSerializing()
    {
        CallCount++;
    }

    [MemoryPackOnSerialized]
    void OnSerialized()
    {
    }

    [MemoryPackOnDeserializing]
    static void OnDeserializing()
    {
    }

    [MemoryPackOnDeserialized]
    void OnDeserialized()
    {
    }
}

[MemoryPackable]
partial class ComplexNested
{
    public List<Dictionary<string, int[]>>? Data { get; set; }
}

[MemoryPackable(GenerateType.CircularReference)]
partial class NodeWithCircular
{
    [MemoryPackOrder(0)]
    public int Id { get; set; }
    
    [MemoryPackOrder(1)]
    public NodeWithCircular? Next { get; set; }
}

[MemoryPackable]
partial struct NullableStruct
{
    public int? NullableInt;
    public string? NullableString;
}

enum Color { Red, Green, Blue }

[Flags]
enum Permissions { None = 0, Read = 1, Write = 2, Execute = 4 }

[MemoryPackable]
partial class WithEnum
{
    public Color FavoriteColor { get; set; }
    public Permissions UserPermissions { get; set; }
}

[MemoryPackable]
partial class RequiredMembersSample
{
    public required int RequiredInt { get; init; }
    public required string RequiredString { get; init; }
    public int OptionalInt { get; init; }
}

[MemoryPackable]
readonly partial struct ReadOnlyPoint
{
    public readonly int X;
    public readonly int Y;

    public ReadOnlyPoint(int x, int y)
    {
        X = x;
        Y = y;
    }
}

[MemoryPackable]
partial class MultiDimensionalData
{
    public int[,]? Matrix2D { get; set; }
    public int[,,]? Matrix3D { get; set; }
}

[MemoryPackable(GenerateType.VersionTolerant)]
partial class VersionTolerant0
{
}

[MemoryPackable(GenerateType.VersionTolerant)]
partial class VersionTolerant1
{
    [MemoryPackOrder(0)]
    public int MyProperty1 { get; set; }
}

[MemoryPackable(GenerateType.VersionTolerant)]
partial class VersionTolerant2
{
    [MemoryPackOrder(0)]
    public int MyProperty1 { get; set; }

    [MemoryPackOrder(1)]
    public long MyProperty2 { get; set; }
}

[MemoryPackable(GenerateType.VersionTolerant)]
partial class VersionTolerant3
{
    [MemoryPackOrder(0)]
    public int MyProperty1 { get; set; }

    [MemoryPackOrder(1)]
    public long MyProperty2 { get; set; }

    [MemoryPackOrder(2)]
    public short MyProperty3 { get; set; }
}

[MemoryPackable(GenerateType.VersionTolerant)]
partial class VersionTolerantWithSkippedField
{
    [MemoryPackOrder(0)]
    public int MyProperty1 { get; set; }

    [MemoryPackOrder(2)]
    public short MyProperty3 { get; set; }
}

[MemoryPackable(GenerateType.VersionTolerant)]
partial class VersionTolerantSparse
{
    [MemoryPackOrder(2)]
    public short MyProperty3 { get; set; }

    [MemoryPackOrder(5)]
    public ushort[] MyProperty6 { get; set; } = Array.Empty<ushort>();
}

[MemoryPackable(GenerateType.VersionTolerant)]
partial class VersionTolerantWithComplexType
{
    [MemoryPackOrder(0)]
    public Version MyProperty1 { get; set; } = new Version(1, 0);

    [MemoryPackOrder(1)]
    public long MyProperty2 { get; set; }

    [MemoryPackOrder(2)]
    public short MyProperty3 { get; set; }
}

[MemoryPackable(GenerateType.VersionTolerant)]
partial class VersionTolerantWithNullable
{
    [MemoryPackOrder(0)]
    public int? NullableInt { get; set; }

    [MemoryPackOrder(1)]
    public string? NullableString { get; set; }

    [MemoryPackOrder(2)]
    public DateTime? NullableDateTime { get; set; }
}