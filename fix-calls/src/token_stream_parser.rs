//! Macro parser in search of renamable getter calls.

use proc_macro2::{Delimiter, TokenStream, TokenTree};
use std::{cell::RefCell, rc::Rc};
use syn::buffer::{Cursor, TokenBuffer};

use rules::ReturnsBool;
use utils::{getter, parser::prelude::*, Getter, NonGetterReason, Scope};

use crate::GetterCallCollection;

#[derive(Debug)]
pub struct TSGetterCallParser {
    state: State,
    getter_collection: GetterCallCollection,
    scope: Rc<RefCell<Scope>>,
}

impl TokenStreamParser for TSGetterCallParser {
    type GetterCollection = GetterCallCollection;

    fn parse(
        stream: &TokenStream,
        scope: &Rc<RefCell<Scope>>,
        getter_collection: &GetterCallCollection,
    ) {
        let mut parser = TSGetterCallParser {
            state: State::default(),
            getter_collection: GetterCallCollection::clone(getter_collection),
            scope: Rc::clone(&scope),
        };
        let token_buf = TokenBuffer::new2(stream.clone());
        parser.parse_(token_buf.begin());
    }
}

impl TSGetterCallParser {
    fn parse_(&mut self, mut rest: Cursor) {
        use NonGetterReason::*;

        while let Some((tt, next)) = rest.token_tree() {
            // Find patterns `.get_suffix()`
            match tt {
                TokenTree::Punct(punct) => {
                    let char_ = punct.as_char();
                    match char_ {
                        '.' => self.state = State::Dot,
                        ':' | '<' => {
                            if let State::MaybeGetter(getter) = self.state.take() {
                                getter::skip(
                                    &self.scope.borrow(),
                                    &getter.name,
                                    &GenericTypeParam,
                                    getter.line,
                                );
                            }
                        }
                        _ => self.state = State::None,
                    }
                }
                TokenTree::Ident(ident) => {
                    if let State::Dot = self.state.take() {
                        let res = self.getter_collection.try_new_getter(
                            ident.to_string(),
                            ReturnsBool::Maybe,
                            ident.span().start().line,
                        );
                        match res {
                            Ok(getter) => {
                                // Will log when the getter is confirmed
                                self.state = State::MaybeGetter(getter)
                            }
                            Err(err) => err.log(&self.scope.borrow()),
                        }
                    }
                }
                TokenTree::Group(group) => {
                    if let State::MaybeGetter(getter) = self.state.take() {
                        if let Delimiter::Parenthesis = group.delimiter() {
                            if group.stream().is_empty() {
                                // found `()` after a getter call
                                getter.log(&self.scope.borrow());
                                self.getter_collection.add(getter);
                            } else {
                                getter::skip(
                                    &self.scope.borrow(),
                                    &getter.name,
                                    &MultipleArgs,
                                    getter.line,
                                );
                            }
                        }
                    }

                    if !group.stream().is_empty() {
                        let token_buf = TokenBuffer::new2(group.stream());
                        self.parse_(token_buf.begin());
                    }
                }
                TokenTree::Literal(_) => {
                    self.state = State::None;
                }
            }

            rest = next;
        }
    }
}

#[derive(Debug)]
enum State {
    None,
    Dot,
    MaybeGetter(Getter),
}

impl State {
    /// Returns current state replacing `self` with the default value.
    fn take(&mut self) -> Self {
        std::mem::replace(self, State::None)
    }
}

impl Default for State {
    fn default() -> Self {
        State::None
    }
}
