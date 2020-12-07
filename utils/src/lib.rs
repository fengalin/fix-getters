//! Utilities for getter renaming.
//!
//! # Features
//!
//! - **`log`** *(enabled by default)* — Logging via the `log` crate.

pub mod error;
pub use error::Error;
pub use error::ParseFileError;

pub mod fs;

pub mod getter;
pub use getter::{Getter, GetterError, NonGetterReason};

pub mod collectors;
pub use collectors::*;

pub mod scope;
pub use scope::Scope;

pub mod parser {
    pub mod prelude {
        pub use super::super::{GetterCollection, GetterVisitor, TokenStreamParser};
    }
}
