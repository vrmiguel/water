//! A parser for WebAssembly Text Format.
//!
//! Functions are mostly all public as to allow doc-tests.

mod function;
mod instruction;
mod module;
mod utils;

pub use function::*;
pub use instruction::*;
pub use module::*;
use nom::error::VerboseError;
pub use utils::*;

/// The result of a parsing operation with added error context
pub type IResult<'a, T> =
    nom::IResult<&'a str, T, VerboseError<&'a str>>;
