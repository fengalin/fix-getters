//! A [`TokenStreamGetterCollector`](utils::TokenStreamGetterCollector) collecting
//! renamable [`Getter`](utils::Getter) calls.

use log::trace;
use proc_macro2::{Delimiter, TokenStream, TokenTree};
use std::path::Path;
use syn::buffer::{Cursor, TokenBuffer};

use rules::ReturnsBool;
use utils::{getter, prelude::*, Getter, NonGetterReason, Scope};

use crate::GetterCallCollection;

/// A [`TokenStreamGetterCollector`](utils::TokenStreamGetterCollector) collecting
/// renamable [`Getter`](utils::Getter) calls.
#[derive(Debug)]
pub struct TsGetterCallCollector<'scope> {
    state: State,
    getter_collection: GetterCallCollection,
    path: &'scope Path,
    identification_mode: IdentificationMode,
    scope: &'scope Scope,
}

impl<'scope> TokenStreamGetterCollector for TsGetterCallCollector<'scope> {
    type GetterCollection = GetterCallCollection;

    fn collect(
        path: &Path,
        scope: &Scope,
        stream: &TokenStream,
        identification_mode: IdentificationMode,
        getter_collection: &GetterCallCollection,
    ) {
        let mut parser = TsGetterCallCollector {
            state: State::default(),
            getter_collection: GetterCallCollection::clone(getter_collection),
            identification_mode,
            path,
            scope,
        };
        let token_buf = TokenBuffer::new2(stream.clone());
        parser.parse_(token_buf.begin());
    }
}

impl<'scope> TsGetterCallCollector<'scope> {
    fn parse_(&mut self, mut rest: Cursor) {
        while let Some((tt, next)) = rest.token_tree() {
            // Find patterns `.get_suffix()`
            match tt {
                TokenTree::Punct(punct) => {
                    let char_ = punct.as_char();
                    match char_ {
                        ';' | ',' | '=' | '{' | '}' | '+' | '-' | '/' | '|' => {
                            // Forget current would be function call, if any.
                            self.state = State::None;
                            rest = next;
                            continue;
                        }
                        _ => (),
                    }

                    match self.state.take() {
                        State::None => {
                            if char_ == '.' {
                                self.state = State::Dot;
                            }
                        }
                        State::Dot => (),
                        State::MaybeNamedFn(maybe) => {
                            if let ':' | '<' = char_ {
                                self.state = State::ParamList(maybe);
                            }
                        }
                        State::ParamList(maybe) => {
                            if char_ == '\'' {
                                self.state = State::ParamLt(maybe);
                            } else {
                                self.state = State::ParamList(maybe);
                            }
                        }
                        State::ParamLt(maybe) => {
                            // unexpected
                            trace!("ts {:?} ParamLt({:?})", punct, maybe);
                        }
                    }
                }
                TokenTree::Ident(ident) => {
                    match self.state.take() {
                        State::None => {
                            self.state = self.try_new_maybe_named_fn(
                                &ident, false, // not a method
                            );
                        }
                        State::Dot => {
                            self.state = self.try_new_maybe_named_fn(
                                &ident, true, // maybe a method
                            );
                        }
                        State::MaybeNamedFn(_) => (),
                        State::ParamList(mut maybe) => {
                            maybe.has_gen_params = true;
                            self.state = State::ParamList(maybe);
                        }
                        State::ParamLt(maybe) => {
                            self.state = State::ParamList(maybe);
                        }
                    }
                }
                TokenTree::Group(group) => {
                    if let State::MaybeNamedFn(mut maybe) | State::ParamList(mut maybe) =
                        self.state.take()
                    {
                        if let Delimiter::Parenthesis = group.delimiter() {
                            if group.stream().is_empty() {
                                // found `()` after a getter call
                                self.process_maybe_getter(maybe);
                            } else {
                                maybe.has_multiple_args = true;
                                self.process_maybe_getter(maybe);
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

    fn process_maybe_getter(&mut self, maybe: MaybeGetter) {
        use NonGetterReason::*;

        if !maybe.getter.returns_bool().is_true() && self.identification_mode.is_conservative() {
            // not a bool getter
            if maybe.has_no_args {
                getter::skip(self.scope, &maybe.getter.name, &NoArgs, maybe.getter.line);
                return;
            }
            if !maybe.is_method {
                getter::skip(
                    self.scope,
                    &maybe.getter.name,
                    &NotAMethod,
                    maybe.getter.line,
                );
                return;
            }
            if maybe.has_gen_params {
                getter::skip(
                    self.scope,
                    &maybe.getter.name,
                    &GenericTypeParam,
                    maybe.getter.line,
                );
                return;
            }
            if maybe.has_multiple_args {
                getter::skip(
                    self.scope,
                    &maybe.getter.name,
                    &MultipleArgs,
                    maybe.getter.line,
                );
                return;
            }
        }

        maybe.getter.log(self.path, self.scope);
        self.getter_collection.add(maybe.getter);
    }

    fn try_new_maybe_named_fn(&mut self, ident: &syn::Ident, is_method: bool) -> State {
        let res = self.getter_collection.try_new_getter(
            ident.to_string(),
            ReturnsBool::Maybe,
            ident.span().start().line,
        );
        match res {
            Ok(getter) => State::MaybeNamedFn(MaybeGetter {
                getter,
                has_gen_params: false,
                is_method,
                has_multiple_args: false,
                has_no_args: false,
            }),
            Err(err) => {
                if is_method {
                    err.log(self.scope);
                }
                State::None
            }
        }
    }
}

#[derive(Debug)]
struct MaybeGetter {
    getter: Getter,
    has_gen_params: bool,
    is_method: bool,
    has_multiple_args: bool,
    has_no_args: bool,
}

#[derive(Debug)]
enum State {
    None,
    Dot,
    MaybeNamedFn(MaybeGetter),
    ParamList(MaybeGetter),
    ParamLt(MaybeGetter),
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
