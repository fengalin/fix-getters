//! A [`SyntaxTreeGetterCollector`](utils::SyntaxTreeGetterCollector) collecting
//! renamable [`Getter`](utils::Getter) definitions as [`GetterDef`](crate::GetterDef).

use std::{
    cell::{Ref, RefCell},
    path::Path,
    rc::Rc,
};
use syn::visit::{self, Visit};
use utils::{getter, prelude::*, DocCodeGetterCollector, NonGetterReason, Scope};

use crate::{GetterDefCollection, TsGetterDefCollector};

/// A [`SyntaxTreeGetterCollector`](utils::SyntaxTreeGetterCollector) collecting
/// renamable [`Getter`](utils::Getter) definitions as [`GetterDef`](crate::GetterDef).
#[derive(Debug)]
pub struct StGetterDefCollector<'path> {
    getter_collection: GetterDefCollection,
    scope_stack: Vec<Rc<RefCell<Scope>>>,
    path: &'path Path,
    doc_code_collector: DocCodeGetterCollector<TsGetterDefCollector<'path>>,
}

impl<'path> SyntaxTreeGetterCollector for StGetterDefCollector<'path> {
    type GetterCollection = GetterDefCollection;

    fn collect(path: &Path, syntax_tree: &syn::File, getter_collection: &GetterDefCollection) {
        let mut visitor = StGetterDefCollector {
            getter_collection: GetterDefCollection::clone(getter_collection),
            doc_code_collector: DocCodeGetterCollector::<TsGetterDefCollector>::new(
                path,
                &getter_collection,
            ),
            path,
            scope_stack: Vec::new(),
        };
        visitor.visit_file(syntax_tree);
    }
}

impl<'path> StGetterDefCollector<'path> {
    fn process(&mut self, sig: &syn::Signature) {
        use NonGetterReason::*;
        use Scope::*;

        let returns_bool = Self::returns_bool(sig);
        let line = sig.ident.span().start().line;

        let res = self
            .getter_collection
            .try_new_getter(sig.ident.to_string(), returns_bool, line);
        let mut getter = match res {
            Ok(getter) => getter,
            Err(err) => {
                err.log(&self.scope());
                return;
            }
        };

        let needs_doc_alias = match *self.scope() {
            StructImpl(_) | Trait(_) | Macro(_) => true,
            TraitImpl { .. } | Attribute(_) => false,
            _ => {
                if !returns_bool {
                    getter::skip(&self.scope(), &sig.ident.to_string(), &NotAMethod, line);
                    return;
                }
                true
            }
        };
        getter.set_needs_doc_alias(needs_doc_alias);

        if !returns_bool {
            for param in &sig.generics.params {
                match param {
                    syn::GenericParam::Lifetime(_) => (),
                    _ => {
                        getter::skip(
                            &self.scope(),
                            getter.name(),
                            &GenericTypeParam,
                            getter.line(),
                        );
                        return;
                    }
                }
            }

            if sig.inputs.len() > 1 {
                getter::skip(&self.scope(), getter.name(), &MultipleArgs, getter.line());
                return;
            }

            match sig.inputs.first() {
                Some(syn::FnArg::Receiver { .. }) => (),
                Some(_) => {
                    getter::skip(
                        &self.scope(),
                        getter.name(),
                        &NonSelfUniqueArg,
                        getter.line(),
                    );
                    return;
                }
                None => {
                    getter::skip(&self.scope(), getter.name(), &NoArgs, getter.line());
                    return;
                }
            }
        }

        getter.log(&self.path, &self.scope());
        self.getter_collection.add(getter);
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

    fn push_scope(&mut self, scope: impl Into<Scope>) {
        let scope = scope.into().into();
        self.scope_stack.push(scope);
    }

    fn pop_scope(&mut self) {
        self.scope_stack.pop();
    }
}

impl<'ast, 'path> Visit<'ast> for StGetterDefCollector<'path> {
    fn visit_item(&mut self, node: &'ast syn::Item) {
        self.push_scope(node);
        visit::visit_item(self, node);
        self.pop_scope();
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
        self.push_scope(node);
        TsGetterDefCollector::collect(
            self.path,
            &self.scope(),
            &node.tokens,
            &self.getter_collection,
        );
        self.pop_scope();
    }

    fn visit_attribute(&mut self, node: &'ast syn::Attribute) {
        // Each doc line is passed as an attribute
        self.doc_code_collector.have_attribute(node);
    }
}
