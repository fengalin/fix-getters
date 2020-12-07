//! [`Getter`](utils::Getter) call sites collection implementations.

pub mod collection;
pub use collection::GetterCallCollection;

pub mod syntax_tree;
pub use syntax_tree::STGetterCallCollector;

pub mod token_stream;
pub use token_stream::TSGetterCallCollector;
