use std::io::{self, Write};

use super::{Emittable, Emitter};
use crate::{ast::NumericalValue, leb128::SignedLeb128};

impl<W: Write> Emittable<NumericalValue> for Emitter<W> {
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
    #[inline(always)]
    pub fn f32_to_bytes(n: f32) -> [u8; 4] {
        n.to_le_bytes()
    }

    #[inline(always)]
    pub fn f64_to_bytes(n: f64) -> [u8; 8] {
        n.to_le_bytes()
    }
}
