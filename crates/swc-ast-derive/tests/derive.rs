use insta::assert_snapshot;
use swc_ast_derive::ToAstStruct;
use swc_ast_trait::ToAst;
use swc_core::{common::DUMMY_SP, ecma::ast::*};

#[test]
fn works() {
    #[derive(ToAstStruct)]
    struct Test {
        string: String,
        number: i32,
        bool: bool,
        vec: Vec<bool>,
        nested: Nested,
    }

    #[derive(ToAstStruct)]
    struct Nested {
        string: String,
    }

    let source = Test {
        string: String::from("string"),
        number: 32,
        bool: true,
        vec: vec![true],
        nested: Nested {
            string: String::from("string"),
        },
    };
    let node = source.to_ast_node();

    assert_snapshot!(format!("{:#?}", node), @r###"
    Object(
        ObjectLit {
            span: 0..0#0,
            props: [
                Prop(
                    KeyValue(
                        KeyValueProp {
                            key: Ident(
                                Ident {
                                    span: 0..0#0,
                                    sym: "string",
                                    optional: false,
                                },
                            ),
                            value: Lit(
                                Str(
                                    Str {
                                        span: 0..0#0,
                                        value: "string",
                                        raw: None,
                                    },
                                ),
                            ),
                        },
                    ),
                ),
                Prop(
                    KeyValue(
                        KeyValueProp {
                            key: Ident(
                                Ident {
                                    span: 0..0#0,
                                    sym: "number",
                                    optional: false,
                                },
                            ),
                            value: Lit(
                                Num(
                                    Number {
                                        span: 0..0#0,
                                        value: 32.0,
                                        raw: None,
                                    },
                                ),
                            ),
                        },
                    ),
                ),
                Prop(
                    KeyValue(
                        KeyValueProp {
                            key: Ident(
                                Ident {
                                    span: 0..0#0,
                                    sym: "bool",
                                    optional: false,
                                },
                            ),
                            value: Lit(
                                Bool(
                                    Bool {
                                        span: 0..0#0,
                                        value: true,
                                    },
                                ),
                            ),
                        },
                    ),
                ),
                Prop(
                    KeyValue(
                        KeyValueProp {
                            key: Ident(
                                Ident {
                                    span: 0..0#0,
                                    sym: "vec",
                                    optional: false,
                                },
                            ),
                            value: Array(
                                ArrayLit {
                                    span: 0..0#0,
                                    elems: [
                                        Some(
                                            ExprOrSpread {
                                                spread: None,
                                                expr: Lit(
                                                    Bool(
                                                        Bool {
                                                            span: 0..0#0,
                                                            value: true,
                                                        },
                                                    ),
                                                ),
                                            },
                                        ),
                                    ],
                                },
                            ),
                        },
                    ),
                ),
                Prop(
                    KeyValue(
                        KeyValueProp {
                            key: Ident(
                                Ident {
                                    span: 0..0#0,
                                    sym: "nested",
                                    optional: false,
                                },
                            ),
                            value: Object(
                                ObjectLit {
                                    span: 0..0#0,
                                    props: [
                                        Prop(
                                            KeyValue(
                                                KeyValueProp {
                                                    key: Ident(
                                                        Ident {
                                                            span: 0..0#0,
                                                            sym: "string",
                                                            optional: false,
                                                        },
                                                    ),
                                                    value: Lit(
                                                        Str(
                                                            Str {
                                                                span: 0..0#0,
                                                                value: "string",
                                                                raw: None,
                                                            },
                                                        ),
                                                    ),
                                                },
                                            ),
                                        ),
                                    ],
                                },
                            ),
                        },
                    ),
                ),
            ],
        },
    )
    "###);
}
