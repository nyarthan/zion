use serde::{Deserialize, Serialize};
use swc_ast_derive::ToAstStruct;
use swc_ast_trait::ToAst;
use swc_core::{common::DUMMY_SP, ecma::ast::*};
use typify::import_types;

import_types!(
    schema = "./schemas/coverage-data.json",
    derives = [Debug, ToAstStruct]
);

#[cfg(test)]
mod test {
    use super::*;

    use swc_helpers::to_js_code;

    use insta::assert_snapshot;

    #[test]
    fn does_something() {
        let coverage_data = CoverageData {
            stmts: vec![CoverageDataStmtsItem {
                start: 1.0,
                end: 1.0,
            }],
            fns: vec![CoverageDataFnsItem {
                name: Some(String::from("fn")),
                loc: CoverageDataFnsItemLoc {
                    end: 1.0,
                    start: 1.0,
                },
                decl: CoverageDataFnsItemDecl {
                    start: 1.0,
                    end: 1.0,
                },
            }],
            branches: vec![CoverageDataBranchesItem {
                loc: CoverageDataBranchesItemLoc {
                    start: 1.0,
                    end: 1.0,
                },
                type_: CoverageDataBranchesItemType::If,
            }],
        };

        let code = to_js_code(coverage_data.to_ast_node());

        assert_snapshot!(code, @r###"
        {
            branches: [
                {
                    loc: {
                        end: 1,
                        start: 1
                    },
                    type_: "if"
                }
            ],
            fns: [
                {
                    decl: {
                        end: 1,
                        start: 1
                    },
                    loc: {
                        end: 1,
                        start: 1
                    },
                    name: "fn"
                }
            ],
            stmts: [
                {
                    end: 1,
                    start: 1
                }
            ]
        };
        "###);
    }
}
