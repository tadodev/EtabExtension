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


// Re-export parser entry function(s)
pub use parser::parse_e2k;


pub type Result<T> = std::result::Result<T, E2kError>;
