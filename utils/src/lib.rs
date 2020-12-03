//! Utilities for getter renaming.
//!
//! # Features
//!
//! - **`log`** *(enabled by default)* — Logging via the `log` crate.

pub mod doc_code_parser;
pub use doc_code_parser::DocCodeParser;

pub mod error;
pub use error::Error;
pub use error::ParseFileError;

pub mod fs;

pub mod getter;
pub use getter::{Getter, GetterError, NonGetterReason};

pub mod getter_collection;
pub use getter_collection::GetterCollection;

pub mod getter_visitor;
pub use getter_visitor::GetterVisitor;

pub mod scope;
pub use scope::Scope;

pub mod token_stream_parser;
pub use token_stream_parser::TokenStreamParser;

pub mod parser {
    pub mod prelude {
        pub use super::super::{GetterCollection, GetterVisitor, TokenStreamParser};
    }
}
