use crate::ast::types::{Range, ScopeKind};
use crate::checkers::ast::Checker;
use crate::define_simple_violation;
use crate::registry::Diagnostic;
use crate::violation::Violation;
use ruff_macros::derive_message_formats;
use rustpython_ast::{Expr, ExprKind};

define_simple_violation!(
    CachedInstanceMethod,
    "Use of `functools.lru_cache` or `functools.cache` on methods can lead to memory leaks"
);

fn is_cache_func(checker: &Checker, expr: &Expr) -> bool {
    checker.resolve_call_path(expr).map_or(false, |call_path| {
        call_path.as_slice() == ["functools", "lru_cache"]
            || call_path.as_slice() == ["functools", "cache"]
    })
}

/// B019
pub fn cached_instance_method(checker: &mut Checker, decorator_list: &[Expr]) {
    if !matches!(checker.current_scope().kind, ScopeKind::Class(_)) {
        return;
    }
    for decorator in decorator_list {
        // TODO(charlie): This should take into account `classmethod-decorators` and
        // `staticmethod-decorators`.
        if let ExprKind::Name { id, .. } = &decorator.node {
            if id == "classmethod" || id == "staticmethod" {
                return;
            }
        }
    }
    for decorator in decorator_list {
        if is_cache_func(
            checker,
            match &decorator.node {
                ExprKind::Call { func, .. } => func,
                _ => decorator,
            },
        ) {
            checker.diagnostics.push(Diagnostic::new(
                CachedInstanceMethod,
                Range::from_located(decorator),
            ));
        }
    }
}
