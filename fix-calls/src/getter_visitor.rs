//! Syn Visitor in search of renamable getter calls.

use rules::ReturnsBool;
use std::{
    cell::{Ref, RefCell},
    rc::Rc,
};
use syn::visit::{self, Visit};
use utils::{getter, parser::prelude::*, DocCodeParser, NonGetterReason, Scope};

use crate::{GetterCallCollection, TSGetterCallParser};

/// Syn Visitor in search of renamable [`Getter`](utils::Getter) calls.
#[derive(Debug)]
pub struct GetterCallVisitor {
    scope_stack: Vec<Rc<RefCell<Scope>>>,
    getter_collection: GetterCallCollection,
    doc_code_parser: DocCodeParser<TSGetterCallParser>,
}

impl GetterVisitor for GetterCallVisitor {
    type GetterCollection = GetterCallCollection;

    fn visit(syntax_tree: &syn::File, getter_collection: &GetterCallCollection) {
        let mut visitor = GetterCallVisitor {
            doc_code_parser: DocCodeParser::<TSGetterCallParser>::new(&getter_collection),
            getter_collection: GetterCallCollection::clone(getter_collection),
            scope_stack: Vec::new(),
        };
        visitor.visit_file(syntax_tree);
    }
}

impl GetterCallVisitor {
    fn process(&mut self, method_call: &syn::ExprMethodCall) {
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

        if method_call.turbofish.is_some() {
            getter::skip(&self.scope(), &getter.name, &GenericTypeParam, getter.line);
            return;
        }

        if !method_call.args.is_empty() {
            getter::skip(&self.scope(), &getter.name, &MultipleArgs, getter.line);
            return;
        }

        getter.log(&self.scope());
        self.getter_collection.add(getter);
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

impl<'ast> Visit<'ast> for GetterCallVisitor {
    fn visit_item(&mut self, node: &'ast syn::Item) {
        self.push_scope(node);
        visit::visit_item(self, node);
        self.pop_scope();
    }

    fn visit_expr_method_call(&mut self, node: &'ast syn::ExprMethodCall) {
        self.process(node);
        visit::visit_expr_method_call(self, node);
    }

    fn visit_macro(&mut self, node: &'ast syn::Macro) {
        self.push_scope(node);
        TSGetterCallParser::parse(
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
