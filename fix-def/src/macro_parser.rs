use proc_macro2::{Delimiter, TokenStream, TokenTree};
use syn::buffer::{Cursor, TokenBuffer};

use rules::function::GetFunction;

#[derive(Debug)]
pub(crate) struct Getter {
    pub(crate) line_idx: usize,
    pub(crate) get_fn: GetFunction,
    pub(crate) returns_bool: bool,
}

#[derive(Debug, Default)]
pub(crate) struct GetterDefs {
    state: State,
    pub(crate) getters: Vec<Getter>,
}

impl GetterDefs {
    pub(crate) fn parse(stream: TokenStream) -> Vec<Getter> {
        let mut fn_calls = GetterDefs::default();
        let token_buf = TokenBuffer::new2(stream);
        fn_calls.collect_getter_calls(token_buf.begin());
        fn_calls.getters
    }

    fn collect_getter_calls(&mut self, mut rest: Cursor) {
        while let Some((tt, next)) = rest.token_tree() {
            // Find patterns `.get_suffix()`
            match tt {
                TokenTree::Punct(punct) => {
                    let char_ = punct.as_char();
                    match self.state.take() {
                        State::MaybeGetter(getter) => {
                            if char_ == '&' {
                                self.state = State::MaybeGetterRef(getter);
                            }
                        }
                        State::MaybeGetterSelf(getter) => {
                            if char_ == '-' {
                                self.state = State::MaybeGetterRet(getter);
                            }
                        }
                        State::MaybeGetterRet(getter) => {
                            if char_ == '>' {
                                self.state = State::MaybeGetterRet(getter);
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
                        if let Ok(get_fn) =
                            rules::function::GetFunction::try_from(ident.to_string())
                        {
                            self.state = State::MaybeGetter(Getter {
                                line_idx: ident.span().start().line - 1,
                                get_fn,
                                returns_bool: false,
                            });
                        }
                    }
                    State::MaybeGetterRef(getter) => {
                        if ident == "self" {
                            self.state = State::MaybeGetterSelf(getter);
                        }
                    }
                    State::MaybeGetterRet(mut getter) => {
                        if ident == "bool" {
                            getter.returns_bool = true;
                        }

                        self.getters.push(getter);
                    }
                    _ => (),
                },
                TokenTree::Group(group) => {
                    match self.state.take() {
                        State::MaybeGetterRet(getter) => {
                            // Returning complexe type
                            self.getters.push(getter);
                        }
                        State::MaybeGetter(getter) => {
                            if group.delimiter() == Delimiter::Parenthesis {
                                // might introduce the argument list
                                self.state = State::MaybeGetter(getter);
                            }
                        }
                        _ => (),
                    }

                    let token_buf = TokenBuffer::new2(group.stream());
                    self.collect_getter_calls(token_buf.begin());
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
    MaybeGetter(Getter),
    MaybeGetterRef(Getter),
    MaybeGetterSelf(Getter),
    MaybeGetterRet(Getter),
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
