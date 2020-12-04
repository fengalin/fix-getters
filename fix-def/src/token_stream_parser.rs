//! Macro parser in search of renamable getter definitions.

use proc_macro2::{Delimiter, TokenStream, TokenTree};
use std::path::Path;
use syn::buffer::{Cursor, TokenBuffer};

use rules::ReturnsBool;
use utils::{getter, parser::prelude::*, NonGetterReason, Scope};

use crate::{GetterDef, GetterDefCollection};

#[derive(Debug)]
pub struct TSGetterDefParser<'scope> {
    state: State,
    getter_collection: GetterDefCollection,
    path: &'scope Path,
    scope: &'scope Scope,
}

impl<'scope> TokenStreamParser for TSGetterDefParser<'scope> {
    type GetterCollection = GetterDefCollection;

    fn parse(
        path: &Path,
        scope: &Scope,
        stream: &TokenStream,
        getter_collection: &GetterDefCollection,
    ) {
        let mut parser = TSGetterDefParser {
            state: State::default(),
            getter_collection: GetterDefCollection::clone(getter_collection),
            path,
            scope,
        };
        let token_buf = TokenBuffer::new2(stream.clone());
        parser.parse_(token_buf.begin());
    }
}

impl<'scope> TSGetterDefParser<'scope> {
    fn parse_(&mut self, mut rest: Cursor) {
        use NonGetterReason::*;

        while let Some((tt, next)) = rest.token_tree() {
            // Find patterns `.get_suffix()`
            match tt {
                TokenTree::Punct(punct) => {
                    let char_ = punct.as_char();
                    match self.state.take() {
                        State::MaybeGetterArgList(getter) => {
                            if char_ == '&' {
                                self.state = State::MaybeGetterRef(getter);
                            } else {
                                getter::skip(self.scope, getter.name(), &NotAMethod, getter.line());
                            }
                        }
                        State::MaybeGetterSelf(getter) => match char_ {
                            '-' => self.state = State::MaybeGetterRet(getter),
                            ',' => {
                                getter::skip(
                                    self.scope,
                                    getter.name(),
                                    &MultipleArgs,
                                    getter.line(),
                                );
                            }
                            _ => (),
                        },
                        State::MaybeGetterRet(getter) => {
                            match char_ {
                                '>' | '&' => self.state = State::MaybeGetterRet(getter),
                                '$' => {
                                    // Return type is a macro argument
                                    self.getter_collection.add(getter);
                                }
                                _ => (),
                            }
                        }
                        State::MaybeGetter(getter) => {
                            if char_ == '<' {
                                getter::skip(
                                    self.scope,
                                    getter.name(),
                                    &GenericTypeParam,
                                    getter.line(),
                                );
                            }
                        }
                        _ => (),
                    }
                }
                TokenTree::Ident(ident) => match self.state.take() {
                    State::None => {
                        if ident == "fn" {
                            self.state = State::Fn;
                        }
                    }
                    State::Fn => {
                        let res = self.getter_collection.try_new_getter(
                            ident.to_string(),
                            ReturnsBool::Maybe,
                            ident.span().start().line,
                            // easier to add all doc aliases everywhere
                            // than parsing scopes in macros
                            true,
                        );
                        match res {
                            Ok(getter) => {
                                // Will log when the getter is confirmed
                                self.state = State::MaybeGetter(getter)
                            }
                            Err(err) => err.log(self.scope),
                        }
                    }
                    State::MaybeGetterRef(getter) => {
                        if ident == "self" {
                            self.state = State::MaybeGetterSelf(getter);
                        } else if ident == "mut" {
                            self.state = State::MaybeGetterRef(getter);
                        } else {
                            getter::skip(self.scope, getter.name(), &NotAMethod, getter.line());
                        }
                    }
                    State::MaybeGetterRet(mut getter) => {
                        getter.set_returns_bool(ident == "bool");
                        getter.log(self.path, self.scope);
                        self.getter_collection.add(getter);
                    }
                    State::MaybeGetterSelf(getter) => {
                        getter::skip(self.scope, getter.name(), &NonSelfUniqueArg, getter.line());
                    }
                    State::MaybeGetterArgList(getter) => {
                        if ident != "self" {
                            getter::skip(self.scope, getter.name(), &NotAMethod, getter.line());
                        }
                        // else is unlikely: a getter consuming self
                    }
                    _ => (),
                },
                TokenTree::Group(group) => {
                    match self.state.take() {
                        State::MaybeGetterRet(mut getter) => {
                            // Returning complexe type
                            getter.set_returns_bool(false);
                            self.getter_collection.add(getter);
                        }
                        State::MaybeGetter(getter) => {
                            if group.delimiter() == Delimiter::Parenthesis {
                                if !group.stream().is_empty() {
                                    self.state = State::MaybeGetterArgList(getter);
                                } else {
                                    getter::skip(self.scope, getter.name(), &NoArgs, getter.line());
                                }
                            }
                        }
                        _ => (),
                    }

                    if !group.stream().is_empty() {
                        let token_buf = TokenBuffer::new2(group.stream());
                        self.parse_(token_buf.begin());
                    }
                }
                TokenTree::Literal(_) => self.state = State::None,
            }

            rest = next;
        }
    }
}

#[derive(Debug)]
enum State {
    None,
    Fn,
    MaybeGetter(GetterDef),
    MaybeGetterArgList(GetterDef),
    MaybeGetterRef(GetterDef),
    MaybeGetterSelf(GetterDef),
    MaybeGetterRet(GetterDef),
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
