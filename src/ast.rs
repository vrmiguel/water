//! The abstract syntax tree `waster` parses to.

use crate::small_string::SmallString;

pub struct Program {
    pub modules: Vec<Module>,
}

/// Represents a WebAssembly Text Format module
pub struct Module {
    // TODO
}

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
    /// The identifiers this function will be exported to, if
    /// any.
    pub exports: Vec<SmallString>,
    /// The parameters of this function.
    /// Ordered according to the order the
    /// parameters were defined.
    pub parameters: Vec<Parameter>,
    /// The local variables of this function.
    /// Ordered according to the order the
    /// locals were defined.
    pub local_variables: Vec<Local>,
}

#[derive(Clone, Debug, PartialEq)]
/// Represents an instruction along the possible "inlined"
/// arguments it may have.
pub struct Instruction {
    /// The actual operation this instruction represents
    pub opcode: Opcode,
    /// The list of "inlined" arguments to this instruction, if
    /// any.
    // TODO: transform this into a "generic" Value
    // TODO: investigate use of SmallVec here
    pub arguments: Vec<Instruction>,
}

/// Represents an `import` statement for functions.
///
/// Consists of the namespace from which we're importing from,
/// the name of the imported and the WAT function signature the
/// imported function will be attached to.
///
/// E.g.:
///
/// ```not-rust
///               function name
///                    ↓↓↓  
/// (import "console" "log" (func $log (param i32 i32)))
///          ↑↑↑↑↑↑↑         ↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑
///         namespace           WAT function signature
/// ```
pub struct ImportFunction {
    pub namespace: SmallString,
    pub fn_name: SmallString,
    pub signature: Function,
}

/// A single instruction that can be located inside a function
/// body
#[derive(Clone, Debug, PartialEq)]
pub enum Opcode {
    /// Calls a function
    Call(Index),
    /// Fetch or set a local or global variable
    VariableInstruction(VariableOperation),
    /// Pushes a numerical constant to the stack.
    ///
    /// E.g. `i32.const 5`, `f64.const 2.5`
    Constant(Constant),
    /// An arithmetic operation
    Arithmetic(ArithmeticOperation),
    Comparison(ComparisonOperation),
    /// Denotes a point in code that should not be reachable.
    /// `unreachable` is an unconditional trap: in the case
    /// where an unreachable is reached and executed, the
    /// instruction traps.
    ///
    /// Note: unreachable accepts any arity.
    ///
    /// ```not-rust
    /// (i32.const 6)
    /// (unreachable (i32.const 5) (i32.const 5))
    /// ```
    Unreachable(Unreachable),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct VariableOperation {
    /// Whether this instruction is in `local.` or `global.`
    pub scope: ScopeKind,
    /// Defines if we're getting/setting/teeing the variable
    pub instruction: VariableInstruction,
    /// Accesses the variable either through its definition
    /// index or by its identifier
    pub index: Index,
}

/// Pushes a numerical constant to the stack.
///
/// E.g. `i32.const 5`, `f64.const 2.5`
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Constant {
    /// Represents both the type of the constant
    /// and the constant itself
    pub value: NumericalValue,
}

/// An arithmetic operation
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ArithmeticOperation {
    /// The related type of this operation (i32, i64, f32 or
    /// f64)
    pub type_: NumericalType,
    /// The arithmetic instruction of this operation (such as
    /// addition, subtraction, etc)
    pub instr: ArithmeticInstruction,
}

#[derive(Clone, Debug, PartialEq, Eq)]
/// A comparison operation
pub struct ComparisonOperation {
    /// The related type of this operation (i32, i64, f32 or
    /// f64)
    pub type_: NumericalType,
    /// The arithmetic instruction of this operation (such as
    /// equal, not equal, greater than, etc)
    pub instr: ComparisonInstruction,
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

/// Whether a given instruction is in `local.` or `global.`
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ScopeKind {
    Global,
    Local,
}

/// Represents an instruction for direct variable access.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum VariableInstruction {
    /// Get the value of an identifier by its index or
    /// identifier.
    ///
    /// E.g. `get $number`
    Get,
    /// Set the value of a variable.
    ///
    /// E.g. `(local.set $var (i32.const 10)) ;; set $var to 10`
    Set,
    /// Like `local.set` but also returns its argument.
    /// Does not exist for `global`.
    Tee,
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

/// Zero-sized type to denote the `unreachable` instruction,
/// which denotes a point in code that should not be reachable.
/// `unreachable` is an unconditional trap: in the case
/// where an unreachable is reached and executed, the
/// instruction traps.
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub struct Unreachable;
