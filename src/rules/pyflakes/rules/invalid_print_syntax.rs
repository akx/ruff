use crate::define_simple_violation;
use ruff_macros::derive_message_formats;
use rustpython_ast::{Expr, ExprKind};

use crate::ast::types::Range;
use crate::checkers::ast::Checker;
use crate::registry::Diagnostic;
use crate::violation::Violation;

define_simple_violation!(
    InvalidPrintSyntax,
    "Use of `>>` is invalid with `print` function"
);

/// F633
pub fn invalid_print_syntax(checker: &mut Checker, left: &Expr) {
    let ExprKind::Name { id, .. } = &left.node else {
        return;
    };
    if id != "print" {
        return;
    }
    if !checker.is_builtin("print") {
        return;
    };
    checker.diagnostics.push(Diagnostic::new(
        InvalidPrintSyntax,
        Range::from_located(left),
    ));
}
