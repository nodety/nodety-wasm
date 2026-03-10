//! Edge WASM wrapper.

use crate::wasm::json::EdgeJson;
use nodety::Edge;
use serde::{Deserialize, Serialize};
use tsify::Tsify;
use wasm_bindgen::prelude::*;

/// An edge connecting a source output port to a target input port.
#[derive(Debug, Clone, Serialize, Deserialize, Tsify)]
#[wasm_bindgen(js_name = Edge)]
pub struct EdgeWrapper(#[wasm_bindgen(skip)] pub Edge);

#[wasm_bindgen(js_class = Edge)]
impl EdgeWrapper {
    /// Create an edge from source output port index to target input port index.
    #[wasm_bindgen(constructor)]
    pub fn new(source_port: usize, target_port: usize) -> EdgeWrapper {
        EdgeWrapper(Edge {
            source_port,
            target_port,
        })
    }

    /// Create from JSON (typed object from `toJson`).
    #[wasm_bindgen(static_method_of = EdgeWrapper, js_name = fromJson)]
    pub fn from_json(obj: EdgeJson) -> EdgeWrapper {
        EdgeWrapper(obj.0)
    }

    /// Serialize to a typed JSON object.
    #[wasm_bindgen(js_name = toJson)]
    pub fn to_json(&self) -> EdgeJson {
        EdgeJson(self.0.clone())
    }
}
