use log::{debug, trace, warn};
use rules::function::{self, RenameError};
use std::{cell::RefCell, collections::HashMap, rc::Rc};
use syn::visit::{self, Visit};
use utils::scope::Scope;

#[derive(Debug)]
pub(crate) struct RenamableCall {
    pub(crate) name: String,
    pub(crate) new_name: String,
}

#[derive(Default)]
pub(crate) struct GetterVisitor {
    scope_stack: Vec<Rc<RefCell<Scope>>>,
    pub(crate) renamable_lines: HashMap<usize, Vec<RenamableCall>>,
}

impl GetterVisitor {
    pub(crate) fn process(&mut self, method_call: &syn::ExprMethodCall) {
        let scope = self.scope_stack.last().expect("empty scope stack").borrow();
        let method = method_call.method.to_string();

        println!("{} {}", scope, method);

        let filter_ok = match function::try_rename_getter_call(method_call) {
            Ok(filter_ok) => filter_ok,
            Err(err) => match err {
                RenameError::NotAGet => {
                    trace!("Getter visitor skipping {} in {}: {}", method, scope, err);
                    return;
                }
                _ => {
                    debug!("Getter visitor skipping {} in {}: {}", method, scope, err);
                    return;
                }
            },
        };

        if filter_ok.is_substituted() {
            if filter_ok.inner() != &method[4..] {
                warn!(
                    "Getter visitor: will substitute {} with {} in {}",
                    method,
                    filter_ok.inner(),
                    scope,
                );
            } else {
                // Substitute is same as suffix => don't advertise as substitute
                debug!(
                    "Getter visitor: will fix {} as {} in {}",
                    method,
                    filter_ok.inner(),
                    scope,
                );
            }
        } else if filter_ok.is_fixed() {
            debug!(
                "Getter visitor: will fix {} as {} in {}",
                method,
                filter_ok.inner(),
                scope,
            );
        } else {
            trace!(
                "Getter visitor will rename {} as {} in {}",
                method,
                filter_ok.inner(),
                scope
            );
        }

        let renamable_line = self
            .renamable_lines
            .entry(method_call.method.span().start().line - 1)
            .or_insert_with(Vec::new);

        renamable_line.push(RenamableCall {
            name: method,
            new_name: filter_ok.into_inner(),
        });
    }
}

impl<'ast> Visit<'ast> for GetterVisitor {
    fn visit_item(&mut self, node: &'ast syn::Item) {
        self.scope_stack.push(utils::scope::item_scope(node).into());
        visit::visit_item(self, node);
        self.scope_stack.pop();
    }

    fn visit_expr_method_call(&mut self, node: &'ast syn::ExprMethodCall) {
        //println!("{}, {:#?}", self.scope_stack.last().unwrap().borrow(), node);
        self.process(node);

        visit::visit_expr_method_call(self, node);
    }
}
