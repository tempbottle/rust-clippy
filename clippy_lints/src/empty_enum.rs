//! lint when there is an enum with no variants

use rustc::lint::*;
use rustc::{declare_lint, lint_array};
use rustc::hir::*;
use crate::utils::span_lint_and_then;

/// **What it does:** Checks for `enum`s with no variants.
///
/// **Why is this bad?** Enum's with no variants should be replaced with `!`,
/// the uninhabited type,
/// or a wrapper around it.
///
/// **Known problems:** None.
///
/// **Example:**
/// ```rust
/// enum Test {}
/// ```
declare_clippy_lint! {
    pub EMPTY_ENUM,
    pedantic,
    "enum with no variants"
}

#[derive(Copy, Clone)]
pub struct EmptyEnum;

impl LintPass for EmptyEnum {
    fn get_lints(&self) -> LintArray {
        lint_array!(EMPTY_ENUM)
    }
}

impl<'a, 'tcx> LateLintPass<'a, 'tcx> for EmptyEnum {
    fn check_item(&mut self, cx: &LateContext, item: &Item) {
        let did = cx.tcx.hir.local_def_id(item.id);
        if let ItemKind::Enum(..) = item.node {
            let ty = cx.tcx.type_of(did);
            let adt = ty.ty_adt_def()
                .expect("already checked whether this is an enum");
            if adt.variants.is_empty() {
                span_lint_and_then(cx, EMPTY_ENUM, item.span, "enum with no variants", |db| {
                    db.span_help(item.span, "consider using the uninhabited type `!` or a wrapper around it");
                });
            }
        }
    }
}
