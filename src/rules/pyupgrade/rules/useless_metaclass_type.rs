use log::error;
use rustpython_ast::{Expr, ExprKind, Stmt};

use crate::ast::types::Range;
use crate::autofix::helpers;
use crate::checkers::ast::Checker;
use crate::registry::Diagnostic;
use crate::violations;

fn rule(targets: &[Expr], value: &Expr, location: Range) -> Option<Diagnostic> {
    if targets.len() != 1 {
        return None;
    }
    let ExprKind::Name { id, .. } = targets.first().map(|expr| &expr.node).unwrap() else {
        return None;
    };
    if id != "__metaclass__" {
        return None;
    }
    let ExprKind::Name { id, .. } = &value.node else {
        return None;
    };
    if id != "type" {
        return None;
    }
    Some(Diagnostic::new(violations::UselessMetaclassType, location))
}

/// UP001
pub fn useless_metaclass_type(checker: &mut Checker, stmt: &Stmt, value: &Expr, targets: &[Expr]) {
    let Some(mut diagnostic) =
        rule(targets, value, Range::from_located(stmt)) else {
            return;
        };
    if checker.patch(diagnostic.kind.code()) {
        let deleted: Vec<&Stmt> = checker
            .deletions
            .iter()
            .map(std::convert::Into::into)
            .collect();
        let defined_by = checker.current_stmt();
        let defined_in = checker.current_stmt_parent();
        match helpers::delete_stmt(
            defined_by.into(),
            defined_in.map(std::convert::Into::into),
            &deleted,
            checker.locator,
            checker.indexer,
        ) {
            Ok(fix) => {
                if fix.content.is_empty() || fix.content == "pass" {
                    checker.deletions.insert(defined_by.clone());
                }
                diagnostic.amend(fix);
            }
            Err(e) => error!("Failed to fix remove metaclass type: {e}"),
        }
    }
    checker.diagnostics.push(diagnostic);
}
