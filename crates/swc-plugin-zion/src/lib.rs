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

const COVERAGE_GLOBAL: &'static str = "globalThis";
const COVERAGE_KEY: &'static str = "__ZION_COVERAGE__";

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

    fn visit_mut_module(&mut self, module: &mut Module) {
        module.visit_mut_children_with(self);

        module.body.insert(
            0,
            ModuleItem::Stmt(Stmt::Decl(Decl::Var(Box::new(VarDecl {
                span: DUMMY_SP,
                kind: VarDeclKind::Const,
                decls: vec![VarDeclarator {
                    span: DUMMY_SP,
                    definite: false,
                    init: Some(Box::new(Expr::Object(ObjectLit {
                        span: DUMMY_SP,
                        props: vec![],
                    }))),
                    name: Pat::Ident(BindingIdent {
                        id: Ident {
                            sym: "coverage".into(),
                            optional: false,
                            span: DUMMY_SP,
                        },
                        type_ann: None,
                    }),
                }],
                declare: false,
            })))),
        );

        module.body.push(ModuleItem::Stmt(Stmt::Expr(ExprStmt {
            span: DUMMY_SP,
            expr: Box::new(Expr::Assign(AssignExpr {
                span: DUMMY_SP,
                op: AssignOp::Assign,
                left: AssignTarget::Simple(SimpleAssignTarget::Member(MemberExpr {
                    span: DUMMY_SP,
                    obj: Box::new(Expr::Ident(Ident {
                        span: DUMMY_SP,
                        optional: false,
                        sym: COVERAGE_GLOBAL.into(),
                    })),
                    prop: MemberProp::Ident(Ident {
                        span: DUMMY_SP,
                        optional: false,
                        sym: COVERAGE_KEY.into(),
                    }),
                })),
                right: Box::new(Expr::Ident(Ident {
                    span: DUMMY_SP,
                    optional: false,
                    sym: "coverage".into(),
                })),
            })),
        })));
    }

    fn visit_mut_fn_decl(&mut self, fn_decl: &mut FnDecl) {
        fn_decl.visit_mut_children_with(self);

        if let Some(block_stmt) = &mut fn_decl.function.body {
            block_stmt.stmts.insert(
                0,
                Stmt::Expr(ExprStmt {
                    span: DUMMY_SP,
                    expr: Box::new(Expr::Assign(AssignExpr {
                        span: DUMMY_SP,
                        op: AssignOp::Assign,
                        left: AssignTarget::Simple(SimpleAssignTarget::Member(MemberExpr {
                            span: DUMMY_SP,
                            obj: Box::new(Expr::Ident(Ident {
                                span: DUMMY_SP,
                                optional: false,
                                sym: "coverage".into(),
                            })),
                            prop: MemberProp::Ident(Ident {
                                span: DUMMY_SP,
                                optional: false,
                                sym: "add".into(),
                            }),
                        })),
                        right: Box::new(Expr::Lit(Lit::Num(Number {
                            span: DUMMY_SP,
                            value: 1.0,
                            raw: None,
                        }))),
                    })),
                }),
            )
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
    module_coverage,
    // Input
    r#"
function add(a, b) {
    return a + b;
}
"#,
    // Output
    r#"
const coverage = {};
function add(a, b) {
    coverage.add = 1;
    return a + b;
}
globalThis.__VITEST_COVERAGE__ = coverage;
"#
);
