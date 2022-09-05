//! A parser for WebAssembly Text Format.
//!
//! Functions are mostly all public as to allow doc-tests.

mod instruction;

pub use instruction::*;
use nom::{
    branch::alt,
    bytes::complete::{tag, take_while1},
    character::complete::{char, multispace0},
    combinator::{cut, opt, value},
    error::{context, VerboseError},
    sequence::{delimited, preceded},
    Parser,
};

use crate::{
    ast::{
        Function, Index, Local, Module, NumericalType,
        Parameter, Type,
    },
    small_string::SmallString,
};

/// The result of a parsing operation with added error context
pub type IResult<'a, T> =
    nom::IResult<&'a str, T, VerboseError<&'a str>>;

/// Parses a WebAssembly Text Format module.
///
/// Eats leading whitespace before and after the first
/// parenthesis.
///
/// ```
/// use water::parser::parse_module;
///
/// assert!(parse_module("(module)").is_ok());
/// assert!(parse_module("\n  (module)").is_ok());
///
/// assert!(parse_module(" (   module").is_err());
/// assert!(parse_module("module)").is_err());
/// assert!(parse_module("(mod)").is_err());
/// ```
pub fn parse_module(input: &str) -> IResult<Module> {
    fn inner(input: &str) -> IResult<Module> {
        let (rest, _) =
            preceded(multispace0, tag("module"))(input)?;

        Ok((rest, Module {}))
    }

    preceded(
        multispace0,
        parse_parenthesis_enclosed(context("module", inner)),
    )(input)
}

/// Parses a function definition
///
/// ```
/// use water::parser::parse_function;
/// use water::ast::{Function, Parameter, Local, Type, NumericalType};
///
/// let parameters = vec![
///     Parameter {
///         identifier: Some("number".into()),
///         type_: Type::Numerical(NumericalType::Float64)
///     },
///     Parameter {
///         identifier: None,
///         type_: Type::Numerical(NumericalType::Int64)
///     },
/// ];
///
/// let local_variables = vec![
///     Local {
///         identifier: Some("l1".into()),
///         type_: Type::Numerical(NumericalType::Int32)
///     },
///     Local {
///         identifier: None,
///         type_: Type::Numerical(NumericalType::Float32)
///     },
/// ];
///
/// let function = Function { identifier: Some("add".into()), parameters, local_variables };
///
/// assert_eq!(
///     parse_function("(func $add (param $number f64) (param i64) (local $l1 i32) (local f32))"),
///     Ok(("", function))
/// );
/// ```
pub fn parse_function(input: &str) -> IResult<Function> {
    fn inner(input: &str) -> IResult<Function> {
        let mut parameters = Vec::new();
        let mut local_variables = Vec::new();

        let (rest, _) =
            preceded(multispace0, tag("func"))(input)?;

        let (mut rest, identifier) =
            preceded(multispace0, opt(parse_identifier))(rest)?;

        while let Ok((new_rest, parameter)) =
            parse_parameter(rest)
        {
            rest = new_rest;
            parameters.push(parameter);
        }

        while let Ok((new_rest, local)) = parse_local(rest) {
            rest = new_rest;
            local_variables.push(local);
        }

        let function = Function {
            identifier,
            parameters,
            local_variables,
        };

        Ok((rest, function))
    }

    parse_parenthesis_enclosed(context("function", inner))(input)
}

/// Parses a function parameter.
///
/// ```
/// use water::ast::{Parameter, Type, NumericalType};
/// use water::parser::parse_parameter;
///
/// let anonymous_i32 = Parameter {
///     identifier: None,
///     type_: Type::Numerical(NumericalType::Int32)
/// };
///
/// let named_f64 = Parameter {
///     identifier: Some("number".into()),
///     type_: Type::Numerical(NumericalType::Float64)
/// };
///
/// assert_eq!(parse_parameter("(param i32)"), Ok(("", anonymous_i32)));
/// assert_eq!(parse_parameter("( param $number f64)"), Ok(("", named_f64)));
/// ```
// TODO: handle cases such as (param f32 f32)
pub fn parse_parameter(input: &str) -> IResult<Parameter> {
    fn inner(input: &str) -> IResult<Parameter> {
        let (rest, _) =
            preceded(multispace0, tag("param"))(input)?;
        let (rest, identifier) =
            opt(preceded(multispace0, parse_identifier))(rest)?;
        let (rest, type_) =
            preceded(multispace0, parse_type)(rest)?;

        let parameter = Parameter { identifier, type_ };

        Ok((rest, parameter))
    }

    preceded(
        multispace0,
        parse_parenthesis_enclosed(context("parameter", inner)),
    )(input)
}

/// Parses a local variable definition.
///
/// ```
/// use water::ast::{Local, Type, NumericalType};
/// use water::parser::parse_local;
/// use water::small_string::SmallString;
///
/// let anonymous_f32 = Local {
///     identifier: None,
///     type_: Type::Numerical(NumericalType::Float32)
/// };
///
/// let named_i64 = Local {
///     identifier: Some("number".into()),
///     type_: Type::Numerical(NumericalType::Int64)
/// };
///
/// assert_eq!(parse_local("(local f32)"), Ok(("", anonymous_f32)));
/// assert_eq!(parse_local("( local $number i64)"), Ok(("", named_i64)));
/// ```
pub fn parse_local(input: &str) -> IResult<Local> {
    fn inner(input: &str) -> IResult<Local> {
        let (rest, _) =
            preceded(multispace0, tag("local"))(input)?;
        let (rest, identifier) =
            opt(preceded(multispace0, parse_identifier))(rest)?;
        let (rest, type_) =
            preceded(multispace0, parse_type)(rest)?;

        let local = Local { identifier, type_ };

        Ok((rest, local))
    }

    preceded(
        multispace0,
        parse_parenthesis_enclosed(context("local", inner)),
    )(input)
}

/// Parses an identifier. WebAssembly Text Format identifiers
/// always start with `$`.
///
/// Does not eat leading whitespace.
///
/// ```
/// use water::parser::parse_identifier;
/// use water::small_string::SmallString;
///
/// assert_eq!(parse_identifier("$idx"), Ok(("", SmallString::new("idx"))));
/// assert_eq!(parse_identifier("$asd_aa? a"), Ok((" a", SmallString::new("asd_aa?"))));
/// ```
pub fn parse_identifier(input: &str) -> IResult<SmallString> {
    let (rest, identifier) = context(
        "identifier",
        preceded(
            char('$'),
            take_while1(is_acceptable_identifier_character),
        ),
    )(input)?;

    Ok((rest, SmallString::new(identifier)))
}

/// Parses a WASM type.
///
/// Does not eat leading whitespace.
pub fn parse_type(input: &str) -> IResult<Type> {
    context(
        "type",
        alt((parse_numerical_type.map(Type::Numerical),)),
    )(input)
}

/// Parses one of the four built-in numerical WASM types.
///
/// Does not eat leading whitespace.
pub fn parse_numerical_type(
    input: &str,
) -> IResult<NumericalType> {
    alt((
        value(NumericalType::Int32, tag("i32")),
        value(NumericalType::Int64, tag("i64")),
        value(NumericalType::Float32, tag("f32")),
        value(NumericalType::Float64, tag("f64")),
    ))(input)
}

/// Parses an index, either numerical or as an identifier.
///
/// Does not eat leading whitespace.
///
/// ```
/// use water::parser::parse_index;
/// use water::small_string::SmallString;
/// use water::ast::Index;
///
/// assert_eq!(parse_index("$var"), Ok(("", Index::Identifier("var".into()))));
/// assert_eq!(parse_index("5"), Ok(("", Index::Numerical(5))));
/// ```
pub fn parse_index(input: &str) -> IResult<Index> {
    alt((
        parse_identifier
            .map(SmallString::new)
            .map(Index::Identifier),
        nom::character::complete::i64.map(Index::Numerical),
    ))(input)
}

// Based on https://github.com/Geal/nom/blob/761ab0a24fccb4c560367b583b608fbae5f31647/examples/s_expression.rs#L155
fn parse_parenthesis_enclosed<'a, T, F>(
    inner: F,
) -> impl FnMut(&'a str) -> IResult<T>
where
    F: Parser<&'a str, T, VerboseError<&'a str>>,
{
    delimited(
        char('('),
        preceded(multispace0, inner),
        context(
            "closing parenthesis",
            cut(preceded(multispace0, char(')'))),
        ),
    )
}

fn is_acceptable_identifier_character(ch: char) -> bool {
    ch.is_ascii_alphanumeric()
        || matches!(
            ch,
            '!' | '#'
                | '$'
                | '%'
                | '&'
                | '´'
                | '*'
                | '+'
                | '-'
                | '.'
                | '/'
                | ':'
                | '<'
                | '='
                | '>'
                | '?'
                | '@'
                | '\\'
                | '^'
                | '_'
                | '`'
                | '|'
                | '~'
        )
}
