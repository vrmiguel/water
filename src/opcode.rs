use crate::ast::{
    ArithmeticInstruction, ArithmeticOperation,
    ComparisonInstruction, ComparisonOperation, Constant,
    NumericalType, NumericalValue, Opcode, ScopeKind,
    Unreachable, VariableInstruction, VariableOperation,
};

pub trait ToOpcode {
    fn to_opcode(&self) -> u8;
}

impl ToOpcode for Unreachable {
    fn to_opcode(&self) -> u8 {
        0x00
    }
}

impl ToOpcode for NumericalValue {
    fn to_opcode(&self) -> u8 {
        match self {
            NumericalValue::Int32(_) => 0x41,
            NumericalValue::Int64(_) => 0x42,
            NumericalValue::Float32(_) => 0x43,
            NumericalValue::Float64(_) => 0x44,
        }
    }
}

impl ToOpcode for ArithmeticOperation {
    fn to_opcode(&self) -> u8 {
        let Self { type_, instr } = self;
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
}

impl ToOpcode for ComparisonOperation {
    fn to_opcode(&self) -> u8 {
        let Self { type_, instr } = self;
        match (type_, instr) {
            (
                NumericalType::Int32,
                ComparisonInstruction::Equal,
            ) => 0x45,
            (
                NumericalType::Int32,
                ComparisonInstruction::NotEqual,
            ) => 0x47,
            (
                NumericalType::Int32,
                ComparisonInstruction::GreaterThan,
            ) => todo!(),
            (
                NumericalType::Int32,
                ComparisonInstruction::LessThan,
            ) => todo!(),
            (
                NumericalType::Int32,
                ComparisonInstruction::GreaterOrEqual,
            ) => todo!(),
            (
                NumericalType::Int32,
                ComparisonInstruction::LessOrEqual,
            ) => todo!(),
            (
                NumericalType::Int64,
                ComparisonInstruction::Equal,
            ) => 0x51,
            (
                NumericalType::Int64,
                ComparisonInstruction::NotEqual,
            ) => 0x52,
            (
                NumericalType::Int64,
                ComparisonInstruction::GreaterThan,
            ) => todo!(),
            (
                NumericalType::Int64,
                ComparisonInstruction::LessThan,
            ) => todo!(),
            (
                NumericalType::Int64,
                ComparisonInstruction::GreaterOrEqual,
            ) => todo!(),
            (
                NumericalType::Int64,
                ComparisonInstruction::LessOrEqual,
            ) => todo!(),
            (
                NumericalType::Float32,
                ComparisonInstruction::Equal,
            ) => 0x5b,
            (
                NumericalType::Float32,
                ComparisonInstruction::NotEqual,
            ) => 0x5c,
            (
                NumericalType::Float32,
                ComparisonInstruction::GreaterThan,
            ) => todo!(),
            (
                NumericalType::Float32,
                ComparisonInstruction::LessThan,
            ) => todo!(),
            (
                NumericalType::Float32,
                ComparisonInstruction::GreaterOrEqual,
            ) => todo!(),
            (
                NumericalType::Float32,
                ComparisonInstruction::LessOrEqual,
            ) => todo!(),
            (
                NumericalType::Float64,
                ComparisonInstruction::Equal,
            ) => 0x61,
            (
                NumericalType::Float64,
                ComparisonInstruction::NotEqual,
            ) => 0x62,
            (
                NumericalType::Float64,
                ComparisonInstruction::GreaterThan,
            ) => todo!(),
            (
                NumericalType::Float64,
                ComparisonInstruction::LessThan,
            ) => todo!(),
            (
                NumericalType::Float64,
                ComparisonInstruction::GreaterOrEqual,
            ) => todo!(),
            (
                NumericalType::Float64,
                ComparisonInstruction::LessOrEqual,
            ) => todo!(),
        }
    }
}

impl ToOpcode for Opcode {
    fn to_opcode(&self) -> u8 {
        match self {
            Opcode::Unreachable(unreachable) => {
                unreachable.to_opcode()
            }
            Opcode::Call(_) => 0x10,
            Opcode::VariableInstruction(variable_operation) => {
                variable_operation.to_opcode()
            }
            Opcode::Constant(Constant { value }) => {
                value.to_opcode()
            }
            Opcode::Arithmetic(op) => op.to_opcode(),
            Opcode::Comparison(op) => op.to_opcode(),
        }
    }
}

impl ToOpcode for VariableOperation {
    fn to_opcode(&self) -> u8 {
        use VariableInstruction as Instr;

        let Self {
            scope, instruction, ..
        } = self;

        match (scope, instruction) {
            // local.get
            (ScopeKind::Local, Instr::Get) => 0x20,
            // local.set
            (ScopeKind::Local, Instr::Set) => 0x21,
            // local.tee
            (ScopeKind::Local, Instr::Tee) => 0x22,
            // global.get
            (ScopeKind::Global, Instr::Get) => 0x23,
            // global.set
            (ScopeKind::Global, Instr::Set) => 0x24,
            (ScopeKind::Global, Instr::Tee) => {
                unreachable!("global.tee is not supported")
            }
        }
    }
}
