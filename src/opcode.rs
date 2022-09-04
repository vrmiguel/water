use crate::ast::{
    ArithmeticInstruction, NumericalType, NumericalValue,
    Opcode, ScopeKind, VariableInstruction,
};

pub trait ToOpcode {
    fn to_opcode(&self) -> u8;
}

impl ToOpcode for Opcode {
    fn to_opcode(&self) -> u8 {
        match self {
            Opcode::Unreachable => 0x00,
            Opcode::Call(_) => 0x10,
            Opcode::VariableInstruction {
                scope,
                instruction,
                ..
            } => match (scope, instruction) {
                (ScopeKind::Local, VariableInstruction::Get) => {
                    0x20
                }
                (ScopeKind::Local, VariableInstruction::Set) => {
                    0x21
                }
                (ScopeKind::Local, VariableInstruction::Tee) => {
                    0x22
                }
                (
                    ScopeKind::Global,
                    VariableInstruction::Get,
                ) => 0x23,
                (
                    ScopeKind::Global,
                    VariableInstruction::Set,
                ) => 0x24,
                (
                    ScopeKind::Global,
                    VariableInstruction::Tee,
                ) => unreachable!("global.tee is not supported"),
            },
            Opcode::Constant { value } => match value {
                NumericalValue::Int32(_) => 0x41,
                NumericalValue::Int64(_) => 0x42,
                NumericalValue::Float32(_) => 0x43,
                NumericalValue::Float64(_) => 0x44,
            },
            Opcode::Arithmetic { type_, instr } => {
                match (type_, instr) {
                    (
                        NumericalType::Int32,
                        ArithmeticInstruction::Addition,
                    ) => 0x6a,
                    (
                        NumericalType::Int32,
                        ArithmeticInstruction::Subtraction,
                    ) => 0x6b,
                    (
                        NumericalType::Int32,
                        ArithmeticInstruction::Multiplication,
                    ) => 0x6c,
                    (
                        NumericalType::Int32,
                        ArithmeticInstruction::SignedDivision,
                    ) => 0x6d,
                    (
                        NumericalType::Int32,
                        ArithmeticInstruction::UnsignedDisivion,
                    ) => 0x6e,
                    (
                        NumericalType::Int32,
                        ArithmeticInstruction::SignedRemainder,
                    ) => 0x6f,
                    (
                        NumericalType::Int32,
                        ArithmeticInstruction::UnsignedRemainder,
                    ) => 0x70,
                    (
                        NumericalType::Int64,
                        ArithmeticInstruction::Addition,
                    ) => 0x7c,
                    (
                        NumericalType::Int64,
                        ArithmeticInstruction::Subtraction,
                    ) => 0x7d,
                    (
                        NumericalType::Int64,
                        ArithmeticInstruction::Multiplication,
                    ) => 0x7e,
                    (
                        NumericalType::Int64,
                        ArithmeticInstruction::SignedDivision,
                    ) => 0x7f,
                    (
                        NumericalType::Int64,
                        ArithmeticInstruction::UnsignedDisivion,
                    ) => 0x80,
                    (
                        NumericalType::Int64,
                        ArithmeticInstruction::SignedRemainder,
                    ) => 0x81,
                    (
                        NumericalType::Int64,
                        ArithmeticInstruction::UnsignedRemainder,
                    ) => 0x82,
                    (
                        NumericalType::Int32
                        | NumericalType::Int64,
                        ArithmeticInstruction::FloatDivision,
                    ) => unreachable!(
                        "no float division for integers"
                    ),
                    (
                        NumericalType::Float32,
                        ArithmeticInstruction::Addition,
                    ) => 0x92,
                    (
                        NumericalType::Float32,
                        ArithmeticInstruction::Subtraction,
                    ) => 0x93,
                    (
                        NumericalType::Float32,
                        ArithmeticInstruction::Multiplication,
                    ) => 0x94,
                    (
                        NumericalType::Float32,
                        ArithmeticInstruction::FloatDivision,
                    ) => 0x95,
                    (
                        NumericalType::Float64,
                        ArithmeticInstruction::Addition,
                    ) => 0xa0,
                    (
                        NumericalType::Float64,
                        ArithmeticInstruction::Subtraction,
                    ) => 0xa1,
                    (
                        NumericalType::Float64,
                        ArithmeticInstruction::Multiplication,
                    ) => 0xa2,
                    (
                        NumericalType::Float64,
                        ArithmeticInstruction::FloatDivision,
                    ) => 0xa3,
                    (
                        NumericalType::Float32
                        | NumericalType::Float64,
                        ArithmeticInstruction::UnsignedDisivion
                        | ArithmeticInstruction::SignedDivision,
                    ) => unreachable!("no signed or unsigned division for floating numbers"),
                    (
                        NumericalType::Float32 | NumericalType::Float64,
                        ArithmeticInstruction::SignedRemainder | ArithmeticInstruction::UnsignedRemainder,
                    ) => unreachable!("no remainder instruction for floating numbers"),
                }
            }
            Opcode::Comparison { type_, instr } => {
                let (_, _) = (type_, instr);
                todo!()
            }
        }
    }
}
