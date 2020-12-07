pub mod collection;
pub use collection::GetterCollection;

pub mod doc_code;
pub use doc_code::DocCodeGetterCollector;

pub mod syntax_tree;
pub use syntax_tree::SyntaxTreeGetterCollector;

pub mod token_stream;
pub use token_stream::TokenStreamGetterCollector;
