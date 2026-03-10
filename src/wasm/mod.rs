//! WASM bindings for nodety-js.
//!
//! Exposes TypeExpr, NodeSignature, and other nodety entities as JS classes
//! with toJson/fromJson support.

mod edge;
mod errors;
mod json;
mod node;
mod node_signature;
mod nodety;
mod port_types;
mod scope_snapshot;
mod type_expr;
mod type_parameters;
