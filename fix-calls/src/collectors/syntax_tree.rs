//! A [`SyntaxTreeGetterCollector`](utils::SyntaxTreeGetterCollector) collecting
//! renamable [`Getter`](utils::Getter) calls.

use rules::ReturnsBool;
use std::{
    cell::{Ref, RefCell},
    path::Path,
    rc::Rc,
};
use syn::visit::{self, Visit};
use utils::{getter, prelude::*, DocCodeGetterCollector, NonGetterReason, Scope};

use crate::{GetterCallCollection, TSGetterCallCollector};

/// A [`SyntaxTreeGetterCollector`](utils::SyntaxTreeGetterCollector) collecting
/// renamable [`Getter`](utils::Getter) calls.
#[derive(Debug)]
pub struct STGetterCallCollector<'path> {
    scope_stack: Vec<Rc<RefCell<Scope>>>,
    getter_collection: GetterCallCollection,
    path: &'path Path,
    doc_code_collector: DocCodeGetterCollector<TSGetterCallCollector<'path>>,
}

impl<'path> SyntaxTreeGetterCollector for STGetterCallCollector<'path> {
    type GetterCollection = GetterCallCollection;

    fn collect(path: &Path, syntax_tree: &syn::File, getter_collection: &GetterCallCollection) {
        let mut visitor = STGetterCallCollector {
            getter_collection: GetterCallCollection::clone(getter_collection),
            doc_code_collector: DocCodeGetterCollector::<TSGetterCallCollector>::new(
                path,
                &getter_collection,
            ),
            path,
            scope_stack: Vec::new(),
        };
        visitor.visit_file(syntax_tree);
    }
}

impl<'path> STGetterCallCollector<'path> {
    fn process_method_call(&mut self, method_call: &syn::ExprMethodCall) {
        use NonGetterReason::*;

        let res = self.getter_collection.try_new_getter(
            method_call.method.to_string(),
            ReturnsBool::Maybe,
            method_call.method.span().start().line,
        );
        let getter = match res {
            Ok(getter) => getter,
            Err(err) => {
                err.log(&self.scope());
                return;
            }
        };

        if !getter.returns_bool().is_true() {
            if method_call.turbofish.is_some() {
                getter::skip(&self.scope(), &getter.name, &GenericTypeParam, getter.line);
                return;
            }

            if !method_call.args.is_empty() {
                getter::skip(&self.scope(), &getter.name, &MultipleArgs, getter.line);
                return;
            }
        }

        getter.log(self.path, &self.scope());
        self.getter_collection.add(getter);
    }

    fn process_fn_call(&mut self, fn_call: &syn::ExprCall) {
        use NonGetterReason::*;

        if let syn::Expr::Path(expr_path) = fn_call.func.as_ref() {
            if let Some(last) = expr_path.path.segments.last() {
                let res = self.getter_collection.try_new_getter(
                    last.ident.to_string(),
                    ReturnsBool::Maybe,
                    last.ident.span().start().line,
                );
                let getter = match res {
                    Ok(getter) => getter,
                    Err(err) => {
                        err.log(&self.scope());
                        return;
                    }
                };

                if !getter.returns_bool().is_true() {
                    getter::skip(&self.scope(), &getter.name, &NotAMethod, getter.line);
                    return;
                }

                getter.log(self.path, &self.scope());
                self.getter_collection.add(getter);
            }
        }
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

impl<'ast, 'path> Visit<'ast> for STGetterCallCollector<'path> {
    fn visit_item(&mut self, node: &'ast syn::Item) {
        self.push_scope(node);
        visit::visit_item(self, node);
        self.pop_scope();
    }

    fn visit_expr_method_call(&mut self, node: &'ast syn::ExprMethodCall) {
        self.process_method_call(node);
        visit::visit_expr_method_call(self, node);
    }

    fn visit_expr_call(&mut self, node: &'ast syn::ExprCall) {
        self.process_fn_call(node);
        visit::visit_expr_call(self, node);
    }

    fn visit_macro(&mut self, node: &'ast syn::Macro) {
        self.push_scope(node);
        TSGetterCallCollector::collect(
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
