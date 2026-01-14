use crate::{types::*, error::Result};
mod prelude;
mod primitives;
mod geometry;
mod loading;
mod analysis;
mod structural;

/// Public entry point
pub fn parse_e2k(content: &str) -> Result<E2KModel> {
    // Orchestrate section parsers
    // Build E2KModel
    unimplemented!()
}