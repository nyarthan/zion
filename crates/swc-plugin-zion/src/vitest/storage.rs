use swc_core::{common::DUMMY_SP, ecma::ast::*};

pub fn new_storage_import() -> ModuleItem {
    ModuleItem::ModuleDecl(ModuleDecl::Import(ImportDecl {
        span: DUMMY_SP,
        src: Box::new(String::from("node:async_hooks").into()),
        with: None,
        phase: ImportPhase::Evaluation,
        type_only: false,
        specifiers: vec![ImportSpecifier::Named(ImportNamedSpecifier {
            span: DUMMY_SP,
            local: Ident {
                span: DUMMY_SP,
                sym: "AsyncLocalStorage".into(),
                optional: false,
            },
            imported: None,
            is_type_only: false,
        })],
    }))
}

pub fn storage_member_expr() -> MemberExpr {
    MemberExpr {
        span: DUMMY_SP,
        obj: Expr::Ident(Ident {
            span: DUMMY_SP,
            sym: "globalThis".into(),
            optional: false,
        })
        .into(),
        prop: MemberProp::Ident(Ident {
            span: DUMMY_SP,
            sym: "__ZION_ASYNC_LOCAL_STORAGE__".into(),
            optional: false,
        }),
    }
}

pub fn new_global_storage() -> Stmt {
    Stmt::Expr(ExprStmt {
        span: DUMMY_SP,
        expr: Expr::Assign(AssignExpr {
            span: DUMMY_SP,
            op: AssignOp::NullishAssign,
            left: AssignTarget::Simple(SimpleAssignTarget::Member(storage_member_expr())),
            right: Expr::New(NewExpr {
                span: DUMMY_SP,
                type_args: None,
                args: Some(vec![]),
                callee: Expr::Ident(Ident {
                    span: DUMMY_SP,
                    optional: false,
                    sym: "AsyncLocalStorage".into(),
                })
                .into(),
            })
            .into(),
        })
        .into(),
    })
}
