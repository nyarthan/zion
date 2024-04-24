use swc_core::{common::DUMMY_SP, ecma::ast::*};

pub trait PatIdentAccess {
    type PatKind;

    fn access_pat_ident(&self, key: &str) -> Option<Expr>;
}

impl PatIdentAccess for Pat {
    type PatKind = Pat;
    fn access_pat_ident(&self, key: &str) -> Option<Expr> {
        match self {
            Pat::Ident(pat) => pat.access_pat_ident(key),
            Pat::Rest(pat) => pat.access_pat_ident(key),
            Pat::Object(pat) => pat.access_pat_ident(key),
            Pat::Array(pat) => pat.access_pat_ident(key),
            Pat::Assign(pat) => pat.access_pat_ident(key),
            _ => None,
        }
    }
}

impl PatIdentAccess for AssignPat {
    type PatKind = AssignPat;

    fn access_pat_ident(&self, key: &str) -> Option<Expr> {
        self.left.access_pat_ident(key)
    }
}

impl PatIdentAccess for RestPat {
    type PatKind = RestPat;

    fn access_pat_ident(&self, key: &str) -> Option<Expr> {
        match &*self.arg {
            Pat::Ident(binding_ident) => Some(Expr::Member(MemberExpr {
                span: DUMMY_SP,
                obj: Box::new(Expr::Member(MemberExpr {
                    span: DUMMY_SP,
                    obj: Box::new(Expr::Ident(Ident {
                        span: DUMMY_SP,
                        sym: binding_ident.sym.clone(),
                        optional: false,
                    })),
                    prop: MemberProp::Computed(ComputedPropName {
                        span: DUMMY_SP,
                        expr: Box::new(Expr::Lit(Lit::Num(Number {
                            span: DUMMY_SP,
                            value: 0.0,
                            raw: None,
                        }))),
                    }),
                })),
                prop: MemberProp::Ident(Ident {
                    span: DUMMY_SP,
                    optional: false,
                    sym: key.into(),
                }),
            })),
            Pat::Array(array_pat) => array_pat.access_pat_ident(key),
            _ => None,
        }
    }
}

impl PatIdentAccess for BindingIdent {
    type PatKind = BindingIdent;

    fn access_pat_ident(&self, key: &str) -> Option<Expr> {
        Some(Expr::Member(MemberExpr {
            span: DUMMY_SP,
            obj: Expr::Ident(self.id.clone()).into(),
            prop: MemberProp::Ident(Ident {
                span: DUMMY_SP,
                sym: key.into(),
                optional: false,
            }),
        }))
    }
}

impl PatIdentAccess for ArrayPat {
    type PatKind = ArrayPat;

    fn access_pat_ident(&self, key: &str) -> Option<Expr> {
        let pat = match self.elems.get(0)? {
            Some(pat) => pat,
            None => return None,
        };

        if let Pat::Ident(BindingIdent { id, .. }) = pat {
            return Some(Expr::Member(MemberExpr {
                span: DUMMY_SP,
                obj: Expr::Ident(id.clone()).into(),
                prop: MemberProp::Ident(Ident {
                    span: DUMMY_SP,
                    sym: key.into(),
                    optional: false,
                }),
            }));
        }

        if let Pat::Assign(AssignPat { left, .. }) = pat {
            if let Pat::Ident(BindingIdent { id, .. }) = &**left {
                return Some(Expr::Member(MemberExpr {
                    span: DUMMY_SP,
                    obj: Expr::Ident(id.clone()).into(),
                    prop: MemberProp::Ident(Ident {
                        span: DUMMY_SP,
                        sym: key.into(),
                        optional: false,
                    }),
                }));
            };
        }

        pat.access_pat_ident(key)
    }
}

impl PatIdentAccess for KeyValuePatProp {
    type PatKind = KeyValuePatProp;

    fn access_pat_ident(&self, key: &str) -> Option<Expr> {
        if let PropName::Ident(ident) = &self.key {
            if ident.sym != key {
                None
            } else {
                match &*self.value {
                    Pat::Ident(binding_ident) => binding_ident.access_pat_ident(key),
                    _ => None,
                }
            }
        } else {
            None
        }
    }
}

impl PatIdentAccess for ObjectPat {
    type PatKind = ObjectPat;

    fn access_pat_ident(&self, key: &str) -> Option<Expr> {
        let object_pat_prop = self.props.first().unwrap();

        let task_ident = self.props.iter().find_map(|prop| match prop {
            ObjectPatProp::Assign(AssignPatProp {
                key: BindingIdent { id, .. },
                ..
            }) => Some(id),
            ObjectPatProp::KeyValue(KeyValuePatProp {
                key: PropName::Ident(_ident),
                value,
            }) => {
                let pat = &**value;
                match pat {
                    Pat::Ident(BindingIdent { id, .. }) => Some(id),
                    _ => None,
                }
            }
            _ => None,
        });

        if let Some(task_ident) = task_ident {
            return Some(Expr::Ident(task_ident.clone()));
        }

        let rest_prop_ident = self.props.iter().find_map(|prop| match prop {
            ObjectPatProp::Rest(RestPat { arg, .. }) => {
                if let Pat::Ident(BindingIdent { id, .. }) = &**arg {
                    return Some(id);
                };
                None
            }
            _ => None,
        });

        if let Some(ident) = rest_prop_ident {
            return Some(Expr::Member(MemberExpr {
                span: DUMMY_SP,
                obj: ident.clone().into(),
                prop: MemberProp::Ident(Ident {
                    span: DUMMY_SP,
                    optional: false,
                    sym: key.into(),
                }),
            }));
        }

        match object_pat_prop {
            ObjectPatProp::Assign(AssignPatProp { key, .. }) => Some(Expr::Ident(key.id.clone())),
            ObjectPatProp::KeyValue(key_value_pat_prop) => key_value_pat_prop.access_pat_ident(key),
            _ => None,
        }
    }
}
