use std::ops::Deref;

use serde::{Deserialize, Serialize};
use swc_core::{atoms::Atom, common::DUMMY_SP, ecma::ast::*};

pub trait ToAst {
    type Node;

    fn to_ast_node(&self) -> Self::Node;
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Loc {
    start: i32,
    end: i32,
}

impl ToAst for Loc {
    type Node = ObjectLit;

    fn to_ast_node(&self) -> Self::Node {
        Self::Node {
            span: DUMMY_SP,
            props: to_obj_lit(vec![
                (
                    "start",
                    Expr::Lit(Lit::Num(Number {
                        span: DUMMY_SP,
                        value: self.start.into(),
                        raw: None,
                    })),
                ),
                (
                    "end",
                    Expr::Lit(Lit::Num(Number {
                        span: DUMMY_SP,
                        value: self.end.into(),
                        raw: None,
                    })),
                ),
            ]),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct CoverageData {
    #[serde(rename = "stmtMap", default)]
    stmts: Stmts,

    #[serde(rename = "fnMap", default)]
    fns: Fns,

    #[serde(rename = "branchMap", default)]
    branches: Branches,
}

impl CoverageData {
    pub fn new() -> Self {
        Self::default()
    }
}

fn to_obj_lit<T, K, V>(vec: T) -> Vec<PropOrSpread>
where
    T: IntoIterator<Item = (K, V)> + AsRef<[(K, V)]>,
    K: Into<Atom>,
    V: Into<Expr>,
{
    vec.into_iter()
        .map(|(key, value)| KeyValueProp {
            key: PropName::Ident(Ident {
                span: DUMMY_SP,
                sym: key.into(),
                optional: false,
            }),
            value: value.into().into(),
        })
        .map(|prop| PropOrSpread::Prop(Prop::KeyValue(prop).into()))
        .collect()
}

fn to_array_lit<T, U>(vec: U) -> Vec<Option<ExprOrSpread>>
where
    T: ToAst,
    T::Node: Into<Box<Expr>>,
    U: AsRef<Vec<T>>,
{
    vec.as_ref()
        .iter()
        .map(|e| e.to_ast_node())
        .map(|e| ExprOrSpread {
            spread: None,
            expr: e.into(),
        })
        .map(|e| Some(e))
        .collect()
}

impl ToAst for CoverageData {
    type Node = ObjectLit;

    fn to_ast_node(&self) -> Self::Node {
        ObjectLit {
            span: DUMMY_SP,
            props: to_obj_lit(vec![
                ("stmts", self.stmts.to_ast_node()),
                ("fns", self.fns.to_ast_node()),
                ("branches", self.branches.to_ast_node()),
            ]),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct Stmts(Vec<StmtMeta>);

impl From<Vec<StmtMeta>> for Stmts {
    fn from(vec: Vec<StmtMeta>) -> Self {
        Stmts(vec)
    }
}

impl Deref for Stmts {
    type Target = Vec<StmtMeta>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl ToAst for Stmts {
    type Node = ArrayLit;

    fn to_ast_node(&self) -> Self::Node {
        Self::Node {
            span: DUMMY_SP,
            elems: to_array_lit(&**self),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct StmtMeta(Loc);

impl ToAst for StmtMeta {
    type Node = ObjectLit;

    fn to_ast_node(&self) -> Self::Node {
        self.0.to_ast_node()
    }
}

impl Deref for StmtMeta {
    type Target = Loc;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct Fns(Vec<FnMeta>);

impl ToAst for Fns {
    type Node = ArrayLit;

    fn to_ast_node(&self) -> Self::Node {
        Self::Node {
            span: DUMMY_SP,
            elems: to_array_lit(&**self),
        }
    }
}

impl Deref for Fns {
    type Target = Vec<FnMeta>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<Vec<FnMeta>> for Fns {
    fn from(vec: Vec<FnMeta>) -> Self {
        Fns(vec)
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct FnMeta {
    name: Option<String>,
    decl: Loc,
    loc: Loc,
}

impl ToAst for FnMeta {
    type Node = ObjectLit;

    fn to_ast_node(&self) -> Self::Node {
        ObjectLit {
            span: DUMMY_SP,
            props: to_obj_lit(vec![
                (
                    "name",
                    match &self.name {
                        Some(name) => Expr::Lit(Lit::Str(Str {
                            span: DUMMY_SP,
                            value: name.clone().into(),
                            raw: None,
                        })),
                        None => Expr::Ident(Ident {
                            span: DUMMY_SP,
                            optional: false,
                            sym: "undefined".into(),
                        }),
                    },
                ),
                ("decl", self.decl.to_ast_node().into()),
                ("loc", self.decl.to_ast_node().into()),
            ]),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct Branches(Vec<BranchMeta>);

impl From<Vec<BranchMeta>> for Branches {
    fn from(vec: Vec<BranchMeta>) -> Self {
        Branches(vec)
    }
}

impl Deref for Branches {
    type Target = Vec<BranchMeta>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl ToAst for Branches {
    type Node = ArrayLit;

    fn to_ast_node(&self) -> Self::Node {
        Self::Node {
            span: DUMMY_SP,
            elems: to_array_lit(&**self),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct BranchMeta {
    r#type: BranchType,
    loc: Loc,
}

impl ToAst for BranchMeta {
    type Node = ObjectLit;

    fn to_ast_node(&self) -> Self::Node {
        ObjectLit {
            span: DUMMY_SP,
            props: to_obj_lit(vec![
                ("type", self.r#type.to_ast_node()),
                ("loc", self.loc.to_ast_node().into()),
            ]),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum BranchType {
    #[serde(rename = "if")]
    If,

    #[serde(rename = "else")]
    Else,
}

impl ToAst for BranchType {
    type Node = Expr;

    fn to_ast_node(&self) -> Self::Node {
        Lit::Str(Str {
            span: DUMMY_SP,
            raw: None,
            value: match self {
                Self::If => "if".into(),
                Self::Else => "else".into(),
            },
        })
        .into()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use insta::{assert_json_snapshot, assert_snapshot};
    use std::{error::Error, sync::Arc};

    use swc_core::{
        common::SourceMap,
        ecma::codegen::{text_writer::JsWriter, Emitter},
    };

    #[test]
    fn serialize() -> Result<(), Box<dyn Error>> {
        let coverage_data = CoverageData {
            stmts: vec![StmtMeta(Loc { start: 0, end: 12 })].into(),
            fns: vec![FnMeta {
                name: String::from("fnName").into(),
                loc: Loc { start: 0, end: 12 },
                decl: Loc { start: 0, end: 12 },
            }]
            .into(),
            branches: vec![BranchMeta {
                r#type: BranchType::If,
                loc: Loc { start: 0, end: 12 },
            }]
            .into(),
        };

        let serialized = serde_json::to_string_pretty(&coverage_data)?;

        assert_snapshot!(serialized, @r###"
        {
          "stmtMap": [
            {
              "start": 0,
              "end": 12
            }
          ],
          "fnMap": [
            {
              "name": "fnName",
              "decl": {
                "start": 0,
                "end": 12
              },
              "loc": {
                "start": 0,
                "end": 12
              }
            }
          ],
          "branchMap": [
            {
              "type": "if",
              "loc": {
                "start": 0,
                "end": 12
              }
            }
          ]
        }
        "###);

        Ok(())
    }

    #[test]
    fn deserialize() -> Result<(), Box<dyn Error>> {
        let json = r###"
        {
          "stmtMap": [
            {
              "start": 0,
              "end": 12
            }
          ],
          "fnMap": [
            {
              "name": "fnName",
              "decl": {
                "start": 0,
                "end": 12
              },
              "loc": {
                "start": 0,
                "end": 12
              }
            }
          ],
          "branchMap": [
            {
              "type": "if",
              "loc": {
                "start": 0,
                "end": 12
              }
            }
          ]
        }
        "###;

        let deserialized = serde_json::from_str::<CoverageData>(&json)?;

        assert_eq!(
            deserialized,
            CoverageData {
                stmts: vec![StmtMeta(Loc { start: 0, end: 12 })].into(),
                fns: vec![FnMeta {
                    name: Some(String::from("fnName")),
                    loc: Loc { start: 0, end: 12 },
                    decl: Loc { start: 0, end: 12 },
                }]
                .into(),
                branches: vec![BranchMeta {
                    r#type: BranchType::If,
                    loc: Loc { start: 0, end: 12 },
                }]
                .into(),
            }
        );

        Ok(())
    }

    #[test]
    fn default() {
        let default_coverage_data = CoverageData::new();

        assert_json_snapshot!(default_coverage_data, @r###"
        {
          "stmtMap": [],
          "fnMap": [],
          "branchMap": []
        }
        "###);
    }

    #[test]
    fn generates_object() {
        let coverage_data = CoverageData {
            stmts: vec![StmtMeta(Loc { start: 0, end: 12 })].into(),
            fns: vec![FnMeta {
                name: Some(String::from("fnName")),
                loc: Loc { start: 0, end: 12 },
                decl: Loc { start: 0, end: 12 },
            }]
            .into(),
            branches: vec![BranchMeta {
                r#type: BranchType::If,
                loc: Loc { start: 0, end: 12 },
            }]
            .into(),
        };

        let module = Module {
            span: DUMMY_SP,
            shebang: None,
            body: vec![Stmt::Expr(ExprStmt {
                span: DUMMY_SP,
                expr: coverage_data.to_ast_node().into(),
            })
            .into()],
        };

        let cm = Arc::new(SourceMap::default());

        let code = {
            let mut buf = Vec::new();

            {
                let mut emitter = Emitter {
                    cfg: Default::default(),
                    comments: None,
                    cm: cm.clone(),
                    wr: JsWriter::new(cm, "\n", &mut buf, None),
                };

                emitter.emit_module(&module).unwrap();
            }

            String::from_utf8_lossy(&buf).to_string()
        };

        assert_snapshot!(code, @r###"
        {
            stmts: [
                {
                    start: 0,
                    end: 12
                }
            ],
            fns: [
                {
                    name: "fnName",
                    decl: {
                        start: 0,
                        end: 12
                    },
                    loc: {
                        start: 0,
                        end: 12
                    }
                }
            ],
            branches: [
                {
                    type: "if",
                    loc: {
                        start: 0,
                        end: 12
                    }
                }
            ]
        };
        "###)
    }
}
