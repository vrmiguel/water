use std::io::{self, Write};

mod constant;
pub mod emittable;
mod numerical_value;

pub use emittable::Emittable;

const MAGIC: &[u8] = b"\0asm";
const VERSION: &[u8] = b"1000";

pub struct Emitter<W: Write> {
    /// Where this Emitter will write to
    writer: W,
}

impl<W: Write> Emitter<W> {
    fn emit(&mut self, bytes: &[u8]) -> io::Result<()> {
        self.writer.write_all(bytes)
    }

    fn emit_magic(&mut self) -> io::Result<()> {
        self.emit(MAGIC)
    }

    fn emit_version(&mut self) -> io::Result<()> {
        self.emit(VERSION)
    }
    
    pub fn new(writer: W) -> Self {
        Self { writer }
    }

    pub fn emit_program(
        &mut self,
        _program: (),
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
