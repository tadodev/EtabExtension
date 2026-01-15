//! Error types for E2K Parser library.
//!
//! This module defines all error types used throughout the E2K parsing
//! and processing pipeline.
//!
//! Error handling philosophy:
//!
//! **System errors MUST bubble up unchanged**
//! - Io: File access, permissions, missing files
//! These indicate real system problems.
//!
//! **Application errors carry domain context**
//! - Parsing: Invalid or corrupted E2K format
//! - Validation: Logical or structural model inconsistencies
//! - Serialization: Export / import errors
//! - UnsupportedFormat - Unknown E2K sections or versions
//!
//! The goal is:
//! - Keep error chains intact
//! - Provide meaningful messages for users
//! - Keep debugging straightforward

use thiserror::Error;
use crate::PanicContext;

/// Standard Result type for the E2K library
pub type Result<T> = std::result::Result<T, E2kError>;

/// Main error type for all E2K operations
#[derive(Debug, Error)]
pub enum E2kError {
    /// File system and I/O errors (bubble up unchanged)
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// Encoding errors (critical for Windows-1251 legacy files)
    #[error("Encoding error: {message}")]
    Encoding {
        message: String,
        #[source]
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
    },

    /// Parsing errors (nom failures, corrupted E2K)
    #[error("Parse error at line {line}: {message}")]
    Parsing {
        line: usize,
        message: String,
        #[source]
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
    },

    /// Unsupported or unknown E2K section
    #[error("Unsupported E2K section: {0}")]
    UnsupportedSection(String),

    /// Domain validation errors (model consistency issues)
    #[error("Validation error: {message}")]
    Validation {
        message: String,
        #[source]
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
    },

    /// Serialization errors (JSON)
    #[error("Serialization error: {message}")]
    Serialization {
        message: String,
        #[source]
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
    },

    /// Unsupported or unknown E2K format / version / section
    #[error("Unsupported format: {0}")]
    UnsupportedFormat(String),

    /// Catch-all for unexpected errors
    #[error("Internal error: {0}")]
    Other(String),
}

// ===== Conversions =====
impl From<std::string::FromUtf8Error> for E2kError {
    fn from(err: std::string::FromUtf8Error) -> Self {
        E2kError::Encoding {
            message: err.to_string(),
            source: Some(Box::new(err)),
        }
    }
}

impl From<PanicContext> for E2kError {
    fn from(ctx: PanicContext) -> Self {
        E2kError::Other(ctx.format())
    }
}
// Handle Nom errors by converting them to owned strings immediately.
// This prevents lifetime issues when passing errors up the stack.
impl<'a> From<nom::Err<nom::error::Error<&'a str>>> for E2kError {
    fn from(err: nom::Err<nom::error::Error<&'a str>>) -> Self {
        E2kError::Parsing {
            line: 0, // Line number usually populated by the specific parser context later
            message: format!("Nom parsing error: {:?}", err),
            source: None,
        }
    }
}

// ===== Error Constructors =====

impl E2kError {
    /// Create a parsing error.
    /// Use this when you know the line number (e.g. inside the loop).
    pub fn parsing(line: usize, message: impl Into<String>) -> Self {
        Self::Parsing {
            line,
            message: message.into(),
            source: None,
        }
    }

    /// Create a general parsing error (line 0/unknown).
    pub fn parsing_general(message: impl Into<String>) -> Self {
        Self::Parsing {
            line: 0,
            message: message.into(),
            source: None,
        }
    }

    pub fn validation(message: impl Into<String>) -> Self {
        Self::Validation {
            message: message.into(),
            source: None,
        }
    }

    pub fn serialization(message: impl Into<String>) -> Self {
        Self::Serialization {
            message: message.into(),
            source: None,
        }
    }

    pub fn other(message: impl Into<String>) -> Self {
        Self::Other(message.into())
    }
}

// ===== Tests =====

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_io_error_from() {
        let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "file not found");
        let e2k_err: E2kError = io_err.into();
        assert!(matches!(e2k_err, E2kError::Io(_)));
    }

    #[test]
    fn test_parsing_error_display() {
        let err = E2kError::parsing(42, "Missing delimiter");
        assert_eq!(err.to_string(), "Parse error at line 42: Missing delimiter");
    }

    #[test]
    fn test_validation_error() {
        let err = E2kError::validation("Point references invalid frame");
        assert_eq!(err.to_string(), "Validation error: Point references invalid frame");
    }

    #[test]
    fn test_nom_error_conversion() {
        // Simulate a Nom error
        let nom_err: nom::Err<nom::error::Error<&str>> =
            nom::Err::Error(nom::error::Error::new("BAD_INPUT", nom::error::ErrorKind::Tag));

        let e2k_err: E2kError = nom_err.into();

        if let E2kError::Parsing { message, .. } = e2k_err {
            assert!(message.contains("Nom parsing error"));
            assert!(message.contains("Tag"));
        } else {
            panic!("Expected Parsing error variant");
        }
    }
}