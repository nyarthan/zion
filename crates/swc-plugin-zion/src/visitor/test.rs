use std::collections::HashSet;

use lazy_static::lazy_static;
use swc_core::{
    common::DUMMY_SP,
    ecma::{
        ast::*,
        utils::prepend_stmt,
        visit::{fields::PropNameField, VisitMut, VisitMutWith},
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
                    let pat = params.first_mut().unwrap();
                    let expr = find_key_in_pat(pat, "task");

                    if let Some(expr) = expr {
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

fn find_key_in_object_pat(object_pat: &ObjectPat, key: &str) -> Option<Expr> {
    let object_pat_prop = object_pat.props.first().unwrap();

    let task_ident = object_pat.props.iter().find_map(|prop| match prop {
        ObjectPatProp::Assign(AssignPatProp {
            key: BindingIdent { id, .. },
            ..
        }) => Some(id),
        ObjectPatProp::KeyValue(KeyValuePatProp {
            key: PropName::Ident(_ident),
            value,
        }) => {
            let pat = &**value;
            match pat {
                Pat::Ident(BindingIdent { id, .. }) => Some(id),
                _ => None,
            }
        }
        _ => None,
    });

    if let Some(task_ident) = task_ident {
        return Some(Expr::Ident(task_ident.clone()));
    }

    let rest_prop_ident = object_pat.props.iter().find_map(|prop| match prop {
        ObjectPatProp::Rest(RestPat { arg, .. }) => {
            if let Pat::Ident(BindingIdent { id, .. }) = &**arg {
                return Some(id);
            };
            None
        }
        _ => None,
    });

    if let Some(ident) = rest_prop_ident {
        return Some(Expr::Member(MemberExpr {
            span: DUMMY_SP,
            obj: ident.clone().into(),
            prop: MemberProp::Ident(Ident {
                span: DUMMY_SP,
                optional: false,
                sym: key.into(),
            }),
        }));
    }

    match object_pat_prop {
        ObjectPatProp::Assign(AssignPatProp { key, .. }) => Some(Expr::Ident(key.id.clone())),
        ObjectPatProp::KeyValue(KeyValuePatProp {
            key: PropName::Ident(ident),
            value,
        }) => {
            if ident.sym != key {
                None
            } else {
                match &**value {
                    Pat::Ident(BindingIdent { id, .. }) => Some(Expr::Ident(id.clone())),
                    _ => None,
                }
            }
        }
        _ => None,
    }
}

fn find_key_in_array_pat(array_pat: &ArrayPat, key: &str) -> Option<Expr> {
    let pat = match array_pat.elems.get(0)? {
        Some(path) => path,
        None => return None,
    };

    match pat {
        Pat::Ident(BindingIdent { id, .. }) => Some(Expr::Member(MemberExpr {
            span: DUMMY_SP,
            obj: Box::new(Expr::Ident(Ident {
                span: DUMMY_SP,
                sym: id.sym.clone(),
                optional: false,
            })),
            prop: MemberProp::Ident(Ident {
                span: DUMMY_SP,
                sym: key.into(),
                optional: false,
            }),
        })),
        Pat::Object(object_pat) => find_key_in_object_pat(object_pat, key),
        Pat::Rest(array_pat) => find_key_in_rest_pat(array_pat, key),
        _ => None,
    }
}

fn find_key_in_rest_pat(rest_pat: &RestPat, key: &str) -> Option<Expr> {
    match &*rest_pat.arg {
        Pat::Ident(binding_ident) => Some(Expr::Member(MemberExpr {
            span: DUMMY_SP,
            obj: Box::new(Expr::Member(MemberExpr {
                span: DUMMY_SP,
                obj: Box::new(Expr::Ident(Ident {
                    span: DUMMY_SP,
                    sym: binding_ident.sym.clone(),
                    optional: false,
                })),
                prop: MemberProp::Computed(ComputedPropName {
                    span: DUMMY_SP,
                    expr: Box::new(Expr::Lit(Lit::Num(Number {
                        span: DUMMY_SP,
                        value: 0.0,
                        raw: None,
                    }))),
                }),
            })),
            prop: MemberProp::Ident(Ident {
                span: DUMMY_SP,
                optional: false,
                sym: key.into(),
            }),
        })),
        Pat::Array(array_pat) => find_key_in_array_pat(array_pat, key),
        _ => None,
    }
}

fn find_key_in_pat(pat: &mut Pat, key: &str) -> Option<Expr> {
    match pat {
        Pat::Ident(BindingIdent { id, .. }) => Some(Expr::Member(MemberExpr {
            span: DUMMY_SP,
            obj: Expr::Ident(id.clone()).into(),
            prop: MemberProp::Ident(Ident {
                span: DUMMY_SP,
                sym: key.into(),
                optional: false,
            }),
        })),
        Pat::Assign(AssignPat { left: pat, .. }) => match &**pat {
            Pat::Object(object_pat) => find_key_in_object_pat(object_pat, key),
            Pat::Ident(BindingIdent { id, .. }) => Some(Expr::Ident(id.clone())),
            _ => None,
        },
        Pat::Object(object_pat) => find_key_in_object_pat(object_pat, key),
        Pat::Rest(rest_pat) => find_key_in_rest_pat(rest_pat, key),
        _ => None,
    }
}
