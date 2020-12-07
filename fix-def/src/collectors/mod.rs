pub mod collection;
pub use collection::GetterDefCollection;

pub mod syntax_tree;
pub use syntax_tree::STGetterDefCollector;

pub mod token_stream;
pub use token_stream::TSGetterDefCollector;
