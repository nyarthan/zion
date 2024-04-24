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
