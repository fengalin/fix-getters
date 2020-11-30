//! Syn Visitor in search of renamable getter calls.

use rules::ReturnsBool;
use std::{
    cell::{Ref, RefCell},
    collections::HashMap,
    rc::Rc,
};
use syn::visit::{self, Visit};
use utils::{getter, Getter, NonGetterReason, Scope};

use crate::macro_parser;

/// Syn Visitor in search of renamable getter calls.
#[derive(Default)]
pub(crate) struct GetterCallsVisitor {
    scope_stack: Vec<Rc<RefCell<Scope>>>,
    pub(crate) getter_calls: HashMap<usize, Vec<Getter>>,
}

impl GetterCallsVisitor {
    fn add(&mut self, getter_call: Getter) {
        let getter_calls_same_line = self
            .getter_calls
            .entry(getter_call.line - 1) // convert line nb to line idx
            .or_insert_with(Vec::new);

        getter_calls_same_line.push(getter_call);
    }

    fn process(&mut self, method_call: &syn::ExprMethodCall) {
        use NonGetterReason::*;

        let res = Getter::try_new_and_log(
            &self.scope(),
            method_call.method.to_string(),
            ReturnsBool::Maybe,
            method_call.method.span().start().line,
        );
        let getter = match res {
            Ok(getter) => getter,
            Err(_) => return,
        };

        if method_call.turbofish.is_some() {
            getter::skip(&self.scope(), getter.name, &GenericTypeParam, getter.line);
            return;
        }

        if !method_call.args.is_empty() {
            getter::skip(&self.scope(), getter.name, &MultipleArgs, getter.line);
            return;
        }

        self.add(getter);
    }

    fn scope(&self) -> Ref<Scope> {
        self.scope_stack.last().expect("empty scope stack").borrow()
    }
}

impl<'ast> Visit<'ast> for GetterCallsVisitor {
    fn visit_item(&mut self, node: &'ast syn::Item) {
        self.scope_stack.push(Scope::from(node).into());
        visit::visit_item(self, node);
        self.scope_stack.pop();
    }

    fn visit_expr_method_call(&mut self, node: &'ast syn::ExprMethodCall) {
        self.process(node);
        visit::visit_expr_method_call(self, node);
    }

    fn visit_macro(&mut self, node: &'ast syn::Macro) {
        let getter_calls =
            macro_parser::GetterCallsCollector::collect(node.tokens.clone(), &self.scope());
        for getter_call in getter_calls {
            self.add(getter_call)
        }
    }
}
