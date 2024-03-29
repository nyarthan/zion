use misc::gen_coverage_symbol;
use swc_core::ecma::utils::prepend_stmt;
use swc_core::plugin::{plugin_transform, proxies::TransformPluginProgramMetadata};
use swc_core::{
    common::DUMMY_SP,
    ecma::{
        ast::*,
        transforms::testing::test_inline,
        visit::{as_folder, FoldWith, VisitMut, VisitMutWith},
    },
};

mod coverage;
mod misc;

pub struct TransformVisitor {
    module_coverage_sym: String,
}

const COVERAGE_GLOBAL: &'static str = "globalThis";
const COVERAGE_KEY: &'static str = "__ZION_COVERAGE__";

impl VisitMut for TransformVisitor {
    fn visit_mut_module(&mut self, module: &mut Module) {
        module.visit_mut_children_with(self);

        prepend_stmt(
            &mut module.body,
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
                            sym: self.module_coverage_sym.clone().into(),
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
                op: op!("="),
                left: AssignTarget::Simple(SimpleAssignTarget::Member(MemberExpr {
                    span: DUMMY_SP,
                    obj: ident_expr!(COVERAGE_GLOBAL),
                    prop: MemberProp::Ident(Ident {
                        span: DUMMY_SP,
                        optional: false,
                        sym: COVERAGE_KEY.into(),
                    }),
                })),
                right: ident_expr!(self.module_coverage_sym.clone()),
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
                        op: op!("="),
                        left: AssignTarget::Simple(SimpleAssignTarget::Member(MemberExpr {
                            span: DUMMY_SP,
                            obj: ident_expr!(self.module_coverage_sym.clone()),
                            prop: MemberProp::Ident(Ident {
                                span: DUMMY_SP,
                                optional: false,
                                sym: fn_decl.ident.sym.clone(),
                            }),
                        })),
                        right: Box::new(Expr::Bin(BinExpr {
                            span: DUMMY_SP,
                            op: op!(bin, "+"),
                            left: Box::new(Expr::Paren(ParenExpr {
                                span: DUMMY_SP,
                                expr: Box::new(Expr::Bin(BinExpr {
                                    span: DUMMY_SP,
                                    op: op!("??"),
                                    left: Box::new(Expr::Member(MemberExpr {
                                        span: DUMMY_SP,
                                        obj: ident_expr!(self.module_coverage_sym.clone()),
                                        prop: MemberProp::Ident(Ident {
                                            span: DUMMY_SP,
                                            optional: false,
                                            sym: fn_decl.ident.sym.clone(),
                                        }),
                                    })),
                                    right: num_lit_expr!(0.),
                                })),
                            })),
                            right: num_lit_expr!(1.),
                        })),
                    })),
                }),
            )
        }
    }
}

#[macro_export]
macro_rules! num_lit_expr {
    ($value:expr) => {
        Box::new(Expr::Lit(Lit::Num(Number {
            span: DUMMY_SP,
            value: $value,
            raw: None,
        })))
    };
}

#[macro_export]
macro_rules! ident_expr {
    ($value:expr) => {
        Box::new(Expr::Ident(Ident {
            span: DUMMY_SP,
            optional: false,
            sym: $value.into(),
        }))
    };
}

#[macro_export]
macro_rules! empty_coverage_data {
    ($value:expr) => {
        quote!(
            r#"
{
    modulePath: '/path/to/module.js',
    stmtMap: {},
    fnMap: {},
    branchMap: {},
    coverage: {
        stmt: {},
        fn: {},
        branch: {}
    }
}
"# as Expr
        )
    };
}

#[plugin_transform]
pub fn process_transform(program: Program, _metadata: TransformPluginProgramMetadata) -> Program {
    program.fold_with(&mut as_folder(TransformVisitor {
        module_coverage_sym: format!("coverage_{}", gen_coverage_symbol("this is a test")),
    }))
}

test_inline!(
    Default::default(),
    |_| as_folder(TransformVisitor {
        module_coverage_sym: String::from("coverage")
    }),
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
    coverage.add = (coverage.add ?? 0) + 1;
    return a + b;
}
globalThis.__ZION_COVERAGE__ = coverage;
"#
);
