
#![feature(plugin_registrar)]
#![feature(rustc_private)]



extern crate rustc;
extern crate rustc_plugin;
extern crate syntax;
extern crate syntax_pos;

use rustc::hir::{Body, FnDecl};
use rustc::hir::intravisit::{FnKind};
use syntax_pos::Span;
use syntax::ast::NodeId;
use rustc::lint::{LintPass, LateLintPass, LintContext, LateContext, LintArray, Lint, Level};

use rustc_plugin::Registry;

#[plugin_registrar]
pub fn plugin_registrar(reg: &mut Registry) {
    reg.register_late_lint_pass(Box::new(Pass));
}

struct Pass;

const TEST_LINT: &'static Lint = &Lint {
    name: "test",
    desc: "Some test linter",
    default_level: Level::Warn,
    edition_lint_opts: None,
    report_in_external_macro: true
};

impl LintPass for Pass {
    fn get_lints(&self) -> LintArray {
        vec![TEST_LINT]
    }
}

impl<'a, 'tcx> LateLintPass<'a,'tcx> for Pass {
    fn check_fn(
        &mut self, 
        ctx: &LateContext<'a, 'tcx>, 
        func: FnKind<'tcx>, 
        _decl: &'tcx FnDecl, 
        body: &'tcx Body, 
        span: Span, 
        _: NodeId
    ) { 
        let _attrs = match func {
            FnKind::ItemFn(name, _gen, _sig, _vis, attrs) => { println!("ItemFn {0:?}", name); attrs } ,
            FnKind::Method(ident, _sig, _vis, attrs) => { println!("Method {0:?}", ident); attrs },
            FnKind::Closure(..) => return (),
        };

        ctx.span_lint(TEST_LINT, span, "Test msg");

        let expr = &body.value;

        println!("{0:?}", expr);
     }
}