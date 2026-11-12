# MemoryPack.rs

A binary wire compatible Rust implementation of [Cysharp/MemoryPack](https://github.com/Cysharp/MemoryPack).

## Install

```shell
cargo add memorypack
```

## Quick Start

Derive `MemoryPackable` on your type, then use `MemoryPackSerializer` to serialize and deserialize it.

```rust
use memorypack::{MemoryPackable, MemoryPackError, MemoryPackSerializer};

#[derive(MemoryPackable, Debug, PartialEq)]
struct Person {
    age: i32,
    name: String,
}

fn main() -> Result<(), MemoryPackError> {
    let person = Person {
        age: 40,
        name: "John".to_owned(),
    };

    let bytes = MemoryPackSerializer::serialize(&person)?;
    let decoded: Person = MemoryPackSerializer::deserialize(&bytes)?;

    assert_eq!(person, decoded);
    Ok(())
}
```

## Built-in Supported Types

Primitives, strings, arrays, tuples, options, and standard collections work out of the box. 

Enable `extended_types` for UUIDs, decimals, half floats, big integers, Jiff dates, Glam types, complex numbers, and URLs.

## Union

Use `#[memorypack(union)]` on an enum whose variants each wrap one serializable type. 

Tags default to declaration order, use `#[tag(n)]` to keep them stable when reordering variants.

```rust
use memorypack::MemoryPackable;

#[derive(MemoryPackable)]
#[memorypack(union)]
enum Shape {
    #[tag(0)]
    Circle(Circle),
    #[tag(1)]
    Rectangle(Rectangle),
}

#[derive(MemoryPackable)]
struct Circle {
    radius: f32,
}

#[derive(MemoryPackable)]
struct Rectangle {
    width: f32,
    height: f32,
}
```

## Version Tolerant

Use `#[memorypack(version_tolerant)]` and give every field a stable `order`.

```rust
use memorypack::MemoryPackable;

#[derive(MemoryPackable)]
#[memorypack(version_tolerant)]
struct Player {
    #[memorypack(order = 0)]
    id: i32,
    #[memorypack(order = 1)]
    name: String,
}
```

## Custom Formatter

Implement `MemoryPackFormatter<T>`, then attach it to a field with `#[memorypack(formatter = ...)]`.

```rust
use std::net::Ipv4Addr;

use memorypack::{
    MemoryPackError, MemoryPackFormatter, MemoryPackReader, MemoryPackWriter, MemoryPackable,
};

#[derive(MemoryPackable)]
struct Endpoint {
    #[memorypack(formatter = Ipv4Formatter)]
    address: Option<Ipv4Addr>,
    port: u16,
}

struct Ipv4Formatter;

impl MemoryPackFormatter<Option<Ipv4Addr>> for Ipv4Formatter {
    fn serialize(
        value: &Option<Ipv4Addr>,
        writer: &mut MemoryPackWriter,
    ) -> Result<(), MemoryPackError> {
        let address = value.ok_or_else(|| {
            MemoryPackError::SerializationError("IPv4 address cannot be null".into())
        })?;
        for octet in address.octets() {
            writer.write_u8(octet)?;
        }
        Ok(())
    }

    fn deserialize(reader: &mut MemoryPackReader<'_>) -> Result<Option<Ipv4Addr>, MemoryPackError> {
        Ok(Some(Ipv4Addr::new(
            reader.read_u8()?,
            reader.read_u8()?,
            reader.read_u8()?,
            reader.read_u8()?,
        )))
    }

    fn serialized_size_hint(_: &Option<Ipv4Addr>) -> usize {
        4
    }
}
```

See the [API documentation](https://docs.rs/memorypack) and [cross-language tests](tests/rust-tests/tests/cross_language.rs) for more examples.
