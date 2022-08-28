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
