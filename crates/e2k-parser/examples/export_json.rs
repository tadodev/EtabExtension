#[cfg(feature = "serde")]
use e2k_parser::{E2KModel, Result};

#[cfg(feature = "serde")]
fn main() -> Result<()> {
    let model = E2KModel::from_file("tests/fixtures/sample.e2k")?;

    // Export to JSON file
    model.to_json_file("output/model.json")?;
    println!("Model exported to output/model.json");

    // Get JSON string
    let json = model.to_json()?;
    println!("\nJSON preview (first 500 chars):");
    println!("{}", &json[..500.min(json.len())]);

    Ok(())
}

#[cfg(not(feature = "serde"))]
fn main() {
    println!("Enable 'serde' feature to use JSON export:");
    println!("  cargo run --example export_json --features serde");
}