use std::io::{self, Write};

pub trait Emittable {
    /// Emit `self` to WebAssembly.
    fn emit_to<W: Write>(
        &self,
        // TODO: change to receive &mut Emitter?
        writer: &mut W,
    ) -> io::Result<usize>;
}
