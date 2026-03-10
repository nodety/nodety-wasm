//! Simple type system for nodety-js: Integer, String, Array.
//!
//! Kept separate from WASM bindings so the type logic can be reused by other crates.

use nodety::{
    NoOperator,
    type_expr::{ScopedTypeExpr, TypeExpr},
};
use std::collections::BTreeMap;

#[cfg(feature = "wasm")]
use serde::{Deserialize, Serialize};
#[cfg(feature = "wasm")]
use tsify::Tsify;

/// Minimal type system for JS/WASM: Integer, String, Array.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "wasm", derive(Serialize, Deserialize, Tsify))]
pub enum MyType {
    Integer,
    String,
    /// Generic: `Array<T>`. Use Constructor with "elements_type" parameter.
    Array,
}

impl nodety::Type for MyType {
    type Operator = NoOperator;

    fn supertype_of(&self, child: &Self) -> bool {
        self == child
    }

    fn key_type(&self, _fields: Option<&BTreeMap<String, ScopedTypeExpr<Self>>>) -> ScopedTypeExpr<Self> {
        match self {
            Self::Array => TypeExpr::Type(Self::Integer),
            _ => TypeExpr::Never,
        }
    }

    fn index(
        &self,
        fields: Option<&BTreeMap<String, ScopedTypeExpr<Self>>>,
        index: &ScopedTypeExpr<Self>,
    ) -> ScopedTypeExpr<Self> {
        match (self, fields, index) {
            (Self::Array, Some(fields), index) => {
                if !matches!(index, TypeExpr::Type(Self::Integer)) {
                    return TypeExpr::Any;
                }
                fields.get("elements_type").cloned().unwrap_or(TypeExpr::Any)
            }
            _ => TypeExpr::Any,
        }
    }
}
