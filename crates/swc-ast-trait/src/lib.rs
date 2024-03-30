use std::collections::{HashMap, HashSet};

use swc_core::{common::DUMMY_SP, ecma::ast::*};

pub trait ToAst {
    fn to_ast_node(&self) -> Expr;
}

impl ToAst for String {
    fn to_ast_node(&self) -> Expr {
        Lit::Str(Str {
            span: DUMMY_SP,
            raw: None,
            value: self.clone().into(),
        })
        .into()
    }
}

impl ToAst for bool {
    fn to_ast_node(&self) -> Expr {
        Lit::Bool(Bool {
            span: DUMMY_SP,
            value: *self,
        })
        .into()
    }
}

trait Numeric {}

impl Numeric for u8 {}
impl Numeric for u16 {}
impl Numeric for u32 {}
impl Numeric for i8 {}
impl Numeric for i16 {}
impl Numeric for i32 {}
impl Numeric for f32 {}
impl Numeric for f64 {}

impl<T> ToAst for T
where
    T: Numeric + Into<f64> + Copy,
{
    fn to_ast_node(&self) -> Expr {
        Lit::Num(Number {
            span: DUMMY_SP,
            raw: None,
            value: (*self).into(),
        })
        .into()
    }
}

impl<T> ToAst for Vec<T>
where
    T: ToAst,
{
    fn to_ast_node(&self) -> Expr {
        Expr::Array(ArrayLit {
            span: DUMMY_SP,
            elems: self
                .iter()
                .map(|e| {
                    Some(ExprOrSpread {
                        spread: None,
                        expr: e.to_ast_node().into(),
                    })
                })
                .collect(),
        })
    }
}

impl<T> ToAst for HashSet<T>
where
    T: ToAst,
{
    fn to_ast_node(&self) -> Expr {
        Expr::Array(ArrayLit {
            span: DUMMY_SP,
            elems: self
                .iter()
                .map(|e| {
                    Some(ExprOrSpread {
                        spread: None,
                        expr: e.to_ast_node().into(),
                    })
                })
                .collect(),
        })
    }
}

impl<V> ToAst for HashMap<String, V>
where
    V: ToAst,
{
    fn to_ast_node(&self) -> Expr {
        Expr::Object(ObjectLit {
            span: DUMMY_SP,
            props: self
                .iter()
                .map(|(k, v)| {
                    PropOrSpread::Prop(
                        Prop::KeyValue(KeyValueProp {
                            key: PropName::Ident(Ident {
                                span: DUMMY_SP,
                                sym: k.clone().into(),
                                optional: false,
                            }),
                            value: v.to_ast_node().into(),
                        })
                        .into(),
                    )
                })
                .collect(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use insta::assert_snapshot;

    use swc_helpers::to_js_code;

    #[test]
    fn string_to_ast() {
        let source = String::from("this is a string");
        let code = to_js_code(source.to_ast_node());

        assert_snapshot!(code, @r###"
        "this is a string";
        "###);
    }

    #[test]
    fn bool_to_ast() {
        let source = true;
        let code = to_js_code(source.to_ast_node());

        assert_snapshot!(code, @r###"
        true;
        "###);
    }

    #[test]
    fn numeric_to_ast() {
        let source_u8: u8 = 1;
        let source_u16: u16 = 1;
        let source_u32: u32 = 1;
        let source_i8: i8 = 1;
        let source_i16: i16 = 1;
        let source_i32: i32 = 1;
        let source_f32: f32 = 1.0;
        let source_f64: f64 = 1.0;

        let code_u8 = to_js_code(source_u8.to_ast_node());
        let node_u8 = source_u8.to_ast_node();

        assert_eq!(node_u8, source_u16.to_ast_node());
        assert_eq!(node_u8, source_u32.to_ast_node());
        assert_eq!(node_u8, source_i8.to_ast_node());
        assert_eq!(node_u8, source_i16.to_ast_node());
        assert_eq!(node_u8, source_i32.to_ast_node());
        assert_eq!(node_u8, source_f32.to_ast_node());
        assert_eq!(node_u8, source_f64.to_ast_node());

        assert_snapshot!(code_u8, @r###"
        1;
        "###);
    }

    #[test]
    fn vec_string_to_ast() {
        let source = vec![String::from("this is a string")];
        let code = to_js_code(source.to_ast_node());

        assert_snapshot!(code, @r###"
        [
            "this is a string"
        ];
        "###);
    }

    #[test]
    fn vec_vec_string_to_ast() {
        let source = vec![vec![String::from("this is a string")]];
        let code = to_js_code(source.to_ast_node());

        assert_snapshot!(code, @r###"
        [
            [
                "this is a string"
            ]
        ];
        "###);
    }

    #[test]
    fn hashset_eq_vec_to_ast() {
        let string = String::from("this is a string");
        let source = HashSet::from([string.clone()]);
        let source2 = vec![string.clone()];

        let code = to_js_code(source.to_ast_node());
        let code2 = to_js_code(source2.to_ast_node());

        assert_eq!(code, code2);
    }

    #[test]
    fn hashmap_string_to_ast() {
        let source = HashMap::from([(String::from("key"), String::from("value"))]);
        let code = to_js_code(source.to_ast_node());

        assert_snapshot!(code, @r###"
        {
            key: "value"
        };
        "###);
    }

    #[test]
    fn hashmap_hashmap_string_to_ast() {
        let source = HashMap::from([(
            String::from("key"),
            HashMap::from([(String::from("key"), String::from("value"))]),
        )]);
        let code = to_js_code(source.to_ast_node());

        assert_snapshot!(code, @r###"
        {
            key: {
                key: "value"
            }
        };
        "###);
    }
}
