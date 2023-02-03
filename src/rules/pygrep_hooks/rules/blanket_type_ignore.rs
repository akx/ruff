use crate::define_simple_violation;
use crate::violation::Violation;
use once_cell::sync::Lazy;
use regex::Regex;
use ruff_macros::derive_message_formats;
use rustpython_ast::Location;

use crate::ast::types::Range;
use crate::registry::Diagnostic;

define_simple_violation!(
    BlanketTypeIgnore,
    "Use specific rule codes when ignoring type issues"
);

static BLANKET_TYPE_IGNORE_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"# type:? *ignore($|\s)").unwrap());

/// PGH003 - use of blanket type ignore comments
pub fn blanket_type_ignore(lineno: usize, line: &str) -> Option<Diagnostic> {
    BLANKET_TYPE_IGNORE_REGEX.find(line).map(|m| {
        Diagnostic::new(
            BlanketTypeIgnore,
            Range::new(
                Location::new(lineno + 1, m.start()),
                Location::new(lineno + 1, m.end()),
            ),
        )
    })
}
