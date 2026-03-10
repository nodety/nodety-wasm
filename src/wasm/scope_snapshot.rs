//! ScopeSnapshot WASM wrapper.

use crate::types::JsType;
use crate::wasm::json::ScopeSnapshotJson;
use nodety::scope::LocalParamID;
use nodety::type_expr::node_signature::type_parameters::TypeParameters;
use nodety::type_expr::{TypeExpr, Unscoped};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use tsify::Tsify;
use wasm_bindgen::prelude::*;

/// Snapshot of a scope's parameters and inferred type variables.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Tsify)]
#[wasm_bindgen(js_name = ScopeSnapshot)]
pub struct ScopeSnapshotWrapper {
    #[wasm_bindgen(skip)]
    pub parameters: TypeParameters<JsType, Unscoped>,
    #[wasm_bindgen(skip)]
    pub inferred: BTreeMap<LocalParamID, TypeExpr<JsType, Unscoped>>,
}

#[wasm_bindgen(js_class = ScopeSnapshot)]
impl ScopeSnapshotWrapper {
    /// Create from JSON (typed object from `toJson`).
    #[wasm_bindgen(static_method_of = ScopeSnapshotWrapper, js_name = fromJson)]
    pub fn from_json(obj: ScopeSnapshotJson) -> ScopeSnapshotWrapper {
        ScopeSnapshotWrapper { parameters: obj.parameters, inferred: obj.inferred }
    }

    /// Serialize to a typed JSON object.
    #[wasm_bindgen(js_name = toJson)]
    pub fn to_json(&self) -> ScopeSnapshotJson {
        ScopeSnapshotJson { parameters: self.parameters.clone(), inferred: self.inferred.clone() }
    }
}
