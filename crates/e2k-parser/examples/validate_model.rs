use e2k_parser::{E2KModel, Result};

fn main() -> Result<()> {
    let model = E2KModel::from_file("tests/fixtures/sample.e2k")?;

    println!("Validating model...\n");

    match model.validate() {
        Ok(report) => {
            println!("✓ Model validation PASSED");
            if report.warning_count() > 0 {
                println!("\nWarnings:");
                for warning in report.warnings() {
                    println!("  ⚠  {}", warning);
                }
            }
        }
        Err(e) => {
            println!("✗ Model validation FAILED");
            println!("Error: {}", e);
        }
    }

    Ok(())
}
