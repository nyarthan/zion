use crate::{ident_expr, num_lit_expr};
use swc_ast_trait::ToAst;
use swc_core::{
    common::DUMMY_SP,
    ecma::{
        ast::*,
        utils::prepend_stmt,
        visit::{VisitMut, VisitMutWith},
    },
};

use crate::coverage::CoverageDataInject;

pub(crate) struct SourceTransformVisitor {
    pub module_coverage_sym: String,
}

const COVERAGE_GLOBAL: &'static str = "globalThis";
const COVERAGE_KEY: &'static str = "__ZION_COVERAGE__";

impl VisitMut for SourceTransformVisitor {
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
                    init: Some(Box::new(CoverageDataInject::new().to_ast_node())),
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
