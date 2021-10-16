//! Library for identifying and converting identifiers naming format (case | notation).
//!
//! It serves three purposes:
//!
//! 1. Judge if an identifier is written in a certain format.
//! (example: [is_camel()](crate::detector::is_camel()))
//!
//! 2. Automatically identify format with [which_case()](crate::detector::which_case()).
//!
//! 3. Convert identifiers between different naming formats.
//! (example: [to_camel()](NamingCase::to_camel()))
pub use detector::*;
pub use naming_case::*;

mod naming_case;
mod detector;