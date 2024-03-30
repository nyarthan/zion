use std::sync::Arc;

use swc_core::{
    common::{SourceMap, DUMMY_SP},
    ecma::{
        ast::*,
        codegen::{text_writer::JsWriter, Emitter},
    },
};

pub fn to_js_code(expr: Expr) -> String {
    let module = Module {
        span: DUMMY_SP,
        shebang: None,
        body: vec![Stmt::Expr(ExprStmt {
            span: DUMMY_SP,
            expr: expr.into(),
        })
        .into()],
    };

    let cm = Arc::new(SourceMap::default());

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
}
