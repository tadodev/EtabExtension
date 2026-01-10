// ============================================================================
// Error Handling
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
pub enum E2KError {
    ParseError(String),
    InvalidSection(String),
    MissingRequiredField(String),
    InvalidValue(String),
}

impl std::fmt::Display for E2KError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            E2KError::ParseError(msg) => write!(f, "Parse error: {}", msg),
            E2KError::InvalidSection(section) => write!(f, "Invalid section: {}", section),
            E2KError::MissingRequiredField(field) => write!(f, "Missing required field: {}", field),
            E2KError::InvalidValue(msg) => write!(f, "Invalid value: {}", msg),
        }
    }
}

impl std::error::Error for E2KError {}