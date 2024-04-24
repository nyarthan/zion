use insta::assert_snapshot;
use swc_ast_derive::ToAstStruct;
use swc_ast_trait::ToAst;
use swc_core::{common::DUMMY_SP, ecma::ast::*};
use swc_helpers::to_js_code;

#[test]
fn works() {
    #[derive(ToAstStruct)]
    struct Test {
        string: String,
        number: i32,
        bool: bool,
        vec: Vec<bool>,
        nested: Nested,
        none: Option<String>,
        some: Option<String>
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
        none: None,
        some: Some(String::from("string"))
    };
    let code = to_js_code(source.to_ast_node());

    assert_snapshot!(code, @r###"
    {
        string: "string",
        number: 32,
        bool: true,
        vec: [
            true
        ],
        nested: {
            string: "string"
        },
        none: undefined,
        some: "string"
    };
    "###);
}
