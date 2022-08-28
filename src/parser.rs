use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alphanumeric1, char, multispace0},
    combinator::{cut, opt, value},
    error::{context, VerboseError},
    sequence::{delimited, preceded},
    Parser,
};

use crate::small_string::SmallString;

/// The result of a parsing operation with added error context
pub type IResult<'a, T> = nom::IResult<&'a str, T, VerboseError<&'a str>>;

pub struct Module {}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum WasmType {
    I32,
    I64,
    F32,
    F64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Parameter {
    /// The identifier of this parameter. May not be present, in which case
    /// the local must be accessed through its index.
    pub identifier: Option<SmallString>,
    /// The type of this parameter
    pub type_: WasmType,
}

/// Parses a WebAssembly Text Format module
///
/// ```
/// use waster::parser::parse_module;
///
/// assert!(parse_module("(module)").is_ok());
/// ```
pub fn parse_module(input: &str) -> IResult<Module> {
    fn inner(input: &str) -> IResult<Module> {
        let (rest, _) = preceded(multispace0, tag("module"))(input)?;

        Ok((rest, Module {}))
    }

    parse_parenthesis_enclosed(inner)(input)
}

/// Parses a function parameter.
///
/// Eats whitespace.
///
/// ```
/// use waster::parser::{Parameter, WasmType, parse_parameter};
/// use waster::small_string::SmallString;
///
/// let _i32 = Parameter {
///     identifier: None,
///     type_: WasmType::I32
/// };
///
/// let named_f64 = Parameter {
///     identifier: Some("number".into()),
///     type_: WasmType::F64
/// };
///
/// assert_eq!(parse_parameter("(param $number f64)"), Ok(("", named_f64)));
///
/// ```
pub fn parse_parameter(input: &str) -> IResult<Parameter> {
    fn inner(input: &str) -> IResult<Parameter> {
        let (rest, _) = preceded(multispace0, tag("param"))(input)?;
        let (rest, identifier) = opt(preceded(multispace0, parse_identifier))(rest)?;
        let (rest, type_) = preceded(multispace0, parse_type)(rest)?;

        let parameter = Parameter { identifier, type_ };

        Ok((rest, parameter))
    }

    parse_parenthesis_enclosed(inner)(input)
}

/// Parses an identifier. WebAssembly Text Format identifiers always start with `$`.
///
/// Does not eat leading whitespace.
///
/// ```
/// use waster::parser::parse_identifier;
/// use waster::small_string::SmallString;
///
/// assert_eq!(parse_identifier("$idx"), Ok(("", SmallString::new("idx"))));
/// ```
pub fn parse_identifier(input: &str) -> IResult<SmallString> {
    // TODO: can identifiers start with a digit? As in `$1a`
    let (rest, identifier) = preceded(char('$'), alphanumeric1)(input)?;

    Ok((rest, SmallString::new(identifier)))
}

/// Parses a WASM type.
///
/// Does not eat leading whitespace.
pub fn parse_type(input: &str) -> IResult<WasmType> {
    alt((
        value(WasmType::I32, tag("i32")),
        value(WasmType::I64, tag("i64")),
        value(WasmType::F32, tag("f32")),
        value(WasmType::F64, tag("f64")),
    ))(input)
}

// Based on https://github.com/Geal/nom/blob/761ab0a24fccb4c560367b583b608fbae5f31647/examples/s_expression.rs#L155
fn parse_parenthesis_enclosed<'a, T, F>(inner: F) -> impl FnMut(&'a str) -> IResult<T>
where
    F: Parser<&'a str, T, VerboseError<&'a str>>,
{
    delimited(
        char('('),
        preceded(multispace0, inner),
        context("closing parenthesis", cut(preceded(multispace0, char(')')))),
    )
}
