use misc::gen_coverage_symbol;
use serde::Deserialize;
use swc_core::ecma::{
    ast::Program,
    transforms::testing::test_inline,
    visit::{as_folder, FoldWith},
};
use swc_core::plugin::{plugin_transform, proxies::TransformPluginProgramMetadata};

mod ast;
mod coverage;
mod misc;
mod visitor;

use visitor::{source::SourceTransformVisitor, test::TestTransformVisitor};

#[derive(Deserialize)]
enum PluginConfigType {
    #[serde(rename = "test")]
    Test,

    #[serde(rename = "source")]
    Source,
}

#[derive(Deserialize)]
struct PluginConfig {
    r#type: PluginConfigType,
}

#[plugin_transform]
pub fn process_transform(program: Program, metadata: TransformPluginProgramMetadata) -> Program {
    let config: PluginConfig =
        serde_json::from_str(&metadata.get_transform_plugin_config().unwrap()).unwrap();

    match config.r#type {
        PluginConfigType::Source => program.fold_with(&mut as_folder(SourceTransformVisitor {
            module_coverage_sym: format!("coverage_{}", gen_coverage_symbol("this is a test")),
        })),
        PluginConfigType::Test => program.fold_with(&mut as_folder(TestTransformVisitor {})),
    }
}

test_inline!(
    Default::default(),
    |_| as_folder(SourceTransformVisitor {
        module_coverage_sym: String::from("coverage")
    }),
    module_coverage,
    // Input
    r#"
function add(a, b) {
    return a + b;
}
"#,
    // Output
    r#"
const coverage = {};
function add(a, b) {
    coverage.add = (coverage.add ?? 0) + 1;
    return a + b;
}
globalThis.__ZION_COVERAGE__ = coverage;
"#
);
