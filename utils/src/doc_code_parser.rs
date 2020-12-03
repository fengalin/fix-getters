//! A generic documentation code [`Getter`](crate::Getter) parser.

use std::{cell::RefCell, rc::Rc};

use crate::{GetterCollection, Scope, TokenStreamParser};

/// A generic documentation code [`Getter`](crate::Getter) parser.
#[derive(Debug)]
pub struct DocCodeParser<P: TokenStreamParser> {
    code: String,
    is_in_code_block: bool,
    ignore_code_block: bool,
    getter_collection: P::GetterCollection,
}

impl<P: TokenStreamParser> DocCodeParser<P> {
    /// Builds a [`DocCodeParser`].
    ///
    /// [`Getter`](crate::Getter)s will be added to the provided [`GetterCollection`].
    /// Documentation alias attributes will be discarded.
    pub fn new(getter_collection: &P::GetterCollection) -> Self {
        let mut getter_collection = P::GetterCollection::clone(getter_collection);
        getter_collection.disable_doc_alias();

        DocCodeParser {
            code: String::with_capacity(512),
            is_in_code_block: false,
            ignore_code_block: false,
            getter_collection,
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
                    &literal.to_string().trim_matches('"').trim_start(),
                    literal.span().start().line,
                );
            }
        }
    }

    fn process(&mut self, doc_line: &str, offset: usize) {
        if doc_line.starts_with("```") {
            if !self.is_in_code_block {
                // starting a doc code block
                self.getter_collection.set_offset(offset);
                self.is_in_code_block = true;
                self.ignore_code_block =
                    doc_line.find("ignore").is_some() || doc_line.find('C').is_some();
            } else {
                // terminating a doc code block
                if !self.ignore_code_block {
                    self.parse();
                }
                self.is_in_code_block = false;
                self.ignore_code_block = false;
            }
        } else if !self.ignore_code_block && self.is_in_code_block && !doc_line.starts_with('#') {
            self.code.push_str(&doc_line.replace('\\', &""));
            self.code.push('\n');
        }
    }

    fn parse(&mut self) {
        match syn::parse_str::<proc_macro2::TokenStream>(&self.code) {
            Ok(syntax_tree) => P::parse(
                &syntax_tree,
                &Rc::new(RefCell::new(Scope::Attribute("doc".to_string()))),
                &self.getter_collection,
            ),
            Err(_err) => {
                #[cfg(feature = "log")]
                log::warn!("Doc @ {}: {:?}", self.getter_collection.offset(), _err);
            }
        }

        self.code.clear();
    }
}
