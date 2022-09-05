use std::io::{self, Write};

use super::{Emittable, Emitter};
use crate::{ast::Unreachable, opcode::ToOpcode};

impl<W: Write> Emittable<Unreachable> for Emitter<W> {
    /// Does not type-check or check arity since unreachable
    /// doesn't really check for anything.
    fn emit_element(
        &mut self,
        unreachable: Unreachable,
    ) -> io::Result<usize> {
        let opcode = unreachable.to_opcode();

        self.emit_byte(opcode)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        ast::Unreachable,
        emitter::{Emittable, Emitter},
    };

    #[test]
    fn emits_unreachable_opcode() {
        let mut emitter = Emitter::new_cursored([0_u8; 1]);

        emitter.emit_element(Unreachable).unwrap();

        assert_eq!(&emitter.into_inner().into_inner(), &[0x00])
    }
}
