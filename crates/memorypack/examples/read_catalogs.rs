use std::fs;

use hashbrown::HashMap;
use memorypack::prelude::*;
use serde::{Deserialize, Serialize};

// Example: Deserialize MemoryPack data from binary files to JSON
#[derive(MemoryPackable, Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct MediaData {
    path: String,
    file_name: String,
    bytes: i64,
    crc: i64,
    is_prologue: bool,
    is_split_download: bool,
    media_type: i32
}

#[derive(MemoryPackable, Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct TableData {
    name: String,
    size: i64,
    crc: i64,
    is_in_build: bool,
    is_changed: bool,
    is_prologue: bool,
    is_split_download: bool,
    includes: Vec<String>
}

#[derive(MemoryPackable, Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "PascalCase")]
struct MediaCatalog {
    table: HashMap<String, MediaData>
}

#[derive(MemoryPackable, Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "PascalCase")]
struct TableCatalog {
    table: HashMap<String, TableData>
}

fn main() -> eyre::Result<()> {
    println!("=== MemoryPack Catalog Deserializer ===\n");

    // Deserialize MediaCatalog
    println!("Reading MediaCatalog.bytes...");
    let media_bytes = fs::read("memorypack/examples/MediaCatalog.bytes")
        .expect("Failed to read MediaCatalog.bytes");

    let media_catalog = MemoryPackSerializer::deserialize::<MediaCatalog>(&media_bytes)?;
    println!("✓ Deserialized {} media entries", media_catalog.table.len());

    let json = serde_json::to_string_pretty(&media_catalog).expect("Failed to serialize to JSON");
    fs::write("MediaCatalog.json", json).expect("Failed to write MediaCatalog.json");
    println!("✓ Saved to MediaCatalog.json\n");

    // Deserialize TableCatalog
    println!("Reading TableCatalog.bytes...");
    let table_bytes = fs::read("memorypack/examples/TableCatalog.bytes")
        .expect("Failed to read TableCatalog.bytes");

    let table_catalog = MemoryPackSerializer::deserialize::<TableCatalog>(&table_bytes)?;
    println!("✓ Deserialized {} table entries", table_catalog.table.len());

    let json = serde_json::to_string_pretty(&table_catalog).expect("Failed to serialize to JSON");
    fs::write("TableCatalog.json", json).expect("Failed to write TableCatalog.json");
    println!("✓ Saved to TableCatalog.json\n");

    println!("=== Complete! ===");
    Ok(())
}
