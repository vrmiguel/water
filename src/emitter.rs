use std::io::{self, Write};

const MAGIC: &[u8] = b"\0asm";
const VERSION: &[u8] = b"1000";

pub struct Emitter<W: Write> {
    /// Where this Emitter will write to
    writer: W,
}

impl<W: Write> Emitter<W> {
    fn emit_asserted(&mut self, bytes: &[u8]) -> io::Result<()> {
        let bytes_written = self.writer.write(bytes)?;
        assert_eq!(
            bytes_written,
            bytes.len(),
            "failed to write the entire buffer to the writer"
        );

        Ok(())
    }

    fn emit_magic(&mut self) -> io::Result<()> {
        self.emit_asserted(MAGIC)
    }

    fn emit_version(&mut self) -> io::Result<()> {
        self.emit_asserted(VERSION)
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
