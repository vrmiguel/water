//! The abstract syntax tree `waster` parses to.

use crate::small_string::SmallString;

/// Represents a WebAssembly Text Format module
pub struct Module {}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum WasmType {
    Int32,
    Int64,
    Float32,
    Float64,
}

/// A function parameter.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Parameter {
    /// The identifier of this parameter. May not be present, in
    /// which case the local must be accessed through its
    /// index.
    pub identifier: Option<SmallString>,
    /// The type of this parameter
    pub type_: WasmType,
}

/// A local variable within a function.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Local {
    /// The identifier of this parameter. May not be present, in
    /// which case the local must be accessed through its
    /// index.
    pub identifier: Option<SmallString>,
    /// The type of this parameter
    pub type_: WasmType,
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
pub enum Instruction {
    // TODO: allow indexes in `call`
    Call(SmallString),
    /// Fetch or set a local or global variable
    VariableInstruction {
        /// Wether this instruction is in `local.` or `global.`
        scope: ScopeKind,

        instruction: VariableInstruction,
        index: Index,
    },
    /// An operation regarding an integer
    Integer(),
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
