use proc_macro2::{Delimiter, TokenStream, TokenTree};
use syn::buffer::{Cursor, TokenBuffer};

use rules::function::GetFunction;

#[derive(Debug)]
pub(crate) struct Getter {
    pub(crate) line_idx: usize,
    pub(crate) get_fn: GetFunction,
}

#[derive(Debug, Default)]
pub(crate) struct GetterCalls {
    state: State,
    pub(crate) getters: Vec<Getter>,
}

impl GetterCalls {
    pub(crate) fn parse(stream: TokenStream) -> Vec<Getter> {
        let mut fn_calls = GetterCalls::default();
        let token_buf = TokenBuffer::new2(stream);
        fn_calls.collect_getter_calls(token_buf.begin());
        fn_calls.getters
    }

    fn collect_getter_calls(&mut self, mut rest: Cursor) {
        while let Some((tt, next)) = rest.token_tree() {
            // Find patterns `.get_suffix()`
            match tt {
                TokenTree::Punct(punct) => {
                    if punct.as_char() == '.' {
                        self.state = State::Dot;
                    } else {
                        self.state = State::None;
                    }
                }
                TokenTree::Ident(ident) => {
                    if let State::Dot = self.state.take() {
                        if let Ok(get_fn) =
                            rules::function::GetFunction::try_from(ident.to_string())
                        {
                            self.state = State::MaybeGetter(Getter {
                                line_idx: ident.span().start().line - 1,
                                get_fn,
                            });
                        }
                    }
                }
                TokenTree::Group(group) => {
                    if group.stream().is_empty() && group.delimiter() == Delimiter::Parenthesis {
                        if let State::MaybeGetter(getter) = self.state.take() {
                            // found `()` after a getter call
                            self.getters.push(getter);
                        }
                    } else {
                        self.state = State::None;
                        let token_buf = TokenBuffer::new2(group.stream());
                        self.collect_getter_calls(token_buf.begin());
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
