use log::{debug, trace, warn};
use rules::{self, RenameError, RenameOk, ReturnsBool};
use std::{cell::RefCell, collections::HashMap, rc::Rc};
use syn::visit::{self, Visit};
use utils::scope::Scope;

use crate::macro_parser::GetterCalls;

#[derive(Default)]
pub(crate) struct GetterVisitor {
    scope_stack: Vec<Rc<RefCell<Scope>>>,
    pub(crate) renamable_lines: HashMap<usize, Vec<RenameOk>>,
}

impl GetterVisitor {
    pub(crate) fn process(&mut self, rename_res: Result<RenameOk, RenameError>, line_idx: usize) {
        let scope = self.scope_stack.last().expect("empty scope stack").borrow();

        let rename = match rename_res {
            Ok(rename) => rename,
            Err(err) => match err {
                RenameError::GetFunction(err) => {
                    trace!("Getter visitor in {}, skipping {}", scope, err);
                    return;
                }
                _ => {
                    debug!("Getter visitor in {}, skipping {}", scope, err);
                    return;
                }
            },
        };

        if rename.is_substitute() {
            if rename.new_name() != &rename.name()[4..] {
                warn!("Getter visitor in {}: {}", scope, rename);
            } else {
                // Substitute is same as fix => don't warn as substitute
                debug!("Getter visitor in {}: {}", scope, rename);
            }
        } else if rename.is_fix() {
            debug!("Getter visitor in {}: {}", scope, rename);
        } else {
            trace!("Getter visitor in {}: {}", scope, rename);
        }

        let renamable_line = self
            .renamable_lines
            .entry(line_idx)
            .or_insert_with(Vec::new);

        renamable_line.push(rename);
    }
}

impl<'ast> Visit<'ast> for GetterVisitor {
    fn visit_item(&mut self, node: &'ast syn::Item) {
        self.scope_stack.push(utils::scope::item_scope(node).into());
        visit::visit_item(self, node);
        self.scope_stack.pop();
    }

    fn visit_expr_method_call(&mut self, node: &'ast syn::ExprMethodCall) {
        self.process(
            rules::try_rename_getter_call(node),
            node.method.span().start().line - 1,
        );

        visit::visit_expr_method_call(self, node);
    }

    fn visit_macro(&mut self, node: &'ast syn::Macro) {
        for getter in GetterCalls::parse(node.tokens.clone()) {
            self.process(
                getter.get_fn.try_rename(|| ReturnsBool::Maybe),
                getter.line_idx,
            )
        }
    }
}
