use e2k_parser::{E2KModel, Result};

fn main() -> Result<()> {
    // Parse from file
    let model = E2KModel::from_file("crates/e2k-parser/tests/fixtures/sample.e2k")?;

    // Print statistics
    let stats = model.get_statistics();
    println!("{}", stats);

    // Access specific elements
    println!("\n=== Stories ===");
    for story in &model.stories {
        println!("  {}: height = {:?}", story.name, story.height);
    }

    println!("\n=== Materials ===");
    for material in &model.materials {
        println!("  {} ({})", material.name, material.material_type);
    }

    println!("\n=== Load Patterns ===");
    for pattern in &model.load_patterns {
        println!("  {} - {} (SW: {})",
                 pattern.name, pattern.load_type, pattern.self_weight);
    }

    Ok(())
}