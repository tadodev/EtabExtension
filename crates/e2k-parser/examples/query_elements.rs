use e2k_parser::{E2KModel, Result};

fn main() -> Result<()> {
    let model = E2KModel::from_file("crates/e2k-parser/tests/fixtures/sample.e2k")?;

    // Query columns
    let columns = model.get_columns();
    println!("Found {} columns:", columns.len());
    for col in columns.iter().take(5) {
        println!("  Column {} ({} -> {})", col.id, col.point_i, col.point_j);
    }

    // Query beams
    let beams = model.get_beams();
    println!("\nFound {} beams:", beams.len());
    for beam in beams.iter().take(5) {
        println!("  Beam {} ({} -> {})", beam.id, beam.point_i, beam.point_j);
    }

    // Find specific story
    if let Some(story) = model.get_story("L01") {
        println!("\nStory L01:");
        println!("  Height: {:?}", story.height);
        println!("  Elevation: {:?}", story.elevation);
    }

    // Find specific material
    if let Some(material) = model.get_material("4000Psi") {
        println!("\nMaterial 4000Psi:");
        println!("  Type: {}", material.material_type);
        println!("  Grade: {:?}", material.grade);
    }

    Ok(())
}