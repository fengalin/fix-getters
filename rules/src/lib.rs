//! Rules definition for getter renaming and Rust crate / workspace directory filtering.
//!
//! # Features
//!
//! - **`dir-entry`** *(enabled by default)* — Directory entry filtering.

#[cfg(feature = "dir-entry")]
pub mod dir_entry;

pub mod function;
pub use function::{try_rename_getter_suffix, try_rename_would_be_getter, RenameError};

pub mod new_name;
pub use new_name::{NewName, NewNameRule, ReturnsBool};
