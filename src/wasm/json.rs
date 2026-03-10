//! JSON types with Tsify for type-safe toJson/fromJson.
//!
//! Each type has a corresponding `*Json` newtype that implements
//! `#[tsify(into_wasm_abi, from_wasm_abi)]` for typed TS signatures.

use crate::types::JsType;
use nodety::Edge;
use nodety::Node;
use nodety::notation::parse::ParseError;
use nodety::scope::{LocalParamID, type_parameter::TypeParameter};
use nodety::type_expr::node_signature::{port_types::PortTypes, type_parameters::TypeParameters};
use nodety::type_expr::{TypeExpr, Unscoped, node_signature::NodeSignature};
use nodety::validation::ValidationError;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use tsify::Tsify;

macro_rules! json_type {
    ($name:ident, $inner:ty, #[doc = $doc:expr]) => {
        #[doc = $doc]
        #[derive(Clone, Serialize, Deserialize, Tsify)]
        #[serde(transparent)]
        #[tsify(into_wasm_abi, from_wasm_abi)]
        pub struct $name(pub $inner);
    };
}

json_type!(
    TypeExprJson,
    TypeExpr<JsType, Unscoped>,
    #[doc = "Type expression—the core type representation. Unions, intersections, conditional types, type variables, keyof, index access, node signatures (function-like types), port types, and more."]
);

json_type!(
    NodeSignatureJson,
    NodeSignature<JsType, Unscoped>,
    #[doc = "Function-like type for a node: generic parameters, inputs, outputs, and defaults. Written in notation as `<T>(T) -> (T)` for the identity node."]
);

json_type!(
    TypeParameterJson,
    TypeParameter<JsType, Unscoped>,
    #[doc = "A generic type parameter with optional bound and default."]
);

json_type!(
    TypeParametersJson,
    TypeParameters<JsType, Unscoped>,
    #[doc = "Generic parameters for a node signature (wrapper for BTreeMap)."]
);

json_type!(
    LocalParamIDJson,
    LocalParamID,
    #[doc = "Identifies a type parameter within a type expression or node signature. Can be created from a single character (e.g. `'T'`) or a string (hashed for multi-char names)."]
);

json_type!(
    PortTypesJson,
    PortTypes<JsType, Unscoped>,
    #[doc = "A list of port types, optionally with a variadic type (`...T`)."]
);

json_type!(
    EdgeJson,
    Edge,
    #[doc = "An edge connecting a source output port to a target input port."]
);

json_type!(
    NodeJson,
    Node<JsType, Unscoped>,
    #[doc = "A node in the nodety graph with signature, optional parent, and type hints."]
);

json_type!(
    ParseErrorJson,
    ParseError,
    #[doc = "Error returned when parsing a type expression or node signature fails."]
);

json_type!(
    ValidationErrorJson,
    ValidationError<JsType>,
    #[doc = "A validation error with its location and kind."]
);

/// Snapshot of a scope's parameters and inferred type variables.
#[derive(Clone, Serialize, Deserialize, Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct ScopeSnapshotJson {
    pub parameters: TypeParameters<JsType, Unscoped>,
    pub inferred: BTreeMap<LocalParamID, TypeExpr<JsType, Unscoped>>,
}

/// Graph serialization: nodes (signature strings) and edges (source, target, port indices).
#[derive(Clone, Serialize, Deserialize, Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct NodetyGraphJson {
    pub nodes: Vec<String>,
    pub edges: Vec<(usize, usize, (usize, usize))>,
}
