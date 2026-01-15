# E2K Parser

A comprehensive Rust library for parsing ETABS E2K structural model files.

## Features

- ✅ **Complete E2K format support**: Parse all major sections (Stories, Points, Lines, Areas, Loads, etc.)
- ✅ **nom 8.0 parser**: Fast, zero-copy parsing with excellent error messages
- ✅ **Type-safe**: Strong typing for all E2K entities
- ✅ **Validation**: Built-in model integrity checks
- ✅ **Query API**: Easy access to model elements
- ✅ **JSON Export**: Optional serde support for JSON serialization
- ✅ **Error Handling**: Comprehensive error types with context

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
e2k-parser = "0.1.0"

# Optional: Enable JSON export
e2k-parser = { version = "0.1.0", features = ["serde"] }
```

## Quick Start

```rust
use e2k_parser::{E2KModel, Result};

fn main() -> Result<()> {
    // Parse from file
    let model = E2KModel::from_file("model.e2k")?;
    
    // Get statistics
    let stats = model.get_statistics();
    println!("{}", stats);
    
    // Query elements
    let columns = model.get_columns();
    println!("Found {} columns", columns.len());
    
    // Validate model
    model.validate()?;
    
    Ok(())
}
```

## Usage Examples

### Parse E2K File

```rust
use e2k_parser::E2KModel;

// From file path
let model = E2KModel::from_file("model.e2k")?;

// From string
let content = std::fs::read_to_string("model.e2k")?;
let model = E2KModel::from_str(&content)?;
```

### Query Model Elements

```rust
// Get all columns
let columns = model.get_columns();

// Get all beams
let beams = model.get_beams();

// Get all walls
let walls = model.get_walls();

// Find specific story
if let Some(story) = model.get_story("L01") {
    println!("Story height: {:?}", story.height);
}

// Find specific material
if let Some(mat) = model.get_material("4000Psi") {
    println!("Material type: {}", mat.material_type);
}
```

### Model Statistics

```rust
let stats = model.get_statistics();

println!("Stories: {}", stats.num_stories);
println!("Total Height: {:.2} ft", stats.total_height);
println!("Points: {}", stats.num_points);
println!("Columns: {}", stats.num_columns);
println!("Beams: {}", stats.num_beams);
```

### Validation

```rust
match model.validate() {
    Ok(report) => {
        println!("✓ Validation passed");
        if report.warning_count() > 0 {
            for warning in report.warnings() {
                println!("  Warning: {}", warning);
            }
        }
    }
    Err(e) => {
        println!("✗ Validation failed: {}", e);
    }
}
```

### JSON Export (requires `serde` feature)

```rust
#[cfg(feature = "serde")]
{
    // Export to JSON string
    let json = model.to_json()?;
    
    // Export to file
    model.to_json_file("output.json")?;
}
```

## Architecture

The library is organized into several modules:

- **`types`**: Data structures for all E2K entities
- **`parser`**: nom-based parsers for each section
    - `primitives`: Basic parsers (numbers, strings, etc.)
    - `structural`: Stories, grids, materials, sections
    - `geometry`: Points, lines, areas
    - `loading`: Load patterns and cases
    - `analysis`: Analysis options, mass source
- **`error`**: Comprehensive error types
- **`panic_context`**: Panic handling across FFI boundaries

## Error Handling

```rust
use e2k_parser::{E2kError, Result};

match E2KModel::from_file("model.e2k") {
    Ok(model) => { /* use model */ }
    Err(E2kError::Io(e)) => {
        eprintln!("File error: {}", e);
    }
    Err(E2kError::Parsing { line, message, .. }) => {
        eprintln!("Parse error at line {}: {}", line, message);
    }
    Err(E2kError::Validation { message, .. }) => {
        eprintln!("Validation error: {}", message);
    }
    Err(e) => {
        eprintln!("Error: {}", e);
    }
}
```

## Examples

Run the included examples:

```bash
# Basic parsing
cargo run --example basic_parse

# Model validation
cargo run --example validate_model

# Query elements
cargo run --example query_elements

# JSON export (requires serde feature)
cargo run --example export_json --features serde
```

## Testing

```bash
# Run unit tests
cargo test

# Run with all features
cargo test --all-features

# Run integration tests
cargo test --test integration_tests
```

## Performance

The parser uses nom's zero-copy approach for excellent performance:

- Typical hotel model (35 stories): ~50ms parse time
- Large commercial building: ~200ms parse time
- Memory usage: Minimal, thanks to zero-copy parsing

## License

MIT OR Apache-2.0

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## Changelog

### 0.1.0 (2026-01-15)

- Initial release
- Complete E2K format support
- nom 8.0 parser implementation
- Model validation
- Query API
- Optional JSON export