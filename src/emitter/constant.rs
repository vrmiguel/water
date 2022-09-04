use std::io::{self, Write};

use super::Emittable;
use crate::{ast::Constant, opcode::ToOpcode};

impl Emittable for Constant {
    fn emit_to<W: Write>(
        &self,
        writer: &mut W,
    ) -> io::Result<usize> {
        let opcode = self.value.to_opcode();

        // Emit the `const` opcode for the given value
        writer.write_all(&[opcode])?;
        // .. and then the actual literal
        self.value.emit_to(writer)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        ast::{Constant, NumericalValue},
        emitter::Emittable,
    };

    #[test]
    fn emits_i32_const_correctly() {
        let mut buf = [0_u8; 3];

        let constant = Constant {
            value: NumericalValue::Int32(128),
        };

        constant.emit_to(&mut buf.as_mut_slice()).unwrap();
        assert_eq!(
            &buf,
            &[
                // `i32.const`'s opcode
                0x41, // and the LEB128 for 128
                128, 1
            ]
        );
    }

    #[test]
    fn emits_i64_const_correctly() {
        let mut buf = [0_u8; 3];

        let constant = Constant {
            value: NumericalValue::Int64(505),
        };

        constant.emit_to(&mut buf.as_mut_slice()).unwrap();
        assert_eq!(
            &buf,
            &[
                // `i64.const`'s opcode
                0x42, // and the LEB128 for 128
                249, 3
            ]
        );
    }

    #[test]
    fn emits_f32_const_correctly() {
        let mut buf = [0_u8; 5];

        let constant = Constant {
            value: NumericalValue::Float32(5.0),
        };

        constant.emit_to(&mut buf.as_mut_slice()).unwrap();
        assert_eq!(
            &buf,
            &[
                // `f32.const`'s opcode
                0x43,
                // and then the LE bit pattern for 5.0
                0x00, 0x00, 0xa0, 0x40,
            ]
        );
    }

    #[test]
    fn emits_f64_const_correctly() {
        let mut buf = [0_u8; 9];

        let constant = Constant {
            value: NumericalValue::Float64(25.50),
        };

        constant.emit_to(&mut buf.as_mut_slice()).unwrap();
        assert_eq!(
            &buf,
            &[
                // `f64.const`'s opcode
                0x44,
                // and then the LE bit pattern for 25.50
                0x00, 0x00, 0x00, 0x00, 0x00, 0x80, 0x39, 0x40,
            ]
        );
    }
}
