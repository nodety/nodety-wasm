//! TypeParameters, TypeParameter, LocalParamID WASM wrappers.

use crate::types::JsType;
use crate::wasm::json::{LocalParamIDJson, TypeParameterJson, TypeParametersJson};
use nodety::scope::{LocalParamID, type_parameter::TypeParameter};
use nodety::type_expr::Unscoped;
use nodety::type_expr::node_signature::type_parameters::TypeParameters;
use serde::{Deserialize, Serialize};
use tsify::Tsify;
use wasm_bindgen::prelude::*;

type NodetyTypeParameter = TypeParameter<JsType, Unscoped>;
type NodetyTypeParameters = TypeParameters<JsType, Unscoped>;

/// A generic type parameter with optional bound and default.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Tsify)]
#[wasm_bindgen(js_name = TypeParameter)]
pub struct TypeParameterWrapper(#[wasm_bindgen(skip)] pub NodetyTypeParameter);

#[wasm_bindgen(js_class = TypeParameter)]
impl TypeParameterWrapper {
    /// Create from JSON (typed object from `toJson`).
    #[wasm_bindgen(static_method_of = TypeParameterWrapper, js_name = fromJson)]
    pub fn from_json(obj: TypeParameterJson) -> TypeParameterWrapper {
        TypeParameterWrapper(obj.0)
    }

    #[wasm_bindgen(js_name = toJson)]
    pub fn to_json(&self) -> TypeParameterJson {
        TypeParameterJson(self.0.clone())
    }
}

/// Generic parameters for a node signature (map of LocalParamID to TypeParameter).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Tsify)]
#[wasm_bindgen(js_name = TypeParameters)]
pub struct TypeParametersWrapper(#[wasm_bindgen(skip)] pub NodetyTypeParameters);

#[wasm_bindgen(js_class = TypeParameters)]
impl TypeParametersWrapper {
    /// Create from JSON (typed object from `toJson`).
    #[wasm_bindgen(static_method_of = TypeParametersWrapper, js_name = fromJson)]
    pub fn from_json(obj: TypeParametersJson) -> TypeParametersWrapper {
        TypeParametersWrapper(obj.0)
    }

    #[wasm_bindgen(js_name = toJson)]
    pub fn to_json(&self) -> TypeParametersJson {
        TypeParametersJson(self.0.clone())
    }
}

/// Identifies a type parameter within a type expression or node signature.
/// Can be created from a single character (e.g. `'T'`) or a string (hashed for multi-char names).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Tsify)]
#[wasm_bindgen(js_name = LocalParamID)]
pub struct LocalParamIDWrapper(#[wasm_bindgen(skip)] pub LocalParamID);

#[wasm_bindgen(js_class = LocalParamID)]
impl LocalParamIDWrapper {
    /// Create from a string (single char or multi-char, hashed).
    #[wasm_bindgen(static_method_of = LocalParamIDWrapper, js_name = fromString)]
    pub fn from_string(s: &str) -> LocalParamIDWrapper {
        LocalParamIDWrapper(s.into())
    }

    /// Create from JSON (typed object from `toJson`).
    #[wasm_bindgen(static_method_of = LocalParamIDWrapper, js_name = fromJson)]
    pub fn from_json(obj: LocalParamIDJson) -> LocalParamIDWrapper {
        LocalParamIDWrapper(obj.0)
    }

    #[wasm_bindgen(js_name = toJson)]
    pub fn to_json(&self) -> LocalParamIDJson {
        LocalParamIDJson(self.0)
    }
}
