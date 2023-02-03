use num_bigint::BigInt;
use rustpython_ast::{Cmpop, Constant, Expr, ExprKind, Located};

use crate::ast::types::Range;
use crate::checkers::ast::Checker;
use crate::registry::{Diagnostic, Rule};
use crate::violation::Violation;

use crate::define_simple_violation;
use ruff_macros::derive_message_formats;

define_simple_violation!(
    SysVersionSlice3Referenced,
    "`sys.version[:3]` referenced (python3.10), use `sys.version_info`"
);

define_simple_violation!(
    SysVersion2Referenced,
    "`sys.version[2]` referenced (python3.10), use `sys.version_info`"
);

define_simple_violation!(
    SysVersionCmpStr3,
    "`sys.version` compared to string (python3.10), use `sys.version_info`"
);

define_simple_violation!(
    SysVersionInfo0Eq3Referenced,
    "`sys.version_info[0] == 3` referenced (python4), use `>=`"
);

define_simple_violation!(
    SixPY3Referenced,
    "`six.PY3` referenced (python4), use `not six.PY2`"
);

define_simple_violation!(
    SysVersionInfo1CmpInt,
    "`sys.version_info[1]` compared to integer (python4), compare `sys.version_info` to \
             tuple"
);

define_simple_violation!(
    SysVersionInfoMinorCmpInt,
    "`sys.version_info.minor` compared to integer (python4), compare `sys.version_info` \
             to tuple"
);

define_simple_violation!(
    SysVersion0Referenced,
    "`sys.version[0]` referenced (python10), use `sys.version_info`"
);

define_simple_violation!(
    SysVersionCmpStr10,
    "`sys.version` compared to string (python10), use `sys.version_info`"
);

define_simple_violation!(
    SysVersionSlice1Referenced,
    "`sys.version[:1]` referenced (python10), use `sys.version_info`"
);

fn is_sys(checker: &Checker, expr: &Expr, target: &str) -> bool {
    checker
        .resolve_call_path(expr)
        .map_or(false, |call_path| call_path.as_slice() == ["sys", target])
}

/// YTT101, YTT102, YTT301, YTT303
pub fn subscript(checker: &mut Checker, value: &Expr, slice: &Expr) {
    if is_sys(checker, value, "version") {
        match &slice.node {
            ExprKind::Slice {
                lower: None,
                upper: Some(upper),
                step: None,
                ..
            } => {
                if let ExprKind::Constant {
                    value: Constant::Int(i),
                    ..
                } = &upper.node
                {
                    if *i == BigInt::from(1)
                        && checker
                            .settings
                            .rules
                            .enabled(&Rule::SysVersionSlice1Referenced)
                    {
                        checker.diagnostics.push(Diagnostic::new(
                            SysVersionSlice1Referenced,
                            Range::from_located(value),
                        ));
                    } else if *i == BigInt::from(3)
                        && checker
                            .settings
                            .rules
                            .enabled(&Rule::SysVersionSlice3Referenced)
                    {
                        checker.diagnostics.push(Diagnostic::new(
                            SysVersionSlice3Referenced,
                            Range::from_located(value),
                        ));
                    }
                }
            }

            ExprKind::Constant {
                value: Constant::Int(i),
                ..
            } => {
                if *i == BigInt::from(2)
                    && checker.settings.rules.enabled(&Rule::SysVersion2Referenced)
                {
                    checker.diagnostics.push(Diagnostic::new(
                        SysVersion2Referenced,
                        Range::from_located(value),
                    ));
                } else if *i == BigInt::from(0)
                    && checker.settings.rules.enabled(&Rule::SysVersion0Referenced)
                {
                    checker.diagnostics.push(Diagnostic::new(
                        SysVersion0Referenced,
                        Range::from_located(value),
                    ));
                }
            }

            _ => {}
        }
    }
}

/// YTT103, YTT201, YTT203, YTT204, YTT302
pub fn compare(checker: &mut Checker, left: &Expr, ops: &[Cmpop], comparators: &[Expr]) {
    match &left.node {
        ExprKind::Subscript { value, slice, .. } if is_sys(checker, value, "version_info") => {
            if let ExprKind::Constant {
                value: Constant::Int(i),
                ..
            } = &slice.node
            {
                if *i == BigInt::from(0) {
                    if let (
                        [Cmpop::Eq | Cmpop::NotEq],
                        [Located {
                            node:
                                ExprKind::Constant {
                                    value: Constant::Int(n),
                                    ..
                                },
                            ..
                        }],
                    ) = (ops, comparators)
                    {
                        if *n == BigInt::from(3)
                            && checker
                                .settings
                                .rules
                                .enabled(&Rule::SysVersionInfo0Eq3Referenced)
                        {
                            checker.diagnostics.push(Diagnostic::new(
                                SysVersionInfo0Eq3Referenced,
                                Range::from_located(left),
                            ));
                        }
                    }
                } else if *i == BigInt::from(1) {
                    if let (
                        [Cmpop::Lt | Cmpop::LtE | Cmpop::Gt | Cmpop::GtE],
                        [Located {
                            node:
                                ExprKind::Constant {
                                    value: Constant::Int(_),
                                    ..
                                },
                            ..
                        }],
                    ) = (ops, comparators)
                    {
                        if checker.settings.rules.enabled(&Rule::SysVersionInfo1CmpInt) {
                            checker.diagnostics.push(Diagnostic::new(
                                SysVersionInfo1CmpInt,
                                Range::from_located(left),
                            ));
                        }
                    }
                }
            }
        }

        ExprKind::Attribute { value, attr, .. }
            if is_sys(checker, value, "version_info") && attr == "minor" =>
        {
            if let (
                [Cmpop::Lt | Cmpop::LtE | Cmpop::Gt | Cmpop::GtE],
                [Located {
                    node:
                        ExprKind::Constant {
                            value: Constant::Int(_),
                            ..
                        },
                    ..
                }],
            ) = (ops, comparators)
            {
                if checker
                    .settings
                    .rules
                    .enabled(&Rule::SysVersionInfoMinorCmpInt)
                {
                    checker.diagnostics.push(Diagnostic::new(
                        SysVersionInfoMinorCmpInt,
                        Range::from_located(left),
                    ));
                }
            }
        }

        _ => {}
    }

    if is_sys(checker, left, "version") {
        if let (
            [Cmpop::Lt | Cmpop::LtE | Cmpop::Gt | Cmpop::GtE],
            [Located {
                node:
                    ExprKind::Constant {
                        value: Constant::Str(s),
                        ..
                    },
                ..
            }],
        ) = (ops, comparators)
        {
            if s.len() == 1 {
                if checker.settings.rules.enabled(&Rule::SysVersionCmpStr10) {
                    checker.diagnostics.push(Diagnostic::new(
                        SysVersionCmpStr10,
                        Range::from_located(left),
                    ));
                }
            } else if checker.settings.rules.enabled(&Rule::SysVersionCmpStr3) {
                checker.diagnostics.push(Diagnostic::new(
                    SysVersionCmpStr3,
                    Range::from_located(left),
                ));
            }
        }
    }
}

/// YTT202
pub fn name_or_attribute(checker: &mut Checker, expr: &Expr) {
    if checker
        .resolve_call_path(expr)
        .map_or(false, |call_path| call_path.as_slice() == ["six", "PY3"])
    {
        checker
            .diagnostics
            .push(Diagnostic::new(SixPY3Referenced, Range::from_located(expr)));
    }
}
