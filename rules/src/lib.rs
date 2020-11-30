//! Rules definition for getter renaming and Rust crate / workspace directory filtering.
//!
//! # Features
//!
//! - **`dir-entry`** *(enabled by default)* — Directory entry filtering.

#[cfg(feature = "dir-entry")]
pub mod dir_entry;

pub mod function;
pub use function::{try_rename_getter, NewName, RenameError, ReturnsBool};
