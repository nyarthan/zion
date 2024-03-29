use serde::{Deserialize, Serialize};
use typify::import_types;

import_types!(schema = "./schemas/coverage-data.json", derives = [Debug]);

#[cfg(test)]
mod test {
    use insta::assert_snapshot;

    use super::*;

    #[test]
    fn does_something() {
        let coverage_data = CoverageData {
            stmts: vec![],
            fns: vec![],
            branches: vec![],
        };

        let blah = format!("{:#?}", coverage_data);

        assert_snapshot!(blah, @r###"
        CoverageData {
            branches: [],
            fns: [],
            stmts: [],
        }
        "###);
    }
}
