#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use serde::{Deserialize, Serialize};
use swc_ast_derive::ToAstStruct;
use swc_ast_trait::ToAst;
use swc_core::{common::DUMMY_SP, ecma::ast::*};
use typify::import_types;
/// Error types.
pub mod error {
    /// Error from a TryFrom or FromStr implementation.
    pub struct ConversionError(std::borrow::Cow<'static, str>);
    impl std::error::Error for ConversionError {}
    impl std::fmt::Display for ConversionError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
            std::fmt::Display::fmt(&self.0, f)
        }
    }
    impl std::fmt::Debug for ConversionError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
            std::fmt::Debug::fmt(&self.0, f)
        }
    }
    impl From<&'static str> for ConversionError {
        fn from(value: &'static str) -> Self {
            Self(value.into())
        }
    }
    impl From<String> for ConversionError {
        fn from(value: String) -> Self {
            Self(value.into())
        }
    }
}
///CoverageData
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "branches",
///    "fns",
///    "stmts"
///  ],
///  "properties": {
///    "branches": {
///      "type": "array",
///      "items": {
///        "type": "object",
///        "required": [
///          "loc",
///          "type"
///        ],
///        "properties": {
///          "loc": {
///            "type": "object",
///            "required": [
///              "end",
///              "start"
///            ],
///            "properties": {
///              "end": {
///                "type": "number"
///              },
///              "start": {
///                "type": "number"
///              }
///            },
///            "additionalProperties": false
///          },
///          "type": {
///            "type": "string",
///            "enum": [
///              "if",
///              "else"
///            ]
///          }
///        },
///        "additionalProperties": false
///      }
///    },
///    "fns": {
///      "type": "array",
///      "items": {
///        "type": "object",
///        "required": [
///          "decl",
///          "loc"
///        ],
///        "properties": {
///          "decl": {
///            "type": "object",
///            "required": [
///              "end",
///              "start"
///            ],
///            "properties": {
///              "end": {
///                "type": "number"
///              },
///              "start": {
///                "type": "number"
///              }
///            },
///            "additionalProperties": false
///          },
///          "loc": {
///            "type": "object",
///            "required": [
///              "end",
///              "start"
///            ],
///            "properties": {
///              "end": {
///                "type": "number"
///              },
///              "start": {
///                "type": "number"
///              }
///            },
///            "additionalProperties": false
///          },
///          "name": {
///            "type": "string"
///          }
///        },
///        "additionalProperties": false
///      }
///    },
///    "stmts": {
///      "type": "array",
///      "items": {
///        "type": "object",
///        "required": [
///          "end",
///          "start"
///        ],
///        "properties": {
///          "end": {
///            "type": "number"
///          },
///          "start": {
///            "type": "number"
///          }
///        },
///        "additionalProperties": false
///      }
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[serde(deny_unknown_fields)]
pub struct CoverageData {
    pub branches: Vec<CoverageDataBranchesItem>,
    pub fns: Vec<CoverageDataFnsItem>,
    pub stmts: Vec<CoverageDataStmtsItem>,
}
#[automatically_derived]
impl ::core::clone::Clone for CoverageData {
    #[inline]
    fn clone(&self) -> CoverageData {
        CoverageData {
            branches: ::core::clone::Clone::clone(&self.branches),
            fns: ::core::clone::Clone::clone(&self.fns),
            stmts: ::core::clone::Clone::clone(&self.stmts),
        }
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for CoverageData {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field3_finish(
            f,
            "CoverageData",
            "branches",
            &self.branches,
            "fns",
            &self.fns,
            "stmts",
            &&self.stmts,
        )
    }
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for CoverageData {
        fn deserialize<__D>(
            __deserializer: __D,
        ) -> _serde::__private::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            #[doc(hidden)]
            enum __Field {
                __field0,
                __field1,
                __field2,
            }
            #[doc(hidden)]
            struct __FieldVisitor;
            impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                type Value = __Field;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "field identifier",
                    )
                }
                fn visit_u64<__E>(
                    self,
                    __value: u64,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        0u64 => _serde::__private::Ok(__Field::__field0),
                        1u64 => _serde::__private::Ok(__Field::__field1),
                        2u64 => _serde::__private::Ok(__Field::__field2),
                        _ => {
                            _serde::__private::Err(
                                _serde::de::Error::invalid_value(
                                    _serde::de::Unexpected::Unsigned(__value),
                                    &"field index 0 <= i < 3",
                                ),
                            )
                        }
                    }
                }
                fn visit_str<__E>(
                    self,
                    __value: &str,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        "branches" => _serde::__private::Ok(__Field::__field0),
                        "fns" => _serde::__private::Ok(__Field::__field1),
                        "stmts" => _serde::__private::Ok(__Field::__field2),
                        _ => {
                            _serde::__private::Err(
                                _serde::de::Error::unknown_field(__value, FIELDS),
                            )
                        }
                    }
                }
                fn visit_bytes<__E>(
                    self,
                    __value: &[u8],
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        b"branches" => _serde::__private::Ok(__Field::__field0),
                        b"fns" => _serde::__private::Ok(__Field::__field1),
                        b"stmts" => _serde::__private::Ok(__Field::__field2),
                        _ => {
                            let __value = &_serde::__private::from_utf8_lossy(__value);
                            _serde::__private::Err(
                                _serde::de::Error::unknown_field(__value, FIELDS),
                            )
                        }
                    }
                }
            }
            impl<'de> _serde::Deserialize<'de> for __Field {
                #[inline]
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    _serde::Deserializer::deserialize_identifier(
                        __deserializer,
                        __FieldVisitor,
                    )
                }
            }
            #[doc(hidden)]
            struct __Visitor<'de> {
                marker: _serde::__private::PhantomData<CoverageData>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = CoverageData;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "struct CoverageData",
                    )
                }
                #[inline]
                fn visit_seq<__A>(
                    self,
                    mut __seq: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::SeqAccess<'de>,
                {
                    let __field0 = match _serde::de::SeqAccess::next_element::<
                        Vec<CoverageDataBranchesItem>,
                    >(&mut __seq)? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                _serde::de::Error::invalid_length(
                                    0usize,
                                    &"struct CoverageData with 3 elements",
                                ),
                            );
                        }
                    };
                    let __field1 = match _serde::de::SeqAccess::next_element::<
                        Vec<CoverageDataFnsItem>,
                    >(&mut __seq)? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                _serde::de::Error::invalid_length(
                                    1usize,
                                    &"struct CoverageData with 3 elements",
                                ),
                            );
                        }
                    };
                    let __field2 = match _serde::de::SeqAccess::next_element::<
                        Vec<CoverageDataStmtsItem>,
                    >(&mut __seq)? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                _serde::de::Error::invalid_length(
                                    2usize,
                                    &"struct CoverageData with 3 elements",
                                ),
                            );
                        }
                    };
                    _serde::__private::Ok(CoverageData {
                        branches: __field0,
                        fns: __field1,
                        stmts: __field2,
                    })
                }
                #[inline]
                fn visit_map<__A>(
                    self,
                    mut __map: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::MapAccess<'de>,
                {
                    let mut __field0: _serde::__private::Option<
                        Vec<CoverageDataBranchesItem>,
                    > = _serde::__private::None;
                    let mut __field1: _serde::__private::Option<
                        Vec<CoverageDataFnsItem>,
                    > = _serde::__private::None;
                    let mut __field2: _serde::__private::Option<
                        Vec<CoverageDataStmtsItem>,
                    > = _serde::__private::None;
                    while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                        __Field,
                    >(&mut __map)? {
                        match __key {
                            __Field::__field0 => {
                                if _serde::__private::Option::is_some(&__field0) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "branches",
                                        ),
                                    );
                                }
                                __field0 = _serde::__private::Some(
                                    _serde::de::MapAccess::next_value::<
                                        Vec<CoverageDataBranchesItem>,
                                    >(&mut __map)?,
                                );
                            }
                            __Field::__field1 => {
                                if _serde::__private::Option::is_some(&__field1) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("fns"),
                                    );
                                }
                                __field1 = _serde::__private::Some(
                                    _serde::de::MapAccess::next_value::<
                                        Vec<CoverageDataFnsItem>,
                                    >(&mut __map)?,
                                );
                            }
                            __Field::__field2 => {
                                if _serde::__private::Option::is_some(&__field2) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("stmts"),
                                    );
                                }
                                __field2 = _serde::__private::Some(
                                    _serde::de::MapAccess::next_value::<
                                        Vec<CoverageDataStmtsItem>,
                                    >(&mut __map)?,
                                );
                            }
                        }
                    }
                    let __field0 = match __field0 {
                        _serde::__private::Some(__field0) => __field0,
                        _serde::__private::None => {
                            _serde::__private::de::missing_field("branches")?
                        }
                    };
                    let __field1 = match __field1 {
                        _serde::__private::Some(__field1) => __field1,
                        _serde::__private::None => {
                            _serde::__private::de::missing_field("fns")?
                        }
                    };
                    let __field2 = match __field2 {
                        _serde::__private::Some(__field2) => __field2,
                        _serde::__private::None => {
                            _serde::__private::de::missing_field("stmts")?
                        }
                    };
                    _serde::__private::Ok(CoverageData {
                        branches: __field0,
                        fns: __field1,
                        stmts: __field2,
                    })
                }
            }
            #[doc(hidden)]
            const FIELDS: &'static [&'static str] = &["branches", "fns", "stmts"];
            _serde::Deserializer::deserialize_struct(
                __deserializer,
                "CoverageData",
                FIELDS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<CoverageData>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl _serde::Serialize for CoverageData {
        fn serialize<__S>(
            &self,
            __serializer: __S,
        ) -> _serde::__private::Result<__S::Ok, __S::Error>
        where
            __S: _serde::Serializer,
        {
            let mut __serde_state = _serde::Serializer::serialize_struct(
                __serializer,
                "CoverageData",
                false as usize + 1 + 1 + 1,
            )?;
            _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "branches",
                &self.branches,
            )?;
            _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "fns",
                &self.fns,
            )?;
            _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "stmts",
                &self.stmts,
            )?;
            _serde::ser::SerializeStruct::end(__serde_state)
        }
    }
};
impl From<&CoverageData> for CoverageData {
    fn from(value: &CoverageData) -> Self {
        value.clone()
    }
}
///CoverageDataBranchesItem
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "loc",
///    "type"
///  ],
///  "properties": {
///    "loc": {
///      "type": "object",
///      "required": [
///        "end",
///        "start"
///      ],
///      "properties": {
///        "end": {
///          "type": "number"
///        },
///        "start": {
///          "type": "number"
///        }
///      },
///      "additionalProperties": false
///    },
///    "type": {
///      "type": "string",
///      "enum": [
///        "if",
///        "else"
///      ]
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[serde(deny_unknown_fields)]
pub struct CoverageDataBranchesItem {
    pub loc: CoverageDataBranchesItemLoc,
    #[serde(rename = "type")]
    pub type_: CoverageDataBranchesItemType,
}
#[automatically_derived]
impl ::core::clone::Clone for CoverageDataBranchesItem {
    #[inline]
    fn clone(&self) -> CoverageDataBranchesItem {
        CoverageDataBranchesItem {
            loc: ::core::clone::Clone::clone(&self.loc),
            type_: ::core::clone::Clone::clone(&self.type_),
        }
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for CoverageDataBranchesItem {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field2_finish(
            f,
            "CoverageDataBranchesItem",
            "loc",
            &self.loc,
            "type_",
            &&self.type_,
        )
    }
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for CoverageDataBranchesItem {
        fn deserialize<__D>(
            __deserializer: __D,
        ) -> _serde::__private::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            #[doc(hidden)]
            enum __Field {
                __field0,
                __field1,
            }
            #[doc(hidden)]
            struct __FieldVisitor;
            impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                type Value = __Field;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "field identifier",
                    )
                }
                fn visit_u64<__E>(
                    self,
                    __value: u64,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        0u64 => _serde::__private::Ok(__Field::__field0),
                        1u64 => _serde::__private::Ok(__Field::__field1),
                        _ => {
                            _serde::__private::Err(
                                _serde::de::Error::invalid_value(
                                    _serde::de::Unexpected::Unsigned(__value),
                                    &"field index 0 <= i < 2",
                                ),
                            )
                        }
                    }
                }
                fn visit_str<__E>(
                    self,
                    __value: &str,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        "loc" => _serde::__private::Ok(__Field::__field0),
                        "type" => _serde::__private::Ok(__Field::__field1),
                        _ => {
                            _serde::__private::Err(
                                _serde::de::Error::unknown_field(__value, FIELDS),
                            )
                        }
                    }
                }
                fn visit_bytes<__E>(
                    self,
                    __value: &[u8],
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        b"loc" => _serde::__private::Ok(__Field::__field0),
                        b"type" => _serde::__private::Ok(__Field::__field1),
                        _ => {
                            let __value = &_serde::__private::from_utf8_lossy(__value);
                            _serde::__private::Err(
                                _serde::de::Error::unknown_field(__value, FIELDS),
                            )
                        }
                    }
                }
            }
            impl<'de> _serde::Deserialize<'de> for __Field {
                #[inline]
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    _serde::Deserializer::deserialize_identifier(
                        __deserializer,
                        __FieldVisitor,
                    )
                }
            }
            #[doc(hidden)]
            struct __Visitor<'de> {
                marker: _serde::__private::PhantomData<CoverageDataBranchesItem>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = CoverageDataBranchesItem;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "struct CoverageDataBranchesItem",
                    )
                }
                #[inline]
                fn visit_seq<__A>(
                    self,
                    mut __seq: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::SeqAccess<'de>,
                {
                    let __field0 = match _serde::de::SeqAccess::next_element::<
                        CoverageDataBranchesItemLoc,
                    >(&mut __seq)? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                _serde::de::Error::invalid_length(
                                    0usize,
                                    &"struct CoverageDataBranchesItem with 2 elements",
                                ),
                            );
                        }
                    };
                    let __field1 = match _serde::de::SeqAccess::next_element::<
                        CoverageDataBranchesItemType,
                    >(&mut __seq)? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                _serde::de::Error::invalid_length(
                                    1usize,
                                    &"struct CoverageDataBranchesItem with 2 elements",
                                ),
                            );
                        }
                    };
                    _serde::__private::Ok(CoverageDataBranchesItem {
                        loc: __field0,
                        type_: __field1,
                    })
                }
                #[inline]
                fn visit_map<__A>(
                    self,
                    mut __map: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::MapAccess<'de>,
                {
                    let mut __field0: _serde::__private::Option<
                        CoverageDataBranchesItemLoc,
                    > = _serde::__private::None;
                    let mut __field1: _serde::__private::Option<
                        CoverageDataBranchesItemType,
                    > = _serde::__private::None;
                    while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                        __Field,
                    >(&mut __map)? {
                        match __key {
                            __Field::__field0 => {
                                if _serde::__private::Option::is_some(&__field0) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("loc"),
                                    );
                                }
                                __field0 = _serde::__private::Some(
                                    _serde::de::MapAccess::next_value::<
                                        CoverageDataBranchesItemLoc,
                                    >(&mut __map)?,
                                );
                            }
                            __Field::__field1 => {
                                if _serde::__private::Option::is_some(&__field1) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("type"),
                                    );
                                }
                                __field1 = _serde::__private::Some(
                                    _serde::de::MapAccess::next_value::<
                                        CoverageDataBranchesItemType,
                                    >(&mut __map)?,
                                );
                            }
                        }
                    }
                    let __field0 = match __field0 {
                        _serde::__private::Some(__field0) => __field0,
                        _serde::__private::None => {
                            _serde::__private::de::missing_field("loc")?
                        }
                    };
                    let __field1 = match __field1 {
                        _serde::__private::Some(__field1) => __field1,
                        _serde::__private::None => {
                            _serde::__private::de::missing_field("type")?
                        }
                    };
                    _serde::__private::Ok(CoverageDataBranchesItem {
                        loc: __field0,
                        type_: __field1,
                    })
                }
            }
            #[doc(hidden)]
            const FIELDS: &'static [&'static str] = &["loc", "type"];
            _serde::Deserializer::deserialize_struct(
                __deserializer,
                "CoverageDataBranchesItem",
                FIELDS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<CoverageDataBranchesItem>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl _serde::Serialize for CoverageDataBranchesItem {
        fn serialize<__S>(
            &self,
            __serializer: __S,
        ) -> _serde::__private::Result<__S::Ok, __S::Error>
        where
            __S: _serde::Serializer,
        {
            let mut __serde_state = _serde::Serializer::serialize_struct(
                __serializer,
                "CoverageDataBranchesItem",
                false as usize + 1 + 1,
            )?;
            _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "loc",
                &self.loc,
            )?;
            _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "type",
                &self.type_,
            )?;
            _serde::ser::SerializeStruct::end(__serde_state)
        }
    }
};
impl From<&CoverageDataBranchesItem> for CoverageDataBranchesItem {
    fn from(value: &CoverageDataBranchesItem) -> Self {
        value.clone()
    }
}
///CoverageDataBranchesItemLoc
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "end",
///    "start"
///  ],
///  "properties": {
///    "end": {
///      "type": "number"
///    },
///    "start": {
///      "type": "number"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[serde(deny_unknown_fields)]
pub struct CoverageDataBranchesItemLoc {
    pub end: f64,
    pub start: f64,
}
#[automatically_derived]
impl ::core::clone::Clone for CoverageDataBranchesItemLoc {
    #[inline]
    fn clone(&self) -> CoverageDataBranchesItemLoc {
        CoverageDataBranchesItemLoc {
            end: ::core::clone::Clone::clone(&self.end),
            start: ::core::clone::Clone::clone(&self.start),
        }
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for CoverageDataBranchesItemLoc {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field2_finish(
            f,
            "CoverageDataBranchesItemLoc",
            "end",
            &self.end,
            "start",
            &&self.start,
        )
    }
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for CoverageDataBranchesItemLoc {
        fn deserialize<__D>(
            __deserializer: __D,
        ) -> _serde::__private::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            #[doc(hidden)]
            enum __Field {
                __field0,
                __field1,
            }
            #[doc(hidden)]
            struct __FieldVisitor;
            impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                type Value = __Field;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "field identifier",
                    )
                }
                fn visit_u64<__E>(
                    self,
                    __value: u64,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        0u64 => _serde::__private::Ok(__Field::__field0),
                        1u64 => _serde::__private::Ok(__Field::__field1),
                        _ => {
                            _serde::__private::Err(
                                _serde::de::Error::invalid_value(
                                    _serde::de::Unexpected::Unsigned(__value),
                                    &"field index 0 <= i < 2",
                                ),
                            )
                        }
                    }
                }
                fn visit_str<__E>(
                    self,
                    __value: &str,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        "end" => _serde::__private::Ok(__Field::__field0),
                        "start" => _serde::__private::Ok(__Field::__field1),
                        _ => {
                            _serde::__private::Err(
                                _serde::de::Error::unknown_field(__value, FIELDS),
                            )
                        }
                    }
                }
                fn visit_bytes<__E>(
                    self,
                    __value: &[u8],
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        b"end" => _serde::__private::Ok(__Field::__field0),
                        b"start" => _serde::__private::Ok(__Field::__field1),
                        _ => {
                            let __value = &_serde::__private::from_utf8_lossy(__value);
                            _serde::__private::Err(
                                _serde::de::Error::unknown_field(__value, FIELDS),
                            )
                        }
                    }
                }
            }
            impl<'de> _serde::Deserialize<'de> for __Field {
                #[inline]
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    _serde::Deserializer::deserialize_identifier(
                        __deserializer,
                        __FieldVisitor,
                    )
                }
            }
            #[doc(hidden)]
            struct __Visitor<'de> {
                marker: _serde::__private::PhantomData<CoverageDataBranchesItemLoc>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = CoverageDataBranchesItemLoc;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "struct CoverageDataBranchesItemLoc",
                    )
                }
                #[inline]
                fn visit_seq<__A>(
                    self,
                    mut __seq: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::SeqAccess<'de>,
                {
                    let __field0 = match _serde::de::SeqAccess::next_element::<
                        f64,
                    >(&mut __seq)? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                _serde::de::Error::invalid_length(
                                    0usize,
                                    &"struct CoverageDataBranchesItemLoc with 2 elements",
                                ),
                            );
                        }
                    };
                    let __field1 = match _serde::de::SeqAccess::next_element::<
                        f64,
                    >(&mut __seq)? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                _serde::de::Error::invalid_length(
                                    1usize,
                                    &"struct CoverageDataBranchesItemLoc with 2 elements",
                                ),
                            );
                        }
                    };
                    _serde::__private::Ok(CoverageDataBranchesItemLoc {
                        end: __field0,
                        start: __field1,
                    })
                }
                #[inline]
                fn visit_map<__A>(
                    self,
                    mut __map: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::MapAccess<'de>,
                {
                    let mut __field0: _serde::__private::Option<f64> = _serde::__private::None;
                    let mut __field1: _serde::__private::Option<f64> = _serde::__private::None;
                    while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                        __Field,
                    >(&mut __map)? {
                        match __key {
                            __Field::__field0 => {
                                if _serde::__private::Option::is_some(&__field0) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("end"),
                                    );
                                }
                                __field0 = _serde::__private::Some(
                                    _serde::de::MapAccess::next_value::<f64>(&mut __map)?,
                                );
                            }
                            __Field::__field1 => {
                                if _serde::__private::Option::is_some(&__field1) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("start"),
                                    );
                                }
                                __field1 = _serde::__private::Some(
                                    _serde::de::MapAccess::next_value::<f64>(&mut __map)?,
                                );
                            }
                        }
                    }
                    let __field0 = match __field0 {
                        _serde::__private::Some(__field0) => __field0,
                        _serde::__private::None => {
                            _serde::__private::de::missing_field("end")?
                        }
                    };
                    let __field1 = match __field1 {
                        _serde::__private::Some(__field1) => __field1,
                        _serde::__private::None => {
                            _serde::__private::de::missing_field("start")?
                        }
                    };
                    _serde::__private::Ok(CoverageDataBranchesItemLoc {
                        end: __field0,
                        start: __field1,
                    })
                }
            }
            #[doc(hidden)]
            const FIELDS: &'static [&'static str] = &["end", "start"];
            _serde::Deserializer::deserialize_struct(
                __deserializer,
                "CoverageDataBranchesItemLoc",
                FIELDS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<
                        CoverageDataBranchesItemLoc,
                    >,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl _serde::Serialize for CoverageDataBranchesItemLoc {
        fn serialize<__S>(
            &self,
            __serializer: __S,
        ) -> _serde::__private::Result<__S::Ok, __S::Error>
        where
            __S: _serde::Serializer,
        {
            let mut __serde_state = _serde::Serializer::serialize_struct(
                __serializer,
                "CoverageDataBranchesItemLoc",
                false as usize + 1 + 1,
            )?;
            _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "end",
                &self.end,
            )?;
            _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "start",
                &self.start,
            )?;
            _serde::ser::SerializeStruct::end(__serde_state)
        }
    }
};
impl From<&CoverageDataBranchesItemLoc> for CoverageDataBranchesItemLoc {
    fn from(value: &CoverageDataBranchesItemLoc) -> Self {
        value.clone()
    }
}
///CoverageDataBranchesItemType
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "enum": [
///    "if",
///    "else"
///  ]
///}
/// ```
/// </details>
pub enum CoverageDataBranchesItemType {
    #[serde(rename = "if")]
    If,
    #[serde(rename = "else")]
    Else,
}
#[automatically_derived]
impl ::core::clone::Clone for CoverageDataBranchesItemType {
    #[inline]
    fn clone(&self) -> CoverageDataBranchesItemType {
        *self
    }
}
#[automatically_derived]
impl ::core::marker::Copy for CoverageDataBranchesItemType {}
#[automatically_derived]
impl ::core::fmt::Debug for CoverageDataBranchesItemType {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::write_str(
            f,
            match self {
                CoverageDataBranchesItemType::If => "If",
                CoverageDataBranchesItemType::Else => "Else",
            },
        )
    }
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for CoverageDataBranchesItemType {
        fn deserialize<__D>(
            __deserializer: __D,
        ) -> _serde::__private::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            #[doc(hidden)]
            enum __Field {
                __field0,
                __field1,
            }
            #[doc(hidden)]
            struct __FieldVisitor;
            impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                type Value = __Field;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "variant identifier",
                    )
                }
                fn visit_u64<__E>(
                    self,
                    __value: u64,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        0u64 => _serde::__private::Ok(__Field::__field0),
                        1u64 => _serde::__private::Ok(__Field::__field1),
                        _ => {
                            _serde::__private::Err(
                                _serde::de::Error::invalid_value(
                                    _serde::de::Unexpected::Unsigned(__value),
                                    &"variant index 0 <= i < 2",
                                ),
                            )
                        }
                    }
                }
                fn visit_str<__E>(
                    self,
                    __value: &str,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        "if" => _serde::__private::Ok(__Field::__field0),
                        "else" => _serde::__private::Ok(__Field::__field1),
                        _ => {
                            _serde::__private::Err(
                                _serde::de::Error::unknown_variant(__value, VARIANTS),
                            )
                        }
                    }
                }
                fn visit_bytes<__E>(
                    self,
                    __value: &[u8],
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        b"if" => _serde::__private::Ok(__Field::__field0),
                        b"else" => _serde::__private::Ok(__Field::__field1),
                        _ => {
                            let __value = &_serde::__private::from_utf8_lossy(__value);
                            _serde::__private::Err(
                                _serde::de::Error::unknown_variant(__value, VARIANTS),
                            )
                        }
                    }
                }
            }
            impl<'de> _serde::Deserialize<'de> for __Field {
                #[inline]
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    _serde::Deserializer::deserialize_identifier(
                        __deserializer,
                        __FieldVisitor,
                    )
                }
            }
            #[doc(hidden)]
            struct __Visitor<'de> {
                marker: _serde::__private::PhantomData<CoverageDataBranchesItemType>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = CoverageDataBranchesItemType;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "enum CoverageDataBranchesItemType",
                    )
                }
                fn visit_enum<__A>(
                    self,
                    __data: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::EnumAccess<'de>,
                {
                    match _serde::de::EnumAccess::variant(__data)? {
                        (__Field::__field0, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(CoverageDataBranchesItemType::If)
                        }
                        (__Field::__field1, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(CoverageDataBranchesItemType::Else)
                        }
                    }
                }
            }
            #[doc(hidden)]
            const VARIANTS: &'static [&'static str] = &["if", "else"];
            _serde::Deserializer::deserialize_enum(
                __deserializer,
                "CoverageDataBranchesItemType",
                VARIANTS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<
                        CoverageDataBranchesItemType,
                    >,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
#[automatically_derived]
impl ::core::marker::StructuralEq for CoverageDataBranchesItemType {}
#[automatically_derived]
impl ::core::cmp::Eq for CoverageDataBranchesItemType {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {}
}
#[automatically_derived]
impl ::core::hash::Hash for CoverageDataBranchesItemType {
    #[inline]
    fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
        let __self_tag = ::core::intrinsics::discriminant_value(self);
        ::core::hash::Hash::hash(&__self_tag, state)
    }
}
#[automatically_derived]
impl ::core::cmp::Ord for CoverageDataBranchesItemType {
    #[inline]
    fn cmp(&self, other: &CoverageDataBranchesItemType) -> ::core::cmp::Ordering {
        let __self_tag = ::core::intrinsics::discriminant_value(self);
        let __arg1_tag = ::core::intrinsics::discriminant_value(other);
        ::core::cmp::Ord::cmp(&__self_tag, &__arg1_tag)
    }
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for CoverageDataBranchesItemType {}
#[automatically_derived]
impl ::core::cmp::PartialEq for CoverageDataBranchesItemType {
    #[inline]
    fn eq(&self, other: &CoverageDataBranchesItemType) -> bool {
        let __self_tag = ::core::intrinsics::discriminant_value(self);
        let __arg1_tag = ::core::intrinsics::discriminant_value(other);
        __self_tag == __arg1_tag
    }
}
#[automatically_derived]
impl ::core::cmp::PartialOrd for CoverageDataBranchesItemType {
    #[inline]
    fn partial_cmp(
        &self,
        other: &CoverageDataBranchesItemType,
    ) -> ::core::option::Option<::core::cmp::Ordering> {
        let __self_tag = ::core::intrinsics::discriminant_value(self);
        let __arg1_tag = ::core::intrinsics::discriminant_value(other);
        ::core::cmp::PartialOrd::partial_cmp(&__self_tag, &__arg1_tag)
    }
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl _serde::Serialize for CoverageDataBranchesItemType {
        fn serialize<__S>(
            &self,
            __serializer: __S,
        ) -> _serde::__private::Result<__S::Ok, __S::Error>
        where
            __S: _serde::Serializer,
        {
            match *self {
                CoverageDataBranchesItemType::If => {
                    _serde::Serializer::serialize_unit_variant(
                        __serializer,
                        "CoverageDataBranchesItemType",
                        0u32,
                        "if",
                    )
                }
                CoverageDataBranchesItemType::Else => {
                    _serde::Serializer::serialize_unit_variant(
                        __serializer,
                        "CoverageDataBranchesItemType",
                        1u32,
                        "else",
                    )
                }
            }
        }
    }
};
impl From<&CoverageDataBranchesItemType> for CoverageDataBranchesItemType {
    fn from(value: &CoverageDataBranchesItemType) -> Self {
        value.clone()
    }
}
impl ToString for CoverageDataBranchesItemType {
    fn to_string(&self) -> String {
        match *self {
            Self::If => "if".to_string(),
            Self::Else => "else".to_string(),
        }
    }
}
impl std::str::FromStr for CoverageDataBranchesItemType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "if" => Ok(Self::If),
            "else" => Ok(Self::Else),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for CoverageDataBranchesItemType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for CoverageDataBranchesItemType {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for CoverageDataBranchesItemType {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///CoverageDataFnsItem
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "decl",
///    "loc"
///  ],
///  "properties": {
///    "decl": {
///      "type": "object",
///      "required": [
///        "end",
///        "start"
///      ],
///      "properties": {
///        "end": {
///          "type": "number"
///        },
///        "start": {
///          "type": "number"
///        }
///      },
///      "additionalProperties": false
///    },
///    "loc": {
///      "type": "object",
///      "required": [
///        "end",
///        "start"
///      ],
///      "properties": {
///        "end": {
///          "type": "number"
///        },
///        "start": {
///          "type": "number"
///        }
///      },
///      "additionalProperties": false
///    },
///    "name": {
///      "type": "string"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[serde(deny_unknown_fields)]
pub struct CoverageDataFnsItem {
    pub decl: CoverageDataFnsItemDecl,
    pub loc: CoverageDataFnsItemLoc,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
#[automatically_derived]
impl ::core::clone::Clone for CoverageDataFnsItem {
    #[inline]
    fn clone(&self) -> CoverageDataFnsItem {
        CoverageDataFnsItem {
            decl: ::core::clone::Clone::clone(&self.decl),
            loc: ::core::clone::Clone::clone(&self.loc),
            name: ::core::clone::Clone::clone(&self.name),
        }
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for CoverageDataFnsItem {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field3_finish(
            f,
            "CoverageDataFnsItem",
            "decl",
            &self.decl,
            "loc",
            &self.loc,
            "name",
            &&self.name,
        )
    }
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for CoverageDataFnsItem {
        fn deserialize<__D>(
            __deserializer: __D,
        ) -> _serde::__private::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            #[doc(hidden)]
            enum __Field {
                __field0,
                __field1,
                __field2,
            }
            #[doc(hidden)]
            struct __FieldVisitor;
            impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                type Value = __Field;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "field identifier",
                    )
                }
                fn visit_u64<__E>(
                    self,
                    __value: u64,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        0u64 => _serde::__private::Ok(__Field::__field0),
                        1u64 => _serde::__private::Ok(__Field::__field1),
                        2u64 => _serde::__private::Ok(__Field::__field2),
                        _ => {
                            _serde::__private::Err(
                                _serde::de::Error::invalid_value(
                                    _serde::de::Unexpected::Unsigned(__value),
                                    &"field index 0 <= i < 3",
                                ),
                            )
                        }
                    }
                }
                fn visit_str<__E>(
                    self,
                    __value: &str,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        "decl" => _serde::__private::Ok(__Field::__field0),
                        "loc" => _serde::__private::Ok(__Field::__field1),
                        "name" => _serde::__private::Ok(__Field::__field2),
                        _ => {
                            _serde::__private::Err(
                                _serde::de::Error::unknown_field(__value, FIELDS),
                            )
                        }
                    }
                }
                fn visit_bytes<__E>(
                    self,
                    __value: &[u8],
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        b"decl" => _serde::__private::Ok(__Field::__field0),
                        b"loc" => _serde::__private::Ok(__Field::__field1),
                        b"name" => _serde::__private::Ok(__Field::__field2),
                        _ => {
                            let __value = &_serde::__private::from_utf8_lossy(__value);
                            _serde::__private::Err(
                                _serde::de::Error::unknown_field(__value, FIELDS),
                            )
                        }
                    }
                }
            }
            impl<'de> _serde::Deserialize<'de> for __Field {
                #[inline]
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    _serde::Deserializer::deserialize_identifier(
                        __deserializer,
                        __FieldVisitor,
                    )
                }
            }
            #[doc(hidden)]
            struct __Visitor<'de> {
                marker: _serde::__private::PhantomData<CoverageDataFnsItem>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = CoverageDataFnsItem;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "struct CoverageDataFnsItem",
                    )
                }
                #[inline]
                fn visit_seq<__A>(
                    self,
                    mut __seq: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::SeqAccess<'de>,
                {
                    let __field0 = match _serde::de::SeqAccess::next_element::<
                        CoverageDataFnsItemDecl,
                    >(&mut __seq)? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                _serde::de::Error::invalid_length(
                                    0usize,
                                    &"struct CoverageDataFnsItem with 3 elements",
                                ),
                            );
                        }
                    };
                    let __field1 = match _serde::de::SeqAccess::next_element::<
                        CoverageDataFnsItemLoc,
                    >(&mut __seq)? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                _serde::de::Error::invalid_length(
                                    1usize,
                                    &"struct CoverageDataFnsItem with 3 elements",
                                ),
                            );
                        }
                    };
                    let __field2 = match _serde::de::SeqAccess::next_element::<
                        Option<String>,
                    >(&mut __seq)? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => _serde::__private::Default::default(),
                    };
                    _serde::__private::Ok(CoverageDataFnsItem {
                        decl: __field0,
                        loc: __field1,
                        name: __field2,
                    })
                }
                #[inline]
                fn visit_map<__A>(
                    self,
                    mut __map: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::MapAccess<'de>,
                {
                    let mut __field0: _serde::__private::Option<
                        CoverageDataFnsItemDecl,
                    > = _serde::__private::None;
                    let mut __field1: _serde::__private::Option<
                        CoverageDataFnsItemLoc,
                    > = _serde::__private::None;
                    let mut __field2: _serde::__private::Option<Option<String>> = _serde::__private::None;
                    while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                        __Field,
                    >(&mut __map)? {
                        match __key {
                            __Field::__field0 => {
                                if _serde::__private::Option::is_some(&__field0) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("decl"),
                                    );
                                }
                                __field0 = _serde::__private::Some(
                                    _serde::de::MapAccess::next_value::<
                                        CoverageDataFnsItemDecl,
                                    >(&mut __map)?,
                                );
                            }
                            __Field::__field1 => {
                                if _serde::__private::Option::is_some(&__field1) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("loc"),
                                    );
                                }
                                __field1 = _serde::__private::Some(
                                    _serde::de::MapAccess::next_value::<
                                        CoverageDataFnsItemLoc,
                                    >(&mut __map)?,
                                );
                            }
                            __Field::__field2 => {
                                if _serde::__private::Option::is_some(&__field2) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("name"),
                                    );
                                }
                                __field2 = _serde::__private::Some(
                                    _serde::de::MapAccess::next_value::<
                                        Option<String>,
                                    >(&mut __map)?,
                                );
                            }
                        }
                    }
                    let __field0 = match __field0 {
                        _serde::__private::Some(__field0) => __field0,
                        _serde::__private::None => {
                            _serde::__private::de::missing_field("decl")?
                        }
                    };
                    let __field1 = match __field1 {
                        _serde::__private::Some(__field1) => __field1,
                        _serde::__private::None => {
                            _serde::__private::de::missing_field("loc")?
                        }
                    };
                    let __field2 = match __field2 {
                        _serde::__private::Some(__field2) => __field2,
                        _serde::__private::None => _serde::__private::Default::default(),
                    };
                    _serde::__private::Ok(CoverageDataFnsItem {
                        decl: __field0,
                        loc: __field1,
                        name: __field2,
                    })
                }
            }
            #[doc(hidden)]
            const FIELDS: &'static [&'static str] = &["decl", "loc", "name"];
            _serde::Deserializer::deserialize_struct(
                __deserializer,
                "CoverageDataFnsItem",
                FIELDS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<CoverageDataFnsItem>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl _serde::Serialize for CoverageDataFnsItem {
        fn serialize<__S>(
            &self,
            __serializer: __S,
        ) -> _serde::__private::Result<__S::Ok, __S::Error>
        where
            __S: _serde::Serializer,
        {
            let mut __serde_state = _serde::Serializer::serialize_struct(
                __serializer,
                "CoverageDataFnsItem",
                false as usize + 1 + 1 + if Option::is_none(&self.name) { 0 } else { 1 },
            )?;
            _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "decl",
                &self.decl,
            )?;
            _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "loc",
                &self.loc,
            )?;
            if !Option::is_none(&self.name) {
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "name",
                    &self.name,
                )?;
            } else {
                _serde::ser::SerializeStruct::skip_field(&mut __serde_state, "name")?;
            }
            _serde::ser::SerializeStruct::end(__serde_state)
        }
    }
};
impl From<&CoverageDataFnsItem> for CoverageDataFnsItem {
    fn from(value: &CoverageDataFnsItem) -> Self {
        value.clone()
    }
}
///CoverageDataFnsItemDecl
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "end",
///    "start"
///  ],
///  "properties": {
///    "end": {
///      "type": "number"
///    },
///    "start": {
///      "type": "number"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[serde(deny_unknown_fields)]
pub struct CoverageDataFnsItemDecl {
    pub end: f64,
    pub start: f64,
}
#[automatically_derived]
impl ::core::clone::Clone for CoverageDataFnsItemDecl {
    #[inline]
    fn clone(&self) -> CoverageDataFnsItemDecl {
        CoverageDataFnsItemDecl {
            end: ::core::clone::Clone::clone(&self.end),
            start: ::core::clone::Clone::clone(&self.start),
        }
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for CoverageDataFnsItemDecl {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field2_finish(
            f,
            "CoverageDataFnsItemDecl",
            "end",
            &self.end,
            "start",
            &&self.start,
        )
    }
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for CoverageDataFnsItemDecl {
        fn deserialize<__D>(
            __deserializer: __D,
        ) -> _serde::__private::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            #[doc(hidden)]
            enum __Field {
                __field0,
                __field1,
            }
            #[doc(hidden)]
            struct __FieldVisitor;
            impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                type Value = __Field;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "field identifier",
                    )
                }
                fn visit_u64<__E>(
                    self,
                    __value: u64,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        0u64 => _serde::__private::Ok(__Field::__field0),
                        1u64 => _serde::__private::Ok(__Field::__field1),
                        _ => {
                            _serde::__private::Err(
                                _serde::de::Error::invalid_value(
                                    _serde::de::Unexpected::Unsigned(__value),
                                    &"field index 0 <= i < 2",
                                ),
                            )
                        }
                    }
                }
                fn visit_str<__E>(
                    self,
                    __value: &str,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        "end" => _serde::__private::Ok(__Field::__field0),
                        "start" => _serde::__private::Ok(__Field::__field1),
                        _ => {
                            _serde::__private::Err(
                                _serde::de::Error::unknown_field(__value, FIELDS),
                            )
                        }
                    }
                }
                fn visit_bytes<__E>(
                    self,
                    __value: &[u8],
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        b"end" => _serde::__private::Ok(__Field::__field0),
                        b"start" => _serde::__private::Ok(__Field::__field1),
                        _ => {
                            let __value = &_serde::__private::from_utf8_lossy(__value);
                            _serde::__private::Err(
                                _serde::de::Error::unknown_field(__value, FIELDS),
                            )
                        }
                    }
                }
            }
            impl<'de> _serde::Deserialize<'de> for __Field {
                #[inline]
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    _serde::Deserializer::deserialize_identifier(
                        __deserializer,
                        __FieldVisitor,
                    )
                }
            }
            #[doc(hidden)]
            struct __Visitor<'de> {
                marker: _serde::__private::PhantomData<CoverageDataFnsItemDecl>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = CoverageDataFnsItemDecl;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "struct CoverageDataFnsItemDecl",
                    )
                }
                #[inline]
                fn visit_seq<__A>(
                    self,
                    mut __seq: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::SeqAccess<'de>,
                {
                    let __field0 = match _serde::de::SeqAccess::next_element::<
                        f64,
                    >(&mut __seq)? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                _serde::de::Error::invalid_length(
                                    0usize,
                                    &"struct CoverageDataFnsItemDecl with 2 elements",
                                ),
                            );
                        }
                    };
                    let __field1 = match _serde::de::SeqAccess::next_element::<
                        f64,
                    >(&mut __seq)? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                _serde::de::Error::invalid_length(
                                    1usize,
                                    &"struct CoverageDataFnsItemDecl with 2 elements",
                                ),
                            );
                        }
                    };
                    _serde::__private::Ok(CoverageDataFnsItemDecl {
                        end: __field0,
                        start: __field1,
                    })
                }
                #[inline]
                fn visit_map<__A>(
                    self,
                    mut __map: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::MapAccess<'de>,
                {
                    let mut __field0: _serde::__private::Option<f64> = _serde::__private::None;
                    let mut __field1: _serde::__private::Option<f64> = _serde::__private::None;
                    while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                        __Field,
                    >(&mut __map)? {
                        match __key {
                            __Field::__field0 => {
                                if _serde::__private::Option::is_some(&__field0) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("end"),
                                    );
                                }
                                __field0 = _serde::__private::Some(
                                    _serde::de::MapAccess::next_value::<f64>(&mut __map)?,
                                );
                            }
                            __Field::__field1 => {
                                if _serde::__private::Option::is_some(&__field1) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("start"),
                                    );
                                }
                                __field1 = _serde::__private::Some(
                                    _serde::de::MapAccess::next_value::<f64>(&mut __map)?,
                                );
                            }
                        }
                    }
                    let __field0 = match __field0 {
                        _serde::__private::Some(__field0) => __field0,
                        _serde::__private::None => {
                            _serde::__private::de::missing_field("end")?
                        }
                    };
                    let __field1 = match __field1 {
                        _serde::__private::Some(__field1) => __field1,
                        _serde::__private::None => {
                            _serde::__private::de::missing_field("start")?
                        }
                    };
                    _serde::__private::Ok(CoverageDataFnsItemDecl {
                        end: __field0,
                        start: __field1,
                    })
                }
            }
            #[doc(hidden)]
            const FIELDS: &'static [&'static str] = &["end", "start"];
            _serde::Deserializer::deserialize_struct(
                __deserializer,
                "CoverageDataFnsItemDecl",
                FIELDS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<CoverageDataFnsItemDecl>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl _serde::Serialize for CoverageDataFnsItemDecl {
        fn serialize<__S>(
            &self,
            __serializer: __S,
        ) -> _serde::__private::Result<__S::Ok, __S::Error>
        where
            __S: _serde::Serializer,
        {
            let mut __serde_state = _serde::Serializer::serialize_struct(
                __serializer,
                "CoverageDataFnsItemDecl",
                false as usize + 1 + 1,
            )?;
            _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "end",
                &self.end,
            )?;
            _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "start",
                &self.start,
            )?;
            _serde::ser::SerializeStruct::end(__serde_state)
        }
    }
};
impl From<&CoverageDataFnsItemDecl> for CoverageDataFnsItemDecl {
    fn from(value: &CoverageDataFnsItemDecl) -> Self {
        value.clone()
    }
}
///CoverageDataFnsItemLoc
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "end",
///    "start"
///  ],
///  "properties": {
///    "end": {
///      "type": "number"
///    },
///    "start": {
///      "type": "number"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[serde(deny_unknown_fields)]
pub struct CoverageDataFnsItemLoc {
    pub end: f64,
    pub start: f64,
}
#[automatically_derived]
impl ::core::clone::Clone for CoverageDataFnsItemLoc {
    #[inline]
    fn clone(&self) -> CoverageDataFnsItemLoc {
        CoverageDataFnsItemLoc {
            end: ::core::clone::Clone::clone(&self.end),
            start: ::core::clone::Clone::clone(&self.start),
        }
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for CoverageDataFnsItemLoc {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field2_finish(
            f,
            "CoverageDataFnsItemLoc",
            "end",
            &self.end,
            "start",
            &&self.start,
        )
    }
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for CoverageDataFnsItemLoc {
        fn deserialize<__D>(
            __deserializer: __D,
        ) -> _serde::__private::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            #[doc(hidden)]
            enum __Field {
                __field0,
                __field1,
            }
            #[doc(hidden)]
            struct __FieldVisitor;
            impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                type Value = __Field;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "field identifier",
                    )
                }
                fn visit_u64<__E>(
                    self,
                    __value: u64,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        0u64 => _serde::__private::Ok(__Field::__field0),
                        1u64 => _serde::__private::Ok(__Field::__field1),
                        _ => {
                            _serde::__private::Err(
                                _serde::de::Error::invalid_value(
                                    _serde::de::Unexpected::Unsigned(__value),
                                    &"field index 0 <= i < 2",
                                ),
                            )
                        }
                    }
                }
                fn visit_str<__E>(
                    self,
                    __value: &str,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        "end" => _serde::__private::Ok(__Field::__field0),
                        "start" => _serde::__private::Ok(__Field::__field1),
                        _ => {
                            _serde::__private::Err(
                                _serde::de::Error::unknown_field(__value, FIELDS),
                            )
                        }
                    }
                }
                fn visit_bytes<__E>(
                    self,
                    __value: &[u8],
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        b"end" => _serde::__private::Ok(__Field::__field0),
                        b"start" => _serde::__private::Ok(__Field::__field1),
                        _ => {
                            let __value = &_serde::__private::from_utf8_lossy(__value);
                            _serde::__private::Err(
                                _serde::de::Error::unknown_field(__value, FIELDS),
                            )
                        }
                    }
                }
            }
            impl<'de> _serde::Deserialize<'de> for __Field {
                #[inline]
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    _serde::Deserializer::deserialize_identifier(
                        __deserializer,
                        __FieldVisitor,
                    )
                }
            }
            #[doc(hidden)]
            struct __Visitor<'de> {
                marker: _serde::__private::PhantomData<CoverageDataFnsItemLoc>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = CoverageDataFnsItemLoc;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "struct CoverageDataFnsItemLoc",
                    )
                }
                #[inline]
                fn visit_seq<__A>(
                    self,
                    mut __seq: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::SeqAccess<'de>,
                {
                    let __field0 = match _serde::de::SeqAccess::next_element::<
                        f64,
                    >(&mut __seq)? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                _serde::de::Error::invalid_length(
                                    0usize,
                                    &"struct CoverageDataFnsItemLoc with 2 elements",
                                ),
                            );
                        }
                    };
                    let __field1 = match _serde::de::SeqAccess::next_element::<
                        f64,
                    >(&mut __seq)? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                _serde::de::Error::invalid_length(
                                    1usize,
                                    &"struct CoverageDataFnsItemLoc with 2 elements",
                                ),
                            );
                        }
                    };
                    _serde::__private::Ok(CoverageDataFnsItemLoc {
                        end: __field0,
                        start: __field1,
                    })
                }
                #[inline]
                fn visit_map<__A>(
                    self,
                    mut __map: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::MapAccess<'de>,
                {
                    let mut __field0: _serde::__private::Option<f64> = _serde::__private::None;
                    let mut __field1: _serde::__private::Option<f64> = _serde::__private::None;
                    while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                        __Field,
                    >(&mut __map)? {
                        match __key {
                            __Field::__field0 => {
                                if _serde::__private::Option::is_some(&__field0) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("end"),
                                    );
                                }
                                __field0 = _serde::__private::Some(
                                    _serde::de::MapAccess::next_value::<f64>(&mut __map)?,
                                );
                            }
                            __Field::__field1 => {
                                if _serde::__private::Option::is_some(&__field1) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("start"),
                                    );
                                }
                                __field1 = _serde::__private::Some(
                                    _serde::de::MapAccess::next_value::<f64>(&mut __map)?,
                                );
                            }
                        }
                    }
                    let __field0 = match __field0 {
                        _serde::__private::Some(__field0) => __field0,
                        _serde::__private::None => {
                            _serde::__private::de::missing_field("end")?
                        }
                    };
                    let __field1 = match __field1 {
                        _serde::__private::Some(__field1) => __field1,
                        _serde::__private::None => {
                            _serde::__private::de::missing_field("start")?
                        }
                    };
                    _serde::__private::Ok(CoverageDataFnsItemLoc {
                        end: __field0,
                        start: __field1,
                    })
                }
            }
            #[doc(hidden)]
            const FIELDS: &'static [&'static str] = &["end", "start"];
            _serde::Deserializer::deserialize_struct(
                __deserializer,
                "CoverageDataFnsItemLoc",
                FIELDS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<CoverageDataFnsItemLoc>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl _serde::Serialize for CoverageDataFnsItemLoc {
        fn serialize<__S>(
            &self,
            __serializer: __S,
        ) -> _serde::__private::Result<__S::Ok, __S::Error>
        where
            __S: _serde::Serializer,
        {
            let mut __serde_state = _serde::Serializer::serialize_struct(
                __serializer,
                "CoverageDataFnsItemLoc",
                false as usize + 1 + 1,
            )?;
            _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "end",
                &self.end,
            )?;
            _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "start",
                &self.start,
            )?;
            _serde::ser::SerializeStruct::end(__serde_state)
        }
    }
};
impl From<&CoverageDataFnsItemLoc> for CoverageDataFnsItemLoc {
    fn from(value: &CoverageDataFnsItemLoc) -> Self {
        value.clone()
    }
}
///CoverageDataStmtsItem
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "end",
///    "start"
///  ],
///  "properties": {
///    "end": {
///      "type": "number"
///    },
///    "start": {
///      "type": "number"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[serde(deny_unknown_fields)]
pub struct CoverageDataStmtsItem {
    pub end: f64,
    pub start: f64,
}
#[automatically_derived]
impl ::core::clone::Clone for CoverageDataStmtsItem {
    #[inline]
    fn clone(&self) -> CoverageDataStmtsItem {
        CoverageDataStmtsItem {
            end: ::core::clone::Clone::clone(&self.end),
            start: ::core::clone::Clone::clone(&self.start),
        }
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for CoverageDataStmtsItem {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field2_finish(
            f,
            "CoverageDataStmtsItem",
            "end",
            &self.end,
            "start",
            &&self.start,
        )
    }
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for CoverageDataStmtsItem {
        fn deserialize<__D>(
            __deserializer: __D,
        ) -> _serde::__private::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            #[doc(hidden)]
            enum __Field {
                __field0,
                __field1,
            }
            #[doc(hidden)]
            struct __FieldVisitor;
            impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                type Value = __Field;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "field identifier",
                    )
                }
                fn visit_u64<__E>(
                    self,
                    __value: u64,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        0u64 => _serde::__private::Ok(__Field::__field0),
                        1u64 => _serde::__private::Ok(__Field::__field1),
                        _ => {
                            _serde::__private::Err(
                                _serde::de::Error::invalid_value(
                                    _serde::de::Unexpected::Unsigned(__value),
                                    &"field index 0 <= i < 2",
                                ),
                            )
                        }
                    }
                }
                fn visit_str<__E>(
                    self,
                    __value: &str,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        "end" => _serde::__private::Ok(__Field::__field0),
                        "start" => _serde::__private::Ok(__Field::__field1),
                        _ => {
                            _serde::__private::Err(
                                _serde::de::Error::unknown_field(__value, FIELDS),
                            )
                        }
                    }
                }
                fn visit_bytes<__E>(
                    self,
                    __value: &[u8],
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        b"end" => _serde::__private::Ok(__Field::__field0),
                        b"start" => _serde::__private::Ok(__Field::__field1),
                        _ => {
                            let __value = &_serde::__private::from_utf8_lossy(__value);
                            _serde::__private::Err(
                                _serde::de::Error::unknown_field(__value, FIELDS),
                            )
                        }
                    }
                }
            }
            impl<'de> _serde::Deserialize<'de> for __Field {
                #[inline]
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    _serde::Deserializer::deserialize_identifier(
                        __deserializer,
                        __FieldVisitor,
                    )
                }
            }
            #[doc(hidden)]
            struct __Visitor<'de> {
                marker: _serde::__private::PhantomData<CoverageDataStmtsItem>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = CoverageDataStmtsItem;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "struct CoverageDataStmtsItem",
                    )
                }
                #[inline]
                fn visit_seq<__A>(
                    self,
                    mut __seq: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::SeqAccess<'de>,
                {
                    let __field0 = match _serde::de::SeqAccess::next_element::<
                        f64,
                    >(&mut __seq)? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                _serde::de::Error::invalid_length(
                                    0usize,
                                    &"struct CoverageDataStmtsItem with 2 elements",
                                ),
                            );
                        }
                    };
                    let __field1 = match _serde::de::SeqAccess::next_element::<
                        f64,
                    >(&mut __seq)? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                _serde::de::Error::invalid_length(
                                    1usize,
                                    &"struct CoverageDataStmtsItem with 2 elements",
                                ),
                            );
                        }
                    };
                    _serde::__private::Ok(CoverageDataStmtsItem {
                        end: __field0,
                        start: __field1,
                    })
                }
                #[inline]
                fn visit_map<__A>(
                    self,
                    mut __map: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::MapAccess<'de>,
                {
                    let mut __field0: _serde::__private::Option<f64> = _serde::__private::None;
                    let mut __field1: _serde::__private::Option<f64> = _serde::__private::None;
                    while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                        __Field,
                    >(&mut __map)? {
                        match __key {
                            __Field::__field0 => {
                                if _serde::__private::Option::is_some(&__field0) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("end"),
                                    );
                                }
                                __field0 = _serde::__private::Some(
                                    _serde::de::MapAccess::next_value::<f64>(&mut __map)?,
                                );
                            }
                            __Field::__field1 => {
                                if _serde::__private::Option::is_some(&__field1) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("start"),
                                    );
                                }
                                __field1 = _serde::__private::Some(
                                    _serde::de::MapAccess::next_value::<f64>(&mut __map)?,
                                );
                            }
                        }
                    }
                    let __field0 = match __field0 {
                        _serde::__private::Some(__field0) => __field0,
                        _serde::__private::None => {
                            _serde::__private::de::missing_field("end")?
                        }
                    };
                    let __field1 = match __field1 {
                        _serde::__private::Some(__field1) => __field1,
                        _serde::__private::None => {
                            _serde::__private::de::missing_field("start")?
                        }
                    };
                    _serde::__private::Ok(CoverageDataStmtsItem {
                        end: __field0,
                        start: __field1,
                    })
                }
            }
            #[doc(hidden)]
            const FIELDS: &'static [&'static str] = &["end", "start"];
            _serde::Deserializer::deserialize_struct(
                __deserializer,
                "CoverageDataStmtsItem",
                FIELDS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<CoverageDataStmtsItem>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl _serde::Serialize for CoverageDataStmtsItem {
        fn serialize<__S>(
            &self,
            __serializer: __S,
        ) -> _serde::__private::Result<__S::Ok, __S::Error>
        where
            __S: _serde::Serializer,
        {
            let mut __serde_state = _serde::Serializer::serialize_struct(
                __serializer,
                "CoverageDataStmtsItem",
                false as usize + 1 + 1,
            )?;
            _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "end",
                &self.end,
            )?;
            _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "start",
                &self.start,
            )?;
            _serde::ser::SerializeStruct::end(__serde_state)
        }
    }
};
impl From<&CoverageDataStmtsItem> for CoverageDataStmtsItem {
    fn from(value: &CoverageDataStmtsItem) -> Self {
        value.clone()
    }
}
const _: &str = "{\"$ref\":\"#/definitions/coverageData\",\"definitions\":{\"coverageData\":{\"type\":\"object\",\"properties\":{\"stmts\":{\"type\":\"array\",\"items\":{\"type\":\"object\",\"properties\":{\"start\":{\"type\":\"number\"},\"end\":{\"type\":\"number\"}},\"required\":[\"start\",\"end\"],\"additionalProperties\":false}},\"fns\":{\"type\":\"array\",\"items\":{\"type\":\"object\",\"properties\":{\"name\":{\"type\":\"string\"},\"decl\":{\"type\":\"object\",\"properties\":{\"start\":{\"type\":\"number\"},\"end\":{\"type\":\"number\"}},\"required\":[\"start\",\"end\"],\"additionalProperties\":false},\"loc\":{\"type\":\"object\",\"properties\":{\"start\":{\"type\":\"number\"},\"end\":{\"type\":\"number\"}},\"required\":[\"start\",\"end\"],\"additionalProperties\":false}},\"required\":[\"decl\",\"loc\"],\"additionalProperties\":false}},\"branches\":{\"type\":\"array\",\"items\":{\"type\":\"object\",\"properties\":{\"type\":{\"type\":\"string\",\"enum\":[\"if\",\"else\"]},\"loc\":{\"type\":\"object\",\"properties\":{\"start\":{\"type\":\"number\"},\"end\":{\"type\":\"number\"}},\"required\":[\"start\",\"end\"],\"additionalProperties\":false}},\"required\":[\"type\",\"loc\"],\"additionalProperties\":false}}},\"required\":[\"stmts\",\"fns\",\"branches\"],\"additionalProperties\":false}},\"$schema\":\"http://json-schema.org/draft-07/schema#\"}";
