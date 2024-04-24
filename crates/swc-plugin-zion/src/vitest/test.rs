use std::str::FromStr;
use strum_macros::{AsRefStr, EnumString};
use swc_core::ecma::ast::*;

pub fn get_test_expr(call_expr: &CallExpr) -> Option<TestExpr> {
    match &call_expr.callee {
        Callee::Expr(expr) => get_test_call_expr(&expr),
        _ => None,
    }
}

pub fn get_test_call_expr(expr: &Expr) -> Option<TestExpr> {
    match expr {
        Expr::Ident(Ident { sym, .. }) => Some(TestExpr {
            sym: get_test_sym(sym.as_str())?,
            modifiers: vec![],
        }),

        Expr::Member(MemberExpr { obj, prop, .. }) => {
            let modifier = get_modifier_prop(prop)?;
            let base = match &**obj {
                Expr::Ident(Ident { sym, .. }) if get_test_sym(sym.as_str()).is_some() => {
                    TestExpr {
                        sym: get_test_sym(sym.as_str())?,
                        modifiers: vec![modifier],
                    }
                }
                other => {
                    let mut base = get_test_call_expr(other)?;
                    base.modifiers.push(modifier);
                    base
                }
            };
            Some(base)
        }

        Expr::Call(CallExpr {
            callee: Callee::Expr(inner_expr),
            ..
        }) => get_test_call_expr(inner_expr),

        _ => None,
    }
}

#[derive(Debug)]
pub struct TestExpr {
    // TODO
    #[allow(dead_code)]
    pub sym: TestSym,
    pub modifiers: Vec<TestModifer>,
}

#[derive(AsRefStr, EnumString, Debug, PartialEq)]
pub enum TestModifer {
    #[strum(ascii_case_insensitive)]
    Concurrent,

    #[strum(ascii_case_insensitive)]
    Sequential,

    #[strum(ascii_case_insensitive)]
    Only,

    #[strum(ascii_case_insensitive)]
    Skip,

    #[strum(ascii_case_insensitive)]
    Todo,

    #[strum(ascii_case_insensitive)]
    Fails,

    #[strum(ascii_case_insensitive)]
    Each,

    #[strum(ascii_case_insensitive)]
    RunIf,

    #[strum(ascii_case_insensitive)]
    SkipIf,
}

#[derive(AsRefStr, EnumString, Debug)]
pub enum TestSym {
    #[strum(ascii_case_insensitive)]
    It,

    #[strum(ascii_case_insensitive)]
    Test,
}

pub fn get_test_sym(sym: &str) -> Option<TestSym> {
    TestSym::from_str(sym).ok()
}

pub fn get_modifier_prop(prop: &MemberProp) -> Option<TestModifer> {
    match prop {
        MemberProp::Ident(Ident { sym, .. }) => TestModifer::from_str(sym.as_str()).ok(),
        _ => None,
    }
}
