//! A generic documentation code [`Getter`](crate::Getter) parser.

use std::path::{Path, PathBuf};

use crate::{GetterCollection, Scope, TokenStreamParser};

/// A generic documentation code [`Getter`](crate::Getter) parser.
#[derive(Debug)]
pub struct DocCodeParser<P: TokenStreamParser> {
    code: String,
    state: State,
    getter_collection: P::GetterCollection,
    path: PathBuf,
}

impl<P: TokenStreamParser> DocCodeParser<P> {
    /// Builds a [`DocCodeParser`].
    ///
    /// [`Getter`](crate::Getter)s will be added to the provided [`GetterCollection`].
    /// Documentation alias attributes will be discarded.
    pub fn new(path: &Path, getter_collection: &P::GetterCollection) -> Self {
        let mut getter_collection = P::GetterCollection::clone(getter_collection);
        getter_collection.disable_doc_alias();

        DocCodeParser {
            code: String::with_capacity(512),
            state: State::None,
            getter_collection,
            path: path.to_owned(),
        }
    }

    /// Analyses the documentation in the provided [`Attribute`](syn::Attribute).
    ///
    /// Note that documentation code is parsed by [`syn`] one line at a time,
    /// this method will take care of parsing any code found in the provided
    /// [`Attribute`](syn::Attribute)s and feed the [`GetterCollection`].
    pub fn have_attribute(&mut self, node: &syn::Attribute) {
        if let Some((_, cursor)) = syn::buffer::TokenBuffer::new2(node.tokens.clone())
            .begin()
            .punct()
        {
            if let Some((literal, _)) = cursor.literal() {
                self.process(
                    &literal.to_string().trim_matches('"').trim(),
                    literal.span().start().line,
                );
            }
        }
    }

    fn process(&mut self, doc_line: &str, offset: usize) {
        if doc_line.starts_with("```") {
            if !self.state.is_code_block() {
                // starting a doc code block
                self.getter_collection.set_offset(offset);
                if doc_line.len() == 3 || doc_line.ends_with("rust") {
                    self.state = State::RustCodeBlock;
                } else {
                    self.state = State::CodeBlock;
                };
            } else {
                // terminating a doc code block
                if self.state.is_rust() {
                    self.parse();
                }
                self.state = State::None;
            }
        } else if self.state.is_rust() && !doc_line.starts_with('#') {
            self.code.push_str(&doc_line.replace('\\', &""));
            self.code.push('\n');
        }
    }

    fn parse(&mut self) {
        match syn::parse_str::<proc_macro2::TokenStream>(&self.code) {
            Ok(syntax_tree) => P::parse(
                &self.path,
                &Scope::Documentation,
                &syntax_tree,
                &self.getter_collection,
            ),
            Err(_err) => {
                #[cfg(feature = "log")]
                log::warn!(
                    "{:?} doc @ {}: {:?}",
                    self.path,
                    self.getter_collection.offset(),
                    _err
                );
            }
        }

        self.code.clear();
    }
}

#[derive(Debug)]
enum State {
    None,
    CodeBlock,
    RustCodeBlock,
}

impl State {
    fn is_code_block(&self) -> bool {
        matches!(self, State::RustCodeBlock | State::CodeBlock)
    }

    fn is_rust(&self) -> bool {
        matches!(self, State::RustCodeBlock)
    }
}
