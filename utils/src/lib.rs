//! Utilities for getter renaming.
//!
//! # Features
//!
//! - **`log`** *(enabled by default)* — Logging via the `log` crate.

pub mod error;
pub use error::Error;
pub use error::ParseFileError;

pub mod crate_traverser;
pub use crate_traverser::CrateTraverser;

pub mod getter;
pub use getter::{Getter, GetterError, NonGetterReason};

pub mod collectors;
pub use collectors::*;

pub mod scope;
pub use scope::Scope;

pub mod prelude {
    pub use super::{
        CrateTraverser, GetterCollection, SyntaxTreeGetterCollector, TokenStreamGetterCollector,
    };
}
