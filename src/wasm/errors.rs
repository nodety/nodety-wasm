//! ValidationError and ParseError WASM wrappers.

use crate::types::JsType;
use crate::wasm::json::{ParseErrorJson, ValidationErrorJson};
use nodety::notation::parse::ParseError;
use nodety::validation::ValidationError;
use serde::{Deserialize, Serialize};
use tsify::Tsify;
use wasm_bindgen::prelude::*;

/// Error returned when parsing a type expression or node signature fails.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Tsify)]
#[wasm_bindgen(js_name = ParseError)]
pub struct ParseErrorWrapper(#[wasm_bindgen(skip)] pub ParseError);

#[wasm_bindgen(js_class = ParseError)]
impl ParseErrorWrapper {
    /// Create from JSON (typed object from `toJson`).
    #[wasm_bindgen(static_method_of = ParseErrorWrapper, js_name = fromJson)]
    pub fn from_json(obj: ParseErrorJson) -> ParseErrorWrapper {
        ParseErrorWrapper(obj.0)
    }

    #[wasm_bindgen(js_name = toJson)]
    pub fn to_json(&self) -> ParseErrorJson {
        ParseErrorJson(self.0.clone())
    }
}

/// A validation error with its location and kind.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Tsify)]
#[wasm_bindgen(js_name = ValidationError)]
pub struct ValidationErrorWrapper(#[wasm_bindgen(skip)] pub ValidationError<JsType>);

#[wasm_bindgen(js_class = ValidationError)]
impl ValidationErrorWrapper {
    /// Create from JSON (typed object from `toJson`).
    #[wasm_bindgen(static_method_of = ValidationErrorWrapper, js_name = fromJson)]
    pub fn from_json(obj: ValidationErrorJson) -> ValidationErrorWrapper {
        ValidationErrorWrapper(obj.0)
    }

    #[wasm_bindgen(js_name = toJson)]
    pub fn to_json(&self) -> ValidationErrorJson {
        ValidationErrorJson(self.0.clone())
    }
}
