use std::io::{self, Write};

use crate::{ast::NumericalValue, leb128::SignedLeb128};

pub trait Emittable {
    /// Emit `self` to WebAssembly.
    fn emit_to<W: Write>(
        &self,
        writer: &mut W,
    ) -> io::Result<usize>;
}

impl Emittable for NumericalValue {
    fn emit_to<W: Write>(
        &self,
        writer: &mut W,
    ) -> io::Result<usize> {
        let value = match self {
            NumericalValue::Int32(int32) => *int32 as i64,
            NumericalValue::Int64(int64) => *int64,
            NumericalValue::Float32(_) => todo!(),
            NumericalValue::Float64(_) => todo!(),
        };

        SignedLeb128::from(value).emit_to(writer)
    }
}
