//! A [`TokenStreamGetterCollector`](utils::TokenStreamGetterCollector) collecting
//! renamable [`Getter`](utils::Getter) definitions as [`GetterDef`](crate::GetterDef).

use log::trace;
use proc_macro2::{Delimiter, TokenStream, TokenTree};
use std::path::Path;
use syn::buffer::{Cursor, TokenBuffer};

use rules::ReturnsBool;
use utils::{getter, prelude::*, NonGetterReason, Scope};

use crate::{GetterDef, GetterDefCollection};

/// A [`TokenStreamGetterCollector`](utils::TokenStreamGetterCollector) collecting
/// renamable [`Getter`](utils::Getter) definitions as [`GetterDef`](crate::GetterDef).
#[derive(Debug)]
pub struct TSGetterDefCollector<'scope> {
    state: State,
    getter_collection: GetterDefCollection,
    path: &'scope Path,
    scope: &'scope Scope,
}

impl<'scope> TokenStreamGetterCollector for TSGetterDefCollector<'scope> {
    type GetterCollection = GetterDefCollection;

    fn collect(
        path: &Path,
        scope: &Scope,
        stream: &TokenStream,
        getter_collection: &GetterDefCollection,
    ) {
        let mut parser = TSGetterDefCollector {
            state: State::default(),
            getter_collection: GetterDefCollection::clone(getter_collection),
            path,
            scope,
        };
        let token_buf = TokenBuffer::new2(stream.clone());
        parser.parse_(token_buf.begin());
    }
}

impl<'scope> TSGetterDefCollector<'scope> {
    fn parse_(&mut self, mut rest: Cursor) {
        while let Some((tt, next)) = rest.token_tree() {
            // Find patterns `.get_suffix()`
            match tt {
                TokenTree::Punct(punct) => {
                    let char_ = punct.as_char();
                    if char_ == ';' {
                        if let Some(maybe) = self.state.take_maybe_getter() {
                            self.process_maybe_getter(maybe);
                        }
                        rest = next;
                        continue;
                    }

                    match self.state.take() {
                        State::None => (),
                        State::Fn => self.state = State::Fn,
                        State::NamedFn(maybe) => {
                            if char_ == '<' {
                                self.state = State::ParamList(maybe);
                            } else {
                                // unexpected
                                trace!("ts {:?} NamedFn({:?})", punct, maybe);
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
                        State::ArgList(mut maybe) => match char_ {
                            '&' => self.state = State::ArgRef(maybe),
                            '\'' => self.state = State::ArgRefLt(maybe),
                            ',' => {
                                maybe.has_multiple_args = true;
                                self.state = State::ArgList(maybe);
                            }
                            '-' => self.state = State::Ret(maybe),
                            _ => {
                                self.state = State::ArgList(maybe);
                            }
                        },
                        State::ArgRef(maybe) => {
                            if char_ == '\'' {
                                self.state = State::ArgRefLt(maybe);
                            } else {
                                self.state = State::ArgList(maybe);
                            }
                        }
                        State::ArgRefLt(maybe) => self.state = State::ArgRef(maybe),
                        State::ArgSelf(mut maybe) => match char_ {
                            '-' => self.state = State::Ret(maybe),
                            ',' => {
                                maybe.has_multiple_args = true;
                                self.state = State::ArgList(maybe);
                            }
                            _ => self.state = State::ArgList(maybe),
                        },
                        State::Ret(mut maybe) => match char_ {
                            '>' | '&' => self.state = State::Ret(maybe),
                            '\'' => self.state = State::RetLt(maybe),
                            _ => {
                                maybe.getter.set_returns_bool(false);
                                self.process_maybe_getter(maybe);
                            }
                        },
                        State::RetLt(maybe) => self.state = State::RetLt(maybe),
                    }
                }
                TokenTree::Ident(ident) => {
                    if ident == "fn" {
                        if let Some(maybe) = self.state.take_maybe_getter() {
                            self.process_maybe_getter(maybe);
                        }
                        self.state = State::Fn;
                        rest = next;
                        continue;
                    }

                    match self.state.take() {
                        State::None => (),
                        State::Fn => {
                            let res = self.getter_collection.try_new_getter(
                                ident.to_string(),
                                // Don't assume boolness before we actual know
                                // from the signature.
                                ReturnsBool::False,
                                ident.span().start().line,
                            );
                            match res {
                                Ok(getter) => {
                                    self.state = State::new_named_fn(getter);
                                }
                                Err(err) => err.log(self.scope),
                            }
                        }
                        State::NamedFn(maybe) => {
                            // unexpected
                            trace!("ts {:?} NamedFn({:?})", ident, maybe);
                        }
                        State::ParamList(mut maybe) => {
                            maybe.has_gen_params = true;
                            self.state = State::ParamList(maybe);
                        }
                        State::ParamLt(maybe) => self.state = State::ParamList(maybe),
                        State::ArgList(maybe) => {
                            // if the getter consumes self, keep the get prefix
                            // => don't consider it a method
                            self.state = State::ArgList(maybe);
                        }
                        State::ArgRefLt(maybe) => self.state = State::ArgRef(maybe),
                        State::ArgRef(mut maybe) => {
                            if ident == "self" {
                                maybe.is_method = true;
                                self.state = State::ArgSelf(maybe);
                            } else if ident == "mut" {
                                self.state = State::ArgRef(maybe);
                            }
                        }
                        State::ArgSelf(maybe) => {
                            // unexpected, but keep going
                            self.state = State::ArgList(maybe);
                        }
                        State::Ret(mut maybe) => {
                            if maybe.getter.returns_bool().is_false() && ident == "bool" {
                                // Set boolness as known so far
                                // will be cleared if prooved wrong later.
                                maybe.getter.set_returns_bool(true);
                                self.state = State::Ret(maybe);
                            } else {
                                // not returning exactly one bool
                                maybe.getter.set_returns_bool(false);
                                self.process_maybe_getter(maybe);
                            }
                        }
                        State::RetLt(maybe) => self.state = State::Ret(maybe),
                    }
                }
                TokenTree::Group(group) => {
                    match self.state.take() {
                        State::None => (),
                        State::Ret(mut maybe) => {
                            if let Delimiter::Brace = group.delimiter() {
                                // Implementation begins
                                self.process_maybe_getter(maybe);
                            } else {
                                // Returning complexe type
                                maybe.getter.set_returns_bool(false);
                                self.process_maybe_getter(maybe);
                            }
                        }
                        State::NamedFn(mut maybe) | State::ParamList(mut maybe) => {
                            if let Delimiter::Parenthesis = group.delimiter() {
                                if group.stream().is_empty() {
                                    // No args, but we still need to check boolness
                                    maybe.has_no_args = true;
                                }
                                self.state = State::ArgList(maybe);
                            }
                        }
                        other => trace!("ts {:?} {:?}", group, other),
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

    fn process_maybe_getter(&mut self, maybe: MaybeGetter) {
        use NonGetterReason::*;

        if !maybe.getter.returns_bool().is_true() {
            // not a bool getter
            if maybe.has_no_args {
                getter::skip(
                    self.scope,
                    maybe.getter.name(),
                    &NoArgs,
                    maybe.getter.line(),
                );
                return;
            }
            if !maybe.is_method {
                getter::skip(
                    self.scope,
                    maybe.getter.name(),
                    &NotAMethod,
                    maybe.getter.line(),
                );
                return;
            }
            if maybe.has_gen_params {
                getter::skip(
                    self.scope,
                    maybe.getter.name(),
                    &GenericTypeParam,
                    maybe.getter.line(),
                );
                return;
            }
            if maybe.has_multiple_args {
                getter::skip(
                    self.scope,
                    maybe.getter.name(),
                    &MultipleArgs,
                    maybe.getter.line(),
                );
                return;
            }
        }

        maybe.getter.log(self.path, self.scope);
        self.getter_collection.add(maybe.getter);
    }
}

#[derive(Debug)]
struct MaybeGetter {
    getter: GetterDef,
    has_gen_params: bool,
    is_method: bool,
    has_multiple_args: bool,
    has_no_args: bool,
}

#[derive(Debug)]
enum State {
    None,
    Fn,
    NamedFn(MaybeGetter),
    ParamList(MaybeGetter),
    ParamLt(MaybeGetter),
    ArgList(MaybeGetter),
    ArgRef(MaybeGetter),
    ArgRefLt(MaybeGetter),
    ArgSelf(MaybeGetter),
    Ret(MaybeGetter),
    RetLt(MaybeGetter),
}

impl State {
    fn new_named_fn(getter: GetterDef) -> Self {
        State::NamedFn(MaybeGetter {
            getter,
            has_gen_params: false,
            is_method: false,
            has_multiple_args: false,
            has_no_args: false,
        })
    }

    /// Returns current state replacing `self` with the default value.
    fn take(&mut self) -> Self {
        std::mem::replace(self, State::None)
    }

    #[allow(unused)]
    fn as_ref_maybe_getter(&self) -> Option<&MaybeGetter> {
        use State::*;

        match &self {
            None | Fn => Option::None,
            NamedFn(maybe) | ParamList(maybe) | ParamLt(maybe) | ArgList(maybe) | ArgRef(maybe)
            | ArgRefLt(maybe) | ArgSelf(maybe) | Ret(maybe) | RetLt(maybe) => Some(maybe),
        }
    }

    fn take_maybe_getter(&mut self) -> Option<MaybeGetter> {
        use State::*;

        match self.take() {
            None | Fn => Option::None,
            NamedFn(maybe) | ParamList(maybe) | ParamLt(maybe) | ArgList(maybe) | ArgRef(maybe)
            | ArgRefLt(maybe) | ArgSelf(maybe) | Ret(maybe) | RetLt(maybe) => Some(maybe),
        }
    }
}

impl Default for State {
    fn default() -> Self {
        State::None
    }
}
