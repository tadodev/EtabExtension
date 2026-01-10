impl E2KParser {
    /// Parse from file path
    pub fn from_file(path: &str) -> Result<E2KModel, String> {
        let content = std::fs::read_to_string(path)
            .map_err(|e| format!("Failed to read file: {}", e))?;
        let parser = E2KParser::new(content);
        parser.parse()
    }

    /// Validate the parsed model
    pub fn validate(model: &E2KModel) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();

        // Check for required sections
        if model.stories.is_empty() {
            errors.push("Model has no stories defined".to_string());
        }

        if model.points.is_empty() {
            errors.push("Model has no points defined".to_string());
        }

        // Validate point references in lines
        for line in &model.lines {
            if !model.points.iter().any(|p| p.id == line.point_i) {
                errors.push(format!("Line {} references undefined point {}", line.id, line.point_i));
            }
            if !model.points.iter().any(|p| p.id == line.point_j) {
                errors.push(format!("Line {} references undefined point {}", line.id, line.point_j));
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "serde")]
impl E2KModel {
    /// Convert to JSON string
    pub fn to_json(&self) -> Result<String, String> {
        serde_json::to_string_pretty(self)
            .map_err(|e| format!("Failed to serialize to JSON: {}", e))
    }

    /// Save to JSON file
    pub fn save_json(&self, path: &str) -> Result<(), String> {
        let json = self.to_json()?;
        std::fs::write(path, json)
            .map_err(|e| format!("Failed to write JSON file: {}", e))
    }
}