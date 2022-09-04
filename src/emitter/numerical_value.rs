use std::io::{self, Write};

use super::{emittable::Emittable2, Emittable, Emitter};
use crate::{ast::NumericalValue, leb128::SignedLeb128};

impl Emittable for NumericalValue {
    fn emit_to<W: Write>(
        &self,
        writer: &mut W,
    ) -> io::Result<usize> {
        use floating_point_emitters::*;

        match *self {
            NumericalValue::Int32(int32) => {
                SignedLeb128::from(int32 as i64).emit_to(writer)
            }
            NumericalValue::Int64(int64) => {
                SignedLeb128::from(int64).emit_to(writer)
            }
            NumericalValue::Float32(f32) => {
                emit_f32(f32, writer)
            }
            NumericalValue::Float64(f64) => {
                emit_f64(f64, writer)
            }
        }
    }
}

impl<W: Write> Emittable2<NumericalValue> for Emitter<W> {
    fn emit_element(
        &mut self,
        element: NumericalValue,
    ) -> io::Result<usize> {
        use floating_point_converters::*;

        match element {
            NumericalValue::Int32(int32) => self
                .emit_element(SignedLeb128::from(int32 as i64)),
            NumericalValue::Int64(int64) => {
                self.emit_element(SignedLeb128::from(int64))
            }
            NumericalValue::Float32(f32) => {
                let bytes = f32_to_bytes(f32);

                self.emit_bytes(&bytes).map(|()| 4)
            }
            NumericalValue::Float64(f64) => {
                let bytes = f64_to_bytes(f64);

                self.emit_bytes(&bytes).map(|()| 8)
            }
        }
    }
}

/// According to the [WebAssembly spec](https://webassembly.github.io/spec/core/binary/values.html),
/// floating-point values are encoded by their IEEE 754-2019
/// (Section 3.4) bit pattern in little endian byte order.
///
/// What we'll do here is emit our Rust floating-point numbers
/// (defined as binary64 in IEEE 754-2008)
/// through `{floating}::to_le_bytes`. I assume that this is
/// enough to fit the spec but I'm not knowledgeable enough about
/// IEEE 754 to be sure.
mod floating_point_converters {
    pub fn f32_to_bytes(n: f32) -> [u8; 4] {
        n.to_le_bytes()
    }

    pub fn f64_to_bytes(n: f64) -> [u8; 8] {
        n.to_le_bytes()
    }
}

/// According to the [WebAssembly spec](https://webassembly.github.io/spec/core/binary/values.html),
/// floating-point values are encoded by their IEEE 754-2019
/// (Section 3.4) bit pattern in little endian byte order.
///
/// What we'll do here is emit our Rust floating-point numbers
/// (defined as binary64 in IEEE 754-2008)
/// through `{floating}::to_le_bytes`. I assume that this is
/// enough to fit the spec but I'm not knowledgeable enough about
/// IEEE 754 to be sure.
mod floating_point_emitters {
    use std::io::{self, Write};

    pub fn emit_f32<W: Write>(
        n: f32,
        writer: &mut W,
    ) -> io::Result<usize> {
        writer.write_all(&n.to_le_bytes()).map(|()| 4)
    }

    pub fn emit_f64<W: Write>(
        n: f64,
        writer: &mut W,
    ) -> io::Result<usize> {
        writer.write_all(&n.to_le_bytes()).map(|()| 8)
    }
}
