//! E2K Parser public API

#![deny(unsafe_code)]
pub mod types;
pub mod parser;
pub mod error;
pub mod panic_context;


// Re-export types so users can do: use e2k_parser::E2KModel;
pub use types::*;
// Re-export error type
pub use error::E2kError;
pub use panic_context::PanicContext;
pub use types::{E2KModel, ModelStatistics, ValidationReport};

// Re-export parser entry function(s)
pub use parser::parse_e2k;


pub type Result<T> = std::result::Result<T, E2kError>;

// Convenience API
impl E2KModel {
    /// Parse E2K file from path
    pub fn from_file(path: &str) -> Result<Self> {
        let content = std::fs::read_to_string(path)
            .map_err(|e| E2kError::Io(e))?;
        parser::parse_e2k(&content)
    }

    /// Parse E2K from string content
    pub fn from_str(content: &str) -> Result<Self> {
        parser::parse_e2k(content)
    }

    /// Export to JSON
    #[cfg(feature = "serde")]
    pub fn to_json(&self) -> Result<String> {
        serde_json::to_string_pretty(self)
            .map_err(|e| E2kError::from(e))
    }

    /// Export to JSON file
    #[cfg(feature = "serde")]
    pub fn to_json_file(&self, path: &str) -> Result<()> {
        let json = self.to_json()?;
        std::fs::write(path, json)
            .map_err(|e| E2kError::Io(e))
    }
}