//! TypeExpr WASM wrapper.

use crate::types::JsType;
use crate::wasm::json::TypeExprJson;
use nodety::{
    scope::ScopePointer,
    type_expr::{TypeExpr, Unscoped},
};
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use tsify::Tsify;
use wasm_bindgen::prelude::*;

type NodetyTypeExpr = TypeExpr<JsType, Unscoped>;

/// Type expression—the core type representation in nodety.
///
/// Can represent unions, intersections, conditional types, type variables, keyof, index access,
/// node signatures (function-like types), port types, and more.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Tsify)]
#[wasm_bindgen(js_name = TypeExpr)]
pub struct TypeExprWrapper(#[wasm_bindgen(skip)] pub NodetyTypeExpr);

#[wasm_bindgen(js_class = TypeExpr)]
impl TypeExprWrapper {
    /// Parse a type expression from string notation (e.g. `"Integer"`, `"Array<String>"`).
    #[wasm_bindgen(static_method_of = TypeExprWrapper, js_name = fromString)]
    pub fn from_string(s: &str) -> Result<TypeExprWrapper, JsValue> {
        NodetyTypeExpr::from_str(s)
            .map(TypeExprWrapper)
            .map_err(|e| JsValue::from_str(&e.to_string()))
    }

    /// Parse a type expression from string notation. Alias for `fromString`.
    #[wasm_bindgen(static_method_of = TypeExprWrapper, js_name = parse)]
    pub fn parse(s: &str) -> Result<TypeExprWrapper, JsValue> {
        Self::from_string(s)
    }

    /// Create from JSON (typed object from `toJson`).
    #[wasm_bindgen(static_method_of = TypeExprWrapper, js_name = fromJson)]
    pub fn from_json(obj: TypeExprJson) -> TypeExprWrapper {
        TypeExprWrapper(obj.0)
    }

    /// Normalize the type expression (resolve conditionals, simplify unions/intersections).
    #[wasm_bindgen(js_name = normalizeNaive)]
    pub fn normalize_naive(&self) -> TypeExprWrapper {
        let scoped = self.0.clone().into_scoped();
        let scope = ScopePointer::new_root();
        let result = scoped.normalize_naive().replace_vars_by_bounds(&scope);
        TypeExprWrapper(result)
    }

    #[wasm_bindgen(js_name = toString)]
    pub fn to_string(&self) -> String {
        format!("{}", self.0)
    }

    /// Serialize to a typed JSON object.
    #[wasm_bindgen(js_name = toJson)]
    pub fn to_json(&self) -> TypeExprJson {
        TypeExprJson(self.0.clone())
    }
}
