//! Tests for TypeExpr parsing and normalization.

use nodety::type_expr::{TypeExpr, Unscoped};
use nodety_wasm::r#type::MyType;
use std::str::FromStr;

fn expr(s: &str) -> TypeExpr<MyType, Unscoped> {
    TypeExpr::from_str(s).expect(&format!("Failed to parse: {s}"))
}

#[test]
fn test_parse_integer() {
    let t = expr("Integer");
    assert!(matches!(t, TypeExpr::Type(MyType::Integer)));
}

#[test]
fn test_parse_string() {
    let t = expr("String");
    assert!(matches!(t, TypeExpr::Type(MyType::String)));
}

#[test]
fn test_parse_array() {
    let t = expr("Array");
    assert!(matches!(t, TypeExpr::Type(MyType::Array)));
}

#[test]
fn test_parse_array_generic() {
    let t = expr("Array<Integer>");
    match &t {
        TypeExpr::Constructor { inner, parameters } => {
            assert!(matches!(inner, MyType::Array));
            assert!(parameters.contains_key("elements_type"));
        }
        _ => panic!("Expected Constructor"),
    }
}

#[test]
fn test_normalize_naive() {
    let t = expr("Integer");
    let normalized = t.clone().into_scoped().normalize_naive();
    let result = normalized.replace_vars_by_bounds(&nodety::scope::ScopePointer::new_root());
    assert_eq!(format!("{}", result), "Integer");
}

#[test]
fn test_node_signature_parse() {
    use nodety::type_expr::node_signature::NodeSignature;
    let sig: NodeSignature<MyType, Unscoped> = "() -> (Integer)".parse().expect("parse");
    assert_eq!(format!("{}", sig), "() -> (Integer)");
}
