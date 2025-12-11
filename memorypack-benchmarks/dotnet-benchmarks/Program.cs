using BenchmarkDotNet.Attributes;
using BenchmarkDotNet.Exporters.Json;
using BenchmarkDotNet.Running;
using MemoryPack;

BenchmarkRunner.Run<MemoryPackBenchmarks>();

[MemoryPackable]
public partial class SimpleData
{
    public int Id { get; set; }
    public string Name { get; set; } = "";
    public double Value { get; set; }
    public bool IsActive { get; set; }
}

[MemoryPackable]
public partial class ComplexData
{
    public int Id { get; set; }
    public string Name { get; set; } = "";
    public List<int> Numbers { get; set; } = new();
    public Dictionary<string, string> Properties { get; set; } = new();
    public SimpleData? Nested { get; set; }
}

[MemoryPackable(GenerateType.VersionTolerant)]
public partial class VersionTolerantData
{
    [MemoryPackOrder(0)]
    public int Property1 { get; set; }
    [MemoryPackOrder(1)]
    public string Property2 { get; set; } = "";
    [MemoryPackOrder(2)]
    public double Property3 { get; set; }
}

public enum Color { Red = 0, Green = 1, Blue = 2 }

[MemoryPackable]
[MemoryPackUnion(0, typeof(FooClass))]
[MemoryPackUnion(1, typeof(BarClass))]
public partial interface IUnionSample { }

[MemoryPackable]
public partial class FooClass : IUnionSample
{
    public int XYZ { get; set; }
}

[MemoryPackable]
public partial class BarClass : IUnionSample
{
    public string? OPQ { get; set; }
}



[MemoryDiagnoser]
[JsonExporterAttribute.Full]
public class MemoryPackBenchmarks
{
    private SimpleData simpleData = null!;
    private ComplexData complexData = null!;
    private VersionTolerantData versionTolerantData = null!;
    private Color enumData;
    private IUnionSample unionData = null!;
    private byte[] simpleBytes = null!;
    private byte[] complexBytes = null!;
    private byte[] versionTolerantBytes = null!;
    private byte[] enumBytes = null!;
    private byte[] unionBytes = null!;

    [GlobalSetup]
    public void Setup()
    {
        simpleData = new SimpleData 
        { 
            Id = 42, 
            Name = "Test Data", 
            Value = 3.14159, 
            IsActive = true 
        };

        complexData = new ComplexData
        {
            Id = 100,
            Name = "Complex Test",
            Numbers = Enumerable.Range(1, 100).ToList(),
            Properties = Enumerable.Range(1, 50).ToDictionary(i => $"key{i}", i => $"value{i}"),
            Nested = new SimpleData { Id = 1, Name = "Nested", Value = 1.23, IsActive = false }
        };

        versionTolerantData = new VersionTolerantData
        {
            Property1 = 1000,
            Property2 = "Version Tolerant",
            Property3 = 99.99
        };

        enumData = Color.Green;

        unionData = new FooClass { XYZ = 999 };

        simpleBytes = MemoryPackSerializer.Serialize(simpleData);
        complexBytes = MemoryPackSerializer.Serialize(complexData);
        versionTolerantBytes = MemoryPackSerializer.Serialize(versionTolerantData);
        enumBytes = MemoryPackSerializer.Serialize(enumData);
        unionBytes = MemoryPackSerializer.Serialize(unionData);
    }

    [Benchmark]
    public byte[] SerializeSimple()
    {
        return MemoryPackSerializer.Serialize(simpleData);
    }

    [Benchmark]
    public SimpleData DeserializeSimple()
    {
        return MemoryPackSerializer.Deserialize<SimpleData>(simpleBytes)!;
    }

    [Benchmark]
    public byte[] SerializeComplex()
    {
        return MemoryPackSerializer.Serialize(complexData);
    }

    [Benchmark]
    public ComplexData DeserializeComplex()
    {
        return MemoryPackSerializer.Deserialize<ComplexData>(complexBytes)!;
    }

    [Benchmark]
    public byte[] SerializeVersionTolerant()
    {
        return MemoryPackSerializer.Serialize(versionTolerantData);
    }

    [Benchmark]
    public VersionTolerantData DeserializeVersionTolerant()
    {
        return MemoryPackSerializer.Deserialize<VersionTolerantData>(versionTolerantBytes)!;
    }

    [Benchmark]
    public byte[] SerializeEnum()
    {
        return MemoryPackSerializer.Serialize(enumData);
    }

    [Benchmark]
    public Color DeserializeEnum()
    {
        return MemoryPackSerializer.Deserialize<Color>(enumBytes);
    }

    [Benchmark]
    public byte[] SerializeUnion()
    {
        return MemoryPackSerializer.Serialize(unionData);
    }

    [Benchmark]
    public IUnionSample DeserializeUnion()
    {
        return MemoryPackSerializer.Deserialize<IUnionSample>(unionBytes)!;
    }

}
