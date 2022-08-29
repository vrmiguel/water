//! The abstract syntax tree `waster` parses to.

use crate::small_string::SmallString;

/// Represents a WebAssembly Text Format module
pub struct Module {}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Type {
    Numerical(NumericalType),
}

#[derive(Clone, Debug, PartialEq, Eq)]
/// The four built-in WebAssembly numerical types.
pub enum NumericalType {
    /// Signed integer of 32 bits
    Int32,
    /// Signed integer of 64 bits
    Int64,
    /// Floating-number of 32 bits
    Float32,
    /// Floating-number of 64 bits
    Float64,
}

/// The same as [`NumericalType`] but actually carries a value
/// that it represents
#[derive(Clone, Copy, Debug, PartialEq)]

pub enum NumericalValue {
    /// Signed integer of 32 bits
    Int32(i32),
    /// Signed integer of 64 bits
    Int64(i64),
    /// Floating-number of 32 bits
    Float32(f32),
    /// Floating-number of 64 bits
    Float64(f64),
}

/// A function parameter.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Parameter {
    /// The identifier of this parameter. May not be present, in
    /// which case the local must be accessed through its
    /// index.
    pub identifier: Option<SmallString>,
    /// The type of this parameter
    pub type_: Type,
}

/// A local variable within a function.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Local {
    /// The identifier of this parameter. May not be present, in
    /// which case the local must be accessed through its
    /// index.
    pub identifier: Option<SmallString>,
    /// The type of this parameter
    pub type_: Type,
}

/// Represents a function definition.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Function {
    /// The identifier for this function, if any.
    pub identifier: Option<SmallString>,
    /// The parameters of this function.
    /// Ordered according to the order the
    /// parameters were defined.
    pub parameters: Vec<Parameter>,
    /// The local variables of this function.
    /// Ordered according to the order the
    /// locals were defined.
    pub local_variables: Vec<Local>,
}

/// A single instruction that can be located inside a function
/// body
#[derive(Clone, Debug, PartialEq)]
pub enum Instruction {
    Call(Index),
    /// Fetch or set a local or global variable
    VariableInstruction {
        /// Wether this instruction is in `local.` or `global.`
        scope: ScopeKind,
        /// Defines if we're getting/setting/teeing the variable
        instruction: VariableInstruction,
    },
    /// Pushes a numerical constant to the stack.
    ///
    /// E.g. `i32.const 5`, `f64.const 2.5`
    Constant {
        /// Represents both the type of the constant
        /// and the constant itself
        value: NumericalValue,
    },
    /// An arithmetic operation
    Arithmetic {
        type_: NumericalType,
        instr: ArithmeticInstruction,
    },
    Comparison {
        type_: NumericalType,
        instr: ComparisonInstruction,
    },
}

/// An index for an instruction, may be an identifier or a
/// numerical index.
///
/// # Examples
///
/// * `call $function` (function is an identifier in an
/// indexing position)
/// * ` local.get 0` (0 is a numerical index)
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Index {
    Identifier(SmallString),
    Numerical(i64),
}

/// Wether a given instruction is in `local.` or `global.`
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ScopeKind {
    Global,
    Local,
}

/// Represents an instruction for direct variable access.
#[derive(Debug, Clone, PartialEq)]
pub enum VariableInstruction {
    /// Get the value of an identifier by its index or
    /// identifier.
    ///
    /// E.g. `get $number`
    Get(Index),
    /// Set the value of a variable.
    ///
    /// E.g. `(local.set $var (i32.const 10)) ;; set $var to 10`
    Set {
        // TODO: check what kind of inline values might be used
        //       in `set`.
        // TOOD: handle extra parenthesis in `{scope}.set`
        //       if there is an "inlined" value
        index: Index,
        /// Currently always None
        value: Option<NumericalValue>,
    },
    /// Like `local.set` but also returns its argument.
    /// Does not exist for `global`.
    Tee(Index),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ArithmeticInstruction {
    /// i32.add, i64.add, f32.add, or f64.add
    Addition,
    /// i32.sub, i64.sub, f32.sub, or f64.sub
    Subtraction,
    /// i32.mul, i64.mul, f32.mul, or f64.mul
    Multiplication,
    /// f32.div, or f64.div
    FloatDivision,
    /// i32.div_s, i64.div_s
    SignedDivision,
    /// i32.div_u, i64.div_u
    UnsignedDisivion,
    /// i32.rem_s or i64.rem_s
    SignedRemainder,
    /// i32.rem_u or i64.rem_u
    UnsignedRemainder,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]

pub enum ComparisonInstruction {
    Equal,
    NotEqual,
    GreaterThan,
    LessThan,
    GreaterOrEqual,
    LessOrEqual,
}
