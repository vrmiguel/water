use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{
        i32 as parse_i32, i64 as parse_i64, multispace0,
    },
    combinator::value,
    error::context,
    number::complete::double as parse_f64,
    sequence::preceded,
    Parser,
};

// use nom::character::complete::double as parse_i64;
use super::{parse_index, parse_numerical_type, IResult};
use crate::ast::{
    Index, Instruction, NumericalType, NumericalValue,
    ScopeKind, VariableInstruction,
};

pub fn parse_instruction(input: &str) -> IResult<Instruction> {
    alt((
        parse_variable_instruction,
        parse_const.map(|value| Instruction::Constant { value }),
        context("call", parse_call).map(Instruction::Call),
    ))(input)
}

/// Parses a `const` operation, such as `i32.const 20` or
/// `f32.const 2.2`
///
/// Does not eat leading whitespace.
///
/// ```
/// use water::ast::{NumericalValue, Instruction};
/// use water::parser::parse_const;
/// use water::parser::parse_instruction;
///
/// assert_eq!(parse_const("i64.const -5"), Ok(("", NumericalValue::Int64(-5))));
/// assert_eq!(parse_instruction("i32.const 5"), Ok(("", Instruction::Constant { value: NumericalValue::Int32(5) })));
/// assert_eq!(parse_const("f64.const 5.5"), Ok(("", NumericalValue::Float64(5.5))));
/// assert_eq!(parse_const("f32.const 2E-3"), Ok(("", NumericalValue::Float32(0.002))));
/// assert_eq!(parse_instruction("f64.const 2e+5"), Ok(("", Instruction::Constant { value: NumericalValue::Float64(200000.0) })));
/// ```
pub fn parse_const(input: &str) -> IResult<NumericalValue> {
    // Parse the numerical type of this instruction: i32, i64,
    // f32 or f64
    let (rest, numerical_type) = parse_numerical_type(input)?;
    // Parse the preceding ".const" opcode
    let (rest, _) = tag(".const")(rest)?;

    match numerical_type {
        NumericalType::Int32 => {
            let (rest, int32) =
                preceded(multispace0, parse_i32)(rest)?;

            Ok((rest, NumericalValue::Int32(int32)))
        }
        NumericalType::Int64 => {
            let (rest, int64) =
                preceded(multispace0, parse_i64)(rest)?;

            Ok((rest, NumericalValue::Int64(int64)))
        }
        NumericalType::Float32 => {
            let (rest, float64) =
                preceded(multispace0, parse_f64)(rest)?;

            // TODO: parsing f32.const as f64 and then casting to
            // f32 is a hack and we should switch to using
            // `nom::number::complete::f32`
            Ok((rest, NumericalValue::Float32(float64 as f32)))
        }
        NumericalType::Float64 => {
            let (rest, float64) =
                preceded(multispace0, parse_f64)(rest)?;

            Ok((rest, NumericalValue::Float64(float64)))
        }
    }
}

/// Parses a `call` instruction alongside its index.
///
/// Does not eat leading whitespace.
///
/// ```
/// use water::ast::{Index, Instruction};
/// use water::parser::parse_call;
/// use water::parser::parse_instruction;
///
/// assert_eq!(parse_call("call 5"), Ok(("", Index::Numerical(5))));
/// assert_eq!(parse_instruction("call 5"), Ok(("", Instruction::Call(Index::Numerical(5)))));
/// assert_eq!(parse_instruction("call $func"), Ok(("", Instruction::Call(Index::Identifier("func".into())))));
/// ```
pub fn parse_call(input: &str) -> IResult<Index> {
    let (rest, _) = tag("call")(input)?;

    preceded(
        multispace0,
        context("numerical index or identifier", parse_index),
    )(rest)
}

/// Parses an instruction for direct variable access.
///
/// Does not eat leading whitespace.
///
/// ```
/// use water::ast::{ScopeKind, VariableInstruction, Instruction, Index};
/// use water::parser::parse_variable_instruction;
///
/// assert_eq!(
/// parse_variable_instruction("local.get $idx"),
/// Ok((
///     "",
///     Instruction::VariableInstruction {
///         scope: ScopeKind::Local,
///         instruction: VariableInstruction::Get(
///             Index::Identifier("idx".into())
///         ),
///     }
/// )));
/// ```
pub fn parse_variable_instruction(
    input: &str,
) -> IResult<Instruction> {
    fn parse_get_and_tee(
        input: &str,
    ) -> IResult<(VariableInstruction, ScopeKind)> {
        #[derive(Clone, PartialEq)]
        pub enum Aux {
            GlobalGet,
            LocalGet,
            LocalTee,
        }
        let parse_global_get =
            value(Aux::GlobalGet, tag("global.get"));
        let parse_local_get =
            value(Aux::LocalGet, tag("local.get"));
        let parse_local_tee =
            value(Aux::LocalTee, tag("local.tee"));

        let (rest, opcode) = alt((
            parse_global_get,
            parse_local_get,
            parse_local_tee,
        ))(input)?;

        let (rest, idx) =
            preceded(multispace0, parse_index)(rest)?;

        let (opcode, scope) = match opcode {
            Aux::GlobalGet => (
                VariableInstruction::Get(idx),
                ScopeKind::Global,
            ),
            Aux::LocalGet => {
                (VariableInstruction::Get(idx), ScopeKind::Local)
            }
            Aux::LocalTee => {
                (VariableInstruction::Tee(idx), ScopeKind::Local)
            }
        };

        Ok((rest, (opcode, scope)))
    }

    fn parse_set(
        input: &str,
    ) -> IResult<(VariableInstruction, ScopeKind)> {
        let (rest, scope) = alt((
            value(ScopeKind::Global, tag("global.set")),
            value(ScopeKind::Local, tag("local.set")),
        ))(input)?;

        let (rest, index) =
            preceded(multispace0, parse_index)(rest)?;

        Ok((
            rest,
            (
                VariableInstruction::Set { index, value: None },
                scope,
            ),
        ))
    }

    let (rest, (opcode, scope)) =
        alt((parse_set, parse_get_and_tee))(input)?;

    let instr = Instruction::VariableInstruction {
        scope,
        instruction: opcode,
    };

    Ok((rest, instr))
}
