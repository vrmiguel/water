use std::io::{self, Write};

use crate::{ast::NumericalValue, leb128::SignedLeb128};

pub trait Emittable {
    /// Emit `self` to WebAssembly.
    fn emit_to<W: Write>(
        &self,
        // TODO: change to receive Emitter?
        writer: &mut W,
    ) -> io::Result<usize>;
}

impl Emittable for NumericalValue {
    fn emit_to<W: Write>(
        &self,
        writer: &mut W,
    ) -> io::Result<usize> {
        use floating_point_emitters::*;

        match self {
            NumericalValue::Int32(int32) => {
                SignedLeb128::from(*int32 as i64).emit_to(writer)
            }
            NumericalValue::Int64(int64) => {
                SignedLeb128::from(*int64).emit_to(writer)
            }
            NumericalValue::Float32(f32) => emit_f32(*f32, writer),
            NumericalValue::Float64(f64) => emit_f64(*f64, writer),
        }
    }
}

/// According to the [WebAssembly spec](https://webassembly.github.io/spec/core/binary/values.html),
/// floating-point values are encoded by their IEEE 754-2019 (Section 3.4) bit pattern in little endian byte order.
/// 
/// What we'll do here is emit our Rust floating-point numbers (defined as binary64 in IEEE 754-2008)
/// through `{floating}::to_le_bytes`. I assume that this is enough to fit the spec but I'm not knowledgeable
/// enough about IEEE 754 to be sure.
mod floating_point_emitters {
    use std::io::{self, Write};

    pub fn emit_f32<W: Write>(n: f32, writer: &mut W) -> io::Result<usize> {
        writer.write_all(&n.to_le_bytes()).map(|_| 4)
    }
    
    pub fn emit_f64<W: Write>(n: f64, writer: &mut W) -> io::Result<usize> {
        writer.write_all(&n.to_le_bytes()).map(|_| 8)
    }
}

