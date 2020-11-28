use log::{debug, trace, warn};
use rules::{self, RenameError, RenameOk, ReturnsBool};
use std::{cell::RefCell, collections::HashMap, rc::Rc};
use syn::visit::{self, Visit};
use utils::scope::Scope;

use crate::macro_parser::GetterDefs;

#[derive(Debug)]
pub(crate) struct RenamableDef {
    pub(crate) rename: RenameOk,
    pub(crate) needs_doc_alias: bool,
}

#[derive(Default)]
pub(crate) struct GetterVisitor {
    scope_stack: Vec<Rc<RefCell<Scope>>>,
    pub(crate) renamable_lines: HashMap<usize, RenamableDef>,
}

impl GetterVisitor {
    pub(crate) fn process_signature(&mut self, sig: &syn::Signature) {
        let line_idx = sig.ident.span().start().line - 1;
        self.process(rules::try_rename_getter_def(sig), line_idx);
    }

    pub(crate) fn process(&mut self, rename_res: Result<RenameOk, RenameError>, line_idx: usize) {
        let scope = self.scope_stack.last().expect("empty scope stack").borrow();

        use Scope::*;
        let needs_doc_alias = match *scope {
            StructImpl(_) | Trait(_) | Macro(_) => true,
            TraitImpl { .. } => false,
            _ => return,
        };

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
            warn!("Getter visitor in {}, {}", scope, rename);
        } else if rename.is_fix() {
            debug!("Getter visitor in {}, {}", scope, rename);
        }

        let rd = RenamableDef {
            rename,
            needs_doc_alias,
        };

        if self.renamable_lines.insert(line_idx, rd).is_some() {
            panic!("Found more than one getter definition @ {}", line_idx);
        }
    }
}

impl<'ast> Visit<'ast> for GetterVisitor {
    fn visit_item(&mut self, node: &'ast syn::Item) {
        self.scope_stack.push(utils::scope::item_scope(node).into());
        visit::visit_item(self, node);
        self.scope_stack.pop();
    }

    fn visit_item_fn(&mut self, node: &'ast syn::ItemFn) {
        let syn::ItemFn { sig, .. } = node;
        self.process_signature(sig);

        visit::visit_item_fn(self, node);
    }

    fn visit_impl_item_method(&mut self, node: &'ast syn::ImplItemMethod) {
        let syn::ImplItemMethod { sig, .. } = node;
        self.process_signature(sig);

        visit::visit_impl_item_method(self, node);
    }

    fn visit_trait_item_method(&mut self, node: &'ast syn::TraitItemMethod) {
        let syn::TraitItemMethod { sig, .. } = node;
        self.process_signature(sig);

        visit::visit_trait_item_method(self, node);
    }

    fn visit_macro(&mut self, node: &'ast syn::Macro) {
        for getter in GetterDefs::parse(node.tokens.clone()) {
            let returns_bool: ReturnsBool = getter.returns_bool.into();
            self.process(getter.get_fn.try_rename(|| returns_bool), getter.line_idx)
        }
    }
}
