use log::{debug, trace, warn};
use rules::function::{self, RenameError};
use std::{cell::RefCell, rc::Rc};
use syn::visit::{self, Visit};
use utils::scope::{FnWithScope, Scope};

#[derive(Debug)]
pub(crate) struct RenamableGetter {
    pub(crate) line_nb: usize,
    pub(crate) name: String,
    pub(crate) new_name: String,
    pub(crate) needs_doc_alias: bool,
}

#[derive(Default)]
pub(crate) struct GetterVisitor {
    scope_stack: Vec<Rc<RefCell<Scope>>>,
    pub(crate) renamable_getters: Vec<RenamableGetter>,
}

impl GetterVisitor {
    pub(crate) fn process(&mut self, sig: &syn::Signature) {
        let fn_with_scope = FnWithScope::new(
            &sig.ident,
            &self.scope_stack.last().expect("empty scope stack"),
        );

        use Scope::*;
        let needs_doc_alias = match *fn_with_scope.scope() {
            StructImpl(_) | Trait(_) => true,
            TraitImpl { .. } => false,
            _ => return,
        };

        let filter_ok = match function::try_rename_getter(sig) {
            Ok(filter_ok) => filter_ok,
            Err(err) => match err {
                RenameError::NotAGet => {
                    trace!("Getter visitor skipping {}: {}", fn_with_scope, err);
                    return;
                }
                _ => {
                    debug!("Getter visitor skipping {}: {}", fn_with_scope, err);
                    return;
                }
            },
        };

        if filter_ok.is_substituted() {
            warn!(
                "Getter visitor: will substitute {} with {}",
                fn_with_scope,
                filter_ok.inner()
            );
        } else if filter_ok.is_fixed() {
            debug!(
                "Getter visitor: will fix {} as {}",
                fn_with_scope,
                filter_ok.inner()
            );
        }

        self.renamable_getters.push(RenamableGetter {
            line_nb: sig.ident.span().start().line - 1,
            name: fn_with_scope.fn_().to_string(),
            new_name: filter_ok.into_inner(),
            needs_doc_alias,
        });
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
        self.process(sig);

        visit::visit_item_fn(self, node);
    }

    fn visit_impl_item_method(&mut self, node: &'ast syn::ImplItemMethod) {
        let syn::ImplItemMethod { sig, .. } = node;
        self.process(sig);

        visit::visit_impl_item_method(self, node);
    }

    fn visit_trait_item_method(&mut self, node: &'ast syn::TraitItemMethod) {
        let syn::TraitItemMethod { sig, .. } = node;
        self.process(sig);

        visit::visit_trait_item_method(self, node);
    }
}
