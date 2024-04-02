use std::collections::HashSet;

use crate::visitor::pat_ident_access::PatIdentAccess;

use lazy_static::lazy_static;
use swc_core::{
    common::DUMMY_SP,
    ecma::{
        ast::*,
        utils::prepend_stmt,
        visit::{VisitMut, VisitMutWith},
    },
};

lazy_static! {
    static ref TEST_FN_IDENTS: HashSet<&'static str> = HashSet::from(["it", "test"]);
}

pub struct TestTransformVisitor {
    // tests: Vec<String>,
    //
}

impl VisitMut for TestTransformVisitor {
    fn visit_mut_call_expr(&mut self, call_expr: &mut CallExpr) {
        call_expr.visit_mut_children_with(self);

        if !is_test_expr(call_expr) {
            return;
        };

        if let Some(ExprOrSpread { expr, .. }) = call_expr.args.get_mut(1) {
            if let Expr::Arrow(arrow_expr) = &mut **expr {
                let body = &mut *arrow_expr.body;

                let params = &mut arrow_expr.params;
                if params.is_empty() {
                    params.push(Pat::Object(ObjectPat {
                        span: DUMMY_SP,
                        props: vec![ObjectPatProp::Assign(AssignPatProp {
                            span: DUMMY_SP,
                            value: None,
                            key: BindingIdent {
                                id: Ident {
                                    span: DUMMY_SP,
                                    optional: false,
                                    sym: "task".into(),
                                },
                                type_ann: None,
                            },
                        })],
                        optional: false,
                        type_ann: None,
                    }))
                } else {
                    let pat = params.first().unwrap();

                    // TODO: this really needs refactoring
                    match pat.access_pat_ident("task") {
                        Some(expr) => {
                            let declaration = Stmt::Decl(Decl::Var(Box::new(VarDecl {
                                span: DUMMY_SP,
                                kind: VarDeclKind::Const,
                                declare: false,
                                decls: vec![VarDeclarator {
                                    span: DUMMY_SP,
                                    name: Pat::Ident(BindingIdent {
                                        type_ann: None,
                                        id: Ident {
                                            span: DUMMY_SP,
                                            sym: "__task".into(),
                                            optional: false,
                                        },
                                    }),
                                    definite: false,
                                    init: Some(Box::new(expr)),
                                }],
                            })));

                            if let BlockStmtOrExpr::BlockStmt(BlockStmt { stmts, .. }) = body {
                                prepend_stmt(stmts, declaration)
                            }
                        }
                        None => {
                            if let Pat::Rest(RestPat { arg, .. }) = pat {
                                let ident = Ident {
                                    span: DUMMY_SP,
                                    optional: false,
                                    sym: "args".into(),
                                };

                                let mut params = params.clone();
                                params.clear();
                                params.push(Pat::Rest(RestPat {
                                    type_ann: None,
                                    span: DUMMY_SP,
                                    dot3_token: DUMMY_SP,
                                    arg: Box::new(Pat::Ident(ident.clone().into())),
                                }));

                                if let Pat::Array(array_pat) = &**arg {
                                    let decl = VarDecl {
                                        span: DUMMY_SP,
                                        kind: VarDeclKind::Const,
                                        decls: vec![VarDeclarator {
                                            span: DUMMY_SP,
                                            init: Some(Box::new(Expr::Ident(ident.clone()))),
                                            definite: false,
                                            name: Pat::Array(array_pat.clone()),
                                        }],
                                        declare: false,
                                    };

                                    let declaration = Stmt::Decl(Decl::Var(Box::new(VarDecl {
                                        span: DUMMY_SP,
                                        kind: VarDeclKind::Const,
                                        declare: false,
                                        decls: vec![VarDeclarator {
                                            span: DUMMY_SP,
                                            name: Pat::Ident(BindingIdent {
                                                type_ann: None,
                                                id: Ident {
                                                    span: DUMMY_SP,
                                                    sym: "__task".into(),
                                                    optional: false,
                                                },
                                            }),
                                            definite: false,
                                            init: Some(Box::new(Expr::Member(MemberExpr {
                                                span: DUMMY_SP,
                                                obj: Box::new(Expr::Member(MemberExpr {
                                                    span: DUMMY_SP,
                                                    obj: Box::new(Expr::Ident(ident)),
                                                    prop: MemberProp::Computed(ComputedPropName {
                                                        span: DUMMY_SP,
                                                        expr: Box::new(Expr::Lit(Lit::Num(
                                                            Number {
                                                                span: DUMMY_SP,
                                                                value: 0.0,
                                                                raw: None,
                                                            },
                                                        ))),
                                                    }),
                                                })),
                                                prop: MemberProp::Ident(Ident {
                                                    span: DUMMY_SP,
                                                    sym: "task".into(),
                                                    optional: false,
                                                }),
                                            }))),
                                        }],
                                    })));

                                    if let BlockStmtOrExpr::BlockStmt(BlockStmt { stmts, .. }) =
                                        body
                                    {
                                        prepend_stmt(stmts, Stmt::Decl(decl.into()));
                                        prepend_stmt(stmts, declaration);
                                    }

                                    arrow_expr.params = params;
                                }
                            } else {
                                let ident = Ident {
                                    span: DUMMY_SP,
                                    optional: false,
                                    sym: "arg".into(),
                                };
                                let mut p = params.clone();
                                p.remove(0);
                                p.insert(0, Pat::Ident(ident.clone().into()));

                                let decl = VarDecl {
                                    span: DUMMY_SP,
                                    kind: VarDeclKind::Const,
                                    declare: false,
                                    decls: vec![VarDeclarator {
                                        span: DUMMY_SP,
                                        name: pat.clone(),
                                        definite: false,
                                        init: Some(Box::new(Expr::Ident(ident.clone()))),
                                    }],
                                };
                                let declaration = Stmt::Decl(Decl::Var(Box::new(VarDecl {
                                    span: DUMMY_SP,
                                    kind: VarDeclKind::Const,
                                    declare: false,
                                    decls: vec![VarDeclarator {
                                        span: DUMMY_SP,
                                        name: Pat::Ident(BindingIdent {
                                            type_ann: None,
                                            id: Ident {
                                                span: DUMMY_SP,
                                                sym: "__task".into(),
                                                optional: false,
                                            },
                                        }),
                                        definite: false,
                                        init: Some(Box::new(Expr::Member(MemberExpr {
                                            span: DUMMY_SP,
                                            obj: Box::new(Expr::Ident(ident)),
                                            prop: MemberProp::Ident(Ident {
                                                span: DUMMY_SP,
                                                optional: false,
                                                sym: "task".into(),
                                            }),
                                        }))),
                                    }],
                                })));

                                if let BlockStmtOrExpr::BlockStmt(BlockStmt { stmts, .. }) = body {
                                    prepend_stmt(stmts, Stmt::Decl(decl.into()));
                                    prepend_stmt(stmts, declaration);
                                }
                                arrow_expr.params = p;
                            }
                        }
                    }
                }
            }
        }
    }
}

fn is_test_expr(call_expr: &CallExpr) -> bool {
    if let Callee::Expr(expr) = &call_expr.callee {
        if let Expr::Ident(ident) = &**expr {
            return TEST_FN_IDENTS.contains(&ident.sym.as_str());
        };
    };

    false
}
