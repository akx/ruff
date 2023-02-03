use ruff_macros::derive_message_formats;
use rustpython_ast::{ExprKind, Stmt, StmtKind};

use crate::ast::types::Range;
use crate::ast::visitor::{self, Visitor};
use crate::checkers::ast::Checker;
use crate::define_simple_violation;
use crate::registry::Diagnostic;
use crate::violation::Violation;

define_simple_violation!(
    ReraiseNoCause,
    "Use `raise from` to specify exception cause"
);

#[derive(Default)]
struct RaiseStatementVisitor<'a> {
    raises: Vec<&'a Stmt>,
}

impl<'a, 'b> Visitor<'b> for RaiseStatementVisitor<'a>
where
    'b: 'a,
{
    fn visit_stmt(&mut self, stmt: &'b Stmt) {
        match stmt.node {
            StmtKind::Raise { .. } => self.raises.push(stmt),
            _ => visitor::walk_stmt(self, stmt),
        }
    }
}

/// TRY200
pub fn reraise_no_cause(checker: &mut Checker, body: &[Stmt]) {
    let raises = {
        let mut visitor = RaiseStatementVisitor::default();
        for stmt in body {
            visitor.visit_stmt(stmt);
        }
        visitor.raises
    };

    for stmt in raises {
        if let StmtKind::Raise { exc, cause, .. } = &stmt.node {
            if exc
                .as_ref()
                .map_or(false, |expr| matches!(expr.node, ExprKind::Call { .. }))
                && cause.is_none()
            {
                checker
                    .diagnostics
                    .push(Diagnostic::new(ReraiseNoCause, Range::from_located(stmt)));
            }
        }
    }
}
