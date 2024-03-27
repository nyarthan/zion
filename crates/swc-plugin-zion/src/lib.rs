use swc_core::plugin::{plugin_transform, proxies::TransformPluginProgramMetadata};
use swc_core::{
    common::DUMMY_SP,
    ecma::{
        ast::*,
        transforms::testing::test_inline,
        visit::{as_folder, FoldWith, VisitMut, VisitMutWith},
    },
};

pub struct TransformVisitor;

impl VisitMut for TransformVisitor {
    fn visit_mut_arrow_expr(&mut self, arrow_expr: &mut ArrowExpr) {
        arrow_expr.visit_mut_children_with(self);

        if let BlockStmtOrExpr::BlockStmt(ret) = &*arrow_expr.body {
            let mut stmts = Vec::new();

            stmts.push(Stmt::Expr(ExprStmt {
                span: DUMMY_SP,
                expr: Box::new(Expr::Call(CallExpr {
                    callee: Callee::Expr(Box::new(Expr::Member(MemberExpr {
                        obj: Box::new(Expr::Ident(Ident {
                            sym: "console".into(),
                            optional: false,
                            span: DUMMY_SP,
                        })),
                        prop: MemberProp::Ident(Ident {
                            sym: "log".into(),
                            optional: false,
                            span: DUMMY_SP,
                        }),
                        span: DUMMY_SP,
                    }))),
                    args: vec![ExprOrSpread {
                        spread: None,
                        expr: Box::new(Expr::Lit(Lit::Str(Str {
                            value: "hi".into(),
                            raw: None,
                            span: DUMMY_SP,
                        }))),
                    }],
                    type_args: None,
                    span: DUMMY_SP,
                })),
            }));

            stmts.push(ret.stmts.last().unwrap().clone());

            arrow_expr.body = Box::new(BlockStmtOrExpr::BlockStmt(BlockStmt {
                span: DUMMY_SP,
                stmts,
            }));
        }
    }
}

#[plugin_transform]
pub fn process_transform(program: Program, _metadata: TransformPluginProgramMetadata) -> Program {
    program.fold_with(&mut as_folder(TransformVisitor))
}

test_inline!(
    Default::default(),
    |_| as_folder(TransformVisitor),
    arrow_expr_add_console_log_hi,
    // Input
    r#"
const add = (a, b) => {
    return a + b;
};
"#,
    // Output
    r#"
const add = (a, b) => {
    console.log("hi");
    return a + b;
}
"#
);
