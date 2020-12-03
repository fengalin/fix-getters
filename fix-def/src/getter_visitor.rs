//! Syn Visitor in search of renamable getter definitions.

use std::{
    cell::{Ref, RefCell},
    rc::Rc,
};
use syn::visit::{self, Visit};
use utils::{getter, parser::prelude::*, DocCodeParser, NonGetterReason, Scope};

use crate::{GetterDefCollection, TSGetterDefParser};

/// Syn Visitor in search of renamable [`Getter`](utils::Getter) definitions.
#[derive(Debug)]
pub struct GetterDefVisitor {
    getter_collection: GetterDefCollection,
    scope_stack: Vec<Rc<RefCell<Scope>>>,
    doc_code_parser: DocCodeParser<TSGetterDefParser>,
}

impl GetterVisitor for GetterDefVisitor {
    type GetterCollection = GetterDefCollection;

    fn visit(syntax_tree: &syn::File, getter_collection: &GetterDefCollection) {
        let mut visitor = GetterDefVisitor {
            doc_code_parser: DocCodeParser::<TSGetterDefParser>::new(&getter_collection),
            getter_collection: GetterDefCollection::clone(getter_collection),
            scope_stack: Vec::new(),
        };
        visitor.visit_file(syntax_tree);
    }
}

impl GetterDefVisitor {
    fn process(&mut self, sig: &syn::Signature) {
        use NonGetterReason::*;
        use Scope::*;

        let needs_doc_alias = match *self.scope() {
            StructImpl(_) | Trait(_) | Macro(_) => true,
            TraitImpl { .. } | Attribute(_) => false,
            _ => return,
        };

        let res = self.getter_collection.try_new_getter(
            sig.ident.to_string(),
            Self::returns_bool(sig),
            sig.ident.span().start().line,
            needs_doc_alias,
        );
        let getter = match res {
            Ok(getter) => getter,
            Err(err) => {
                err.log(&self.scope());
                return;
            }
        };

        if !sig.generics.params.is_empty() {
            getter::skip(
                &self.scope(),
                getter.name(),
                &GenericTypeParam,
                getter.line(),
            );
            return;
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

        getter.log(&self.scope());
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

impl<'ast> Visit<'ast> for GetterDefVisitor {
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
        TSGetterDefParser::parse(
            &node.tokens,
            &self.scope_stack.last().expect("empty scope stack"),
            &self.getter_collection,
        );
        self.pop_scope();
    }

    fn visit_attribute(&mut self, node: &'ast syn::Attribute) {
        // Each doc line is passed as an attribute
        self.doc_code_parser.have_attribute(node);
    }
}
