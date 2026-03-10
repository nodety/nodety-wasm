//! Node WASM wrapper.

use crate::types::JsType;
use crate::wasm::json::NodeJson;
use nodety::Node;
use nodety::type_expr::Unscoped;
use serde::{Deserialize, Serialize};
use tsify::Tsify;
use wasm_bindgen::prelude::*;

type NodetyNode = Node<JsType, Unscoped>;

/// A node in the nodety graph with signature, optional parent, and type hints.
#[derive(Debug, Clone, Serialize, Deserialize, Tsify)]
#[wasm_bindgen(js_name = Node)]
pub struct NodeWrapper(#[wasm_bindgen(skip)] pub NodetyNode);

#[wasm_bindgen(js_class = Node)]
impl NodeWrapper {
    /// Create from JSON (typed object from `toJson`).
    #[wasm_bindgen(static_method_of = NodeWrapper, js_name = fromJson)]
    pub fn from_json(obj: NodeJson) -> NodeWrapper {
        NodeWrapper(obj.0)
    }

    /// Serialize to a typed JSON object.
    #[wasm_bindgen(js_name = toJson)]
    pub fn to_json(&self) -> NodeJson {
        NodeJson(self.0.clone())
    }
}
