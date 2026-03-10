//! Nodety graph WASM wrapper.

use crate::types::JsType;
use crate::wasm::edge::EdgeWrapper;
use crate::wasm::json::NodetyGraphJson;
use crate::wasm::node::NodeWrapper;
use nodety::{inference::InferenceStep, nodety_cached::NodetyCached};
use petgraph::graph::{EdgeIndex, NodeIndex};
use petgraph::visit::{EdgeRef, IntoEdgeReferences};
use wasm_bindgen::prelude::*;

/// Directed graph of nodes and edges for type-checked programs.
///
/// Represents a program as a graph of NodeSignatures with edges between their ports.
/// Performs type inference and validation.
#[wasm_bindgen(js_name = Nodety)]
pub struct NodetyWrapper {
    inner: NodetyCached<JsType>,
}

#[wasm_bindgen(js_class = Nodety)]
impl NodetyWrapper {
    /// Create a new empty graph.
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self { inner: NodetyCached::new(InferenceStep::default_steps()) }
    }

    /// Add a node to the graph. Returns the node index.
    #[wasm_bindgen(js_name = addNode)]
    pub fn add_node(&mut self, node: NodeWrapper) -> Result<u32, JsValue> {
        self.inner.add_node(node.0).map(|idx| idx.index() as u32).map_err(|e| JsValue::from_str(&e.to_string()))
    }

    /// Update an existing node by index.
    #[wasm_bindgen(js_name = updateNode)]
    pub fn update_node(&mut self, node_idx: u32, node: NodeWrapper) -> Result<(), JsValue> {
        self.inner.update_node(NodeIndex::new(node_idx as usize), node.0).map_err(|e| JsValue::from_str(&e.to_string()))
    }

    /// Add an edge between two nodes. Returns the edge index.
    #[wasm_bindgen(js_name = addEdge)]
    pub fn add_edge(&mut self, source_node: u32, target_node: u32, edge: EdgeWrapper) -> Result<u32, JsValue> {
        self.inner
            .add_edge(NodeIndex::new(source_node as usize), NodeIndex::new(target_node as usize), edge.0)
            .map(|idx| idx.index() as u32)
            .map_err(|e| JsValue::from_str(&e.to_string()))
    }

    /// Remove a node and its edges by index.
    #[wasm_bindgen(js_name = removeNode)]
    pub fn remove_node(&mut self, node_idx: u32) -> Result<(), JsValue> {
        self.inner.remove_node(NodeIndex::new(node_idx as usize)).map_err(|e| JsValue::from_str(&e.to_string()))
    }

    /// Remove an edge by index. Returns the removed edge if it existed.
    #[wasm_bindgen(js_name = removeEdge)]
    pub fn remove_edge(&mut self, edge_idx: u32) -> Option<EdgeWrapper> {
        self.inner.remove_edge(EdgeIndex::new(edge_idx as usize)).map(EdgeWrapper)
    }

    /// Serialize the graph to JSON (nodes and edges).
    #[wasm_bindgen(js_name = toJson)]
    pub fn to_json(&self) -> Result<NodetyGraphJson, JsValue> {
        let graph = self.inner.inner();
        let nodes: Vec<String> = graph.program().node_weights().map(|n| format!("{:?}", n.signature)).collect();
        let edges: Vec<(usize, usize, (usize, usize))> = graph
            .program()
            .edge_references()
            .map(|e| {
                let edge = &graph.program()[e.id()];
                (e.source().index(), e.target().index(), (edge.source_port, edge.target_port))
            })
            .collect();
        Ok(NodetyGraphJson { nodes, edges })
    }

    /// Export the graph in DOT format for visualization.
    #[wasm_bindgen(js_name = toDot)]
    pub fn to_dot(&self) -> String {
        self.inner.to_dot()
    }

    /// Validate the graph. Returns a list of validation errors.
    #[wasm_bindgen(js_name = validate)]
    pub fn validate(&mut self) -> Vec<crate::wasm::errors::ValidationErrorWrapper> {
        self.inner.validate().into_iter().map(crate::wasm::errors::ValidationErrorWrapper).collect()
    }
}
