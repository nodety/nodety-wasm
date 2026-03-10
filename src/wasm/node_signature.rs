//! NodeSignature WASM wrapper.

use crate::types::JsType;
use crate::wasm::json::NodeSignatureJson;
use nodety::type_expr::Unscoped;
use nodety::type_expr::node_signature::NodeSignature;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use tsify::Tsify;
use wasm_bindgen::prelude::*;

type NodetyNodeSignature = NodeSignature<JsType, Unscoped>;

/// Function-like type for a node: generic parameters, inputs, outputs, and defaults.
/// Written in notation as `<T>(T) -> (T)` for the identity node.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Tsify)]
#[wasm_bindgen(js_name = NodeSignature)]
pub struct NodeSignatureWrapper(#[wasm_bindgen(skip)] pub NodetyNodeSignature);

#[wasm_bindgen(js_class = NodeSignature)]
impl NodeSignatureWrapper {
    /// Parse a node signature from string notation (e.g. `"<T>(T) -> (T)"`).
    #[wasm_bindgen(static_method_of = NodeSignatureWrapper, js_name = fromString)]
    pub fn from_string(s: &str) -> Result<NodeSignatureWrapper, JsValue> {
        NodetyNodeSignature::from_str(s).map(NodeSignatureWrapper).map_err(|e| JsValue::from_str(&e.to_string()))
    }

    /// Create from JSON (typed object from `toJson`).
    #[wasm_bindgen(static_method_of = NodeSignatureWrapper, js_name = fromJson)]
    pub fn from_json(obj: NodeSignatureJson) -> NodeSignatureWrapper {
        NodeSignatureWrapper(obj.0)
    }

    #[wasm_bindgen(js_name = toString)]
    pub fn to_string(&self) -> String {
        format!("{}", self.0)
    }

    /// Serialize to a typed JSON object.
    #[wasm_bindgen(js_name = toJson)]
    pub fn to_json(&self) -> NodeSignatureJson {
        NodeSignatureJson(self.0.clone())
    }
}
