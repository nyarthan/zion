use std::ops::Deref;

use typegen_rs::CoverageData;

pub struct CoverageDataInject(pub CoverageData);

impl CoverageDataInject {
    pub fn new() -> Self {
        CoverageDataInject(CoverageData {
            stmts: vec![],
            fns: vec![],
            branches: vec![],
        })
    }
}

impl Deref for CoverageDataInject {
    type Target = CoverageData;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
