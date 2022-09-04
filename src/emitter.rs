use std::io::{self, Write};

mod constant;
pub mod emittable;
mod numerical_value;

pub use emittable::Emittable;

use crate::ast::Program;

const MAGIC: &[u8] = b"\0asm";
const VERSION: &[u8] = b"1000";

pub struct Emitter<W: Write> {
    /// Where this Emitter will write to
    writer: W,
}

impl<W: Write> Emitter<W> {
    #[cfg(test)]
    fn into_inner(self) -> W {
        self.writer
    }

    /// Emit a single byte to the writer
    pub fn emit_byte(&mut self, byte: u8) -> io::Result<()> {
        self.emit_bytes(&[byte])
    }

    /// Emit a sequence of bytes to the writer
    pub fn emit_bytes(
        &mut self,
        bytes: &[u8],
    ) -> io::Result<()> {
        self.writer.write_all(bytes)
    }

    /// Emits the WASM magic constant
    fn emit_magic(&mut self) -> io::Result<()> {
        self.emit_bytes(MAGIC)
    }

    /// Emits the WASM version tag
    fn emit_version(&mut self) -> io::Result<()> {
        self.emit_bytes(VERSION)
    }

    /// Builds a new emitter with the given writer
    pub fn new(writer: W) -> Self {
        Self { writer }
    }

    /// Emit the given program to WASM
    pub fn emit_program(
        &mut self,
        _program: Program,
    ) -> io::Result<()> {
        self.emit_magic()?;
        self.emit_version()?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::MAGIC;

    #[test]
    fn assert_correct_magic() {
        assert_eq!(MAGIC, &[0x00, 0x61, 0x73, 0x6d])
    }
}
