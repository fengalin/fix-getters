//! Macro parser in search of renamable getter calls.

use proc_macro2::{Delimiter, TokenStream, TokenTree};
use syn::buffer::{Cursor, TokenBuffer};

use rules::ReturnsBool;
use utils::{getter, Getter, NonGetterReason, Scope};

#[derive(Debug)]
pub(crate) struct GetterCallsCollector<'scope> {
    state: State,
    pub(crate) getter_calls: Vec<Getter>,
    scope: &'scope Scope,
}

impl<'scope> GetterCallsCollector<'scope> {
    pub(crate) fn collect(stream: TokenStream, scope: &Scope) -> Vec<Getter> {
        let mut this = GetterCallsCollector {
            state: State::default(),
            getter_calls: Vec::new(),
            scope,
        };
        let token_buf = TokenBuffer::new2(stream);
        this.parse(token_buf.begin());
        this.getter_calls
    }

    fn parse(&mut self, mut rest: Cursor) {
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
                                    self.scope,
                                    getter.name,
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
                        let res = Getter::try_new(
                            ident.to_string(),
                            ReturnsBool::Maybe,
                            ident.span().start().line,
                        );
                        match res {
                            Ok(getter) => {
                                // Will log when the getter is confirmed
                                self.state = State::MaybeGetter(getter)
                            }
                            Err(err) => getter::log_err(self.scope, &err),
                        }
                    }
                }
                TokenTree::Group(group) => {
                    if let State::MaybeGetter(getter) = self.state.take() {
                        if let Delimiter::Parenthesis = group.delimiter() {
                            if group.stream().is_empty() {
                                // found `()` after a getter call
                                getter.log(self.scope);
                                self.getter_calls.push(getter);
                            } else {
                                getter::skip(self.scope, getter.name, &MultipleArgs, getter.line);
                            }
                        }
                    }

                    if !group.stream().is_empty() {
                        let token_buf = TokenBuffer::new2(group.stream());
                        self.parse(token_buf.begin());
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
