//! Utilities for getter renaming.
//!
//! # Features
//!
//! - **`fs`** *(enabled by default)* — File system traversal helper.
//! - **`log`** *(enabled by default)* — Logging via the `log` crate.
//! - **`parser`** *(enabled by default)* — Rust code parser related utilities.
//!   This features enables both **`parser-error`** & **`scope`**.
//! - **`parser-error`** *(enabled by default)* — Rust code parser error wrapper.
//! - **`scope`** *(enabled by default)* — Rust code `Scope` identification.

pub mod error;
pub use error::Error;
#[cfg(feature = "parser-error")]
pub use error::ParseFileError;

#[cfg(feature = "fs")]
pub mod fs;

pub mod getter;
pub use getter::{Getter, GetterError, NonGetterReason};

#[cfg(feature = "scope")]
pub mod scope;
#[cfg(feature = "scope")]
pub use scope::Scope;
