use std::io::{self, Write};

pub trait Emittable {
    /// Emit `self` to WebAssembly.
    fn emit_to<W: Write>(
        &self,
        // TODO: change to receive &mut Emitter?
        writer: &mut W,
    ) -> io::Result<usize>;
}

pub trait Emittable2<T> {
    /// Emit `element` to WebAssembly.
    fn emit_element(
        &mut self,
        // TODO: change to receive &mut Emitter?
        element: T,
    ) -> io::Result<usize>;
}
