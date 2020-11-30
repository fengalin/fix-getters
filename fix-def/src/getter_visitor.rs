//! Syn Visitor in search of renamable getter definitions.

use std::{
    cell::{Ref, RefCell},
    collections::HashMap,
    rc::Rc,
};
use syn::visit::{self, Visit};
use utils::{getter, NonGetterReason, Scope};

use crate::{macro_parser, GetterDef};

/// Syn Visitor in search of renamable getter definitions.
#[derive(Default)]
pub(crate) struct GetterDefsVisitor {
    scope_stack: Vec<Rc<RefCell<Scope>>>,
    pub(crate) getter_defs: HashMap<usize, GetterDef>,
}

impl GetterDefsVisitor {
    fn add(&mut self, getter_def: GetterDef) {
        // convert line nb to line idx
        let line_idx = getter_def.line - 1;
        if self.getter_defs.insert(line_idx, getter_def).is_some() {
            panic!("Found more than one getter definition @ {}", line_idx + 1);
        }
    }

    fn process(&mut self, sig: &syn::Signature) {
        use NonGetterReason::*;
        use Scope::*;

        let needs_doc_alias = match *self.scope() {
            StructImpl(_) | Trait(_) | Macro(_) => true,
            TraitImpl { .. } => false,
            _ => return,
        };

        let res = GetterDef::try_new_and_log(
            &self.scope(),
            sig.ident.to_string(),
            Self::returns_bool(sig),
            sig.ident.span().start().line,
            needs_doc_alias,
        );
        let getter = match res {
            Ok(getter) => getter,
            Err(_) => return,
        };

        if !sig.generics.params.is_empty() {
            getter::skip(&self.scope(), getter.name, &GenericTypeParam, getter.line);
            return;
        }

        if sig.inputs.len() > 1 {
            getter::skip(&self.scope(), getter.name, &MultipleArgs, getter.line);
            return;
        }

        match sig.inputs.first() {
            Some(syn::FnArg::Receiver { .. }) => (),
            Some(_) => {
                getter::skip(&self.scope(), getter.name, &NonSelfUniqueArg, getter.line);
                return;
            }
            None => {
                getter::skip(&self.scope(), getter.name, &NoArgs, getter.line);
                return;
            }
        }

        self.add(getter);
    }

    fn returns_bool(sig: &syn::Signature) -> bool {
        if let syn::ReturnType::Type(_, type_) = &sig.output {
            if let syn::Type::Path(path_type) = type_.as_ref() {
                let path = &path_type.path;
                if path.segments.len() == 1 {
                    if let Some(syn::PathSegment { ident, .. }) = path.segments.first() {
                        if ident == "bool" {
                            return true;
                        }
                    }
                }
            }
        }

        false
    }

    fn scope(&self) -> Ref<Scope> {
        self.scope_stack.last().expect("empty scope stack").borrow()
    }
}

impl<'ast> Visit<'ast> for GetterDefsVisitor {
    fn visit_item(&mut self, node: &'ast syn::Item) {
        self.scope_stack.push(Scope::from(node).into());
        visit::visit_item(self, node);
        self.scope_stack.pop();
    }

    fn visit_item_fn(&mut self, node: &'ast syn::ItemFn) {
        self.process(&node.sig);
        visit::visit_item_fn(self, node);
    }

    fn visit_impl_item_method(&mut self, node: &'ast syn::ImplItemMethod) {
        self.process(&node.sig);
        visit::visit_impl_item_method(self, node);
    }

    fn visit_trait_item_method(&mut self, node: &'ast syn::TraitItemMethod) {
        self.process(&node.sig);
        visit::visit_trait_item_method(self, node);
    }

    fn visit_macro(&mut self, node: &'ast syn::Macro) {
        let getter_defs =
            macro_parser::GetterDefsCollector::collect(node.tokens.clone(), &self.scope());
        for getter_def in getter_defs {
            self.add(getter_def)
        }
    }
}
