//! Parsing and formatting for JsType.
//!
//! Implements nodety's ParsableType and FormattableType so that TypeExpr and NodeSignature
//! can be parsed from and formatted to string notation.

use crate::r#type::MyType;
use nodety::{
    notation::{
        format::FormattableType,
        parse::{ParsableType, parse_type_expr},
    },
    type_expr::{TypeExpr, TypeExprScope},
};
use nom::{IResult, Parser, branch::alt, bytes::complete::tag, combinator::value};
use std::collections::BTreeMap;
use std::fmt;

impl ParsableType for MyType {
    fn parse<S: TypeExprScope>(input: &str) -> IResult<&str, TypeExpr<Self, S>> {
        alt((
            value(TypeExpr::Type(MyType::Integer), tag("Integer")),
            value(TypeExpr::Type(MyType::String), tag("String")),
            parse_array::<S>,
            value(TypeExpr::Type(MyType::Array), tag("Array")),
        ))
        .parse(input)
    }

    fn parse_operator(input: &str) -> IResult<&str, Self::Operator> {
        // NoOperator: no custom operators, always fail
        Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::Tag,
        )))
    }
}

fn parse_array<S: TypeExprScope>(input: &str) -> IResult<&str, TypeExpr<MyType, S>> {
    (
        tag("Array"),
        nom::combinator::opt((
            nom::character::complete::char('<'),
            parse_type_expr,
            nom::character::complete::char('>'),
        )),
    )
        .map(|(_, elements_type)| match elements_type {
            Some((_, elements_type, _)) => TypeExpr::Constructor {
                inner: MyType::Array,
                parameters: BTreeMap::from([("elements_type".into(), elements_type)]),
            },
            None => TypeExpr::Type(MyType::Array),
        })
        .parse(input)
}

impl FormattableType for MyType {
    fn format_type(
        &self,
        parameters: Option<&BTreeMap<String, TypeExpr<Self>>>,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::Integer => write!(f, "Integer"),
            Self::String => write!(f, "String"),
            Self::Array => {
                write!(f, "Array")?;
                if let Some(elements_type) =
                    parameters.as_ref().and_then(|p| p.get("elements_type"))
                {
                    write!(f, "<")?;
                    elements_type.format_type(f, false)?;
                    write!(f, ">")?;
                }
                Ok(())
            }
        }
    }

    fn format_operator(operator: &Self::Operator, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *operator {}
    }
}
