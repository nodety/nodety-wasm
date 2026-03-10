//! PortTypes WASM wrapper.

use crate::types::JsType;
use crate::wasm::json::PortTypesJson;
use nodety::type_expr::Unscoped;
use nodety::type_expr::node_signature::port_types::PortTypes;
use serde::{Deserialize, Serialize};
use tsify::Tsify;
use wasm_bindgen::prelude::*;

type NodetyPortTypes = PortTypes<JsType, Unscoped>;

/// A list of port types, optionally with a variadic type (`...T`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Tsify)]
#[wasm_bindgen(js_name = PortTypes)]
pub struct PortTypesWrapper(#[wasm_bindgen(skip)] pub NodetyPortTypes);

#[wasm_bindgen(js_class = PortTypes)]
impl PortTypesWrapper {
    /// Create from JSON (typed object from `toJson`).
    #[wasm_bindgen(static_method_of = PortTypesWrapper, js_name = fromJson)]
    pub fn from_json(obj: PortTypesJson) -> PortTypesWrapper {
        PortTypesWrapper(obj.0)
    }

    /// Serialize to a typed JSON object.
    #[wasm_bindgen(js_name = toJson)]
    pub fn to_json(&self) -> PortTypesJson {
        PortTypesJson(self.0.clone())
    }
}
