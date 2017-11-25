use std::io::{Write, Read, Error};

use tokio_io::{AsyncRead, AsyncWrite};
use futures::Poll;

/// Implements both (Async)Read and (Async)Write by delegating to an (Async)Read
/// and an (Async)Write, taking ownership of both.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Duplex<R, W> {
    r: R,
    w: W,
}

impl<R, W> Duplex<R, W> {
    /// Takes ownership of a Read and a Write and creates a new Duplex.
    pub fn new(r: R, w: W) -> Duplex<R, W> {
        Duplex { r, w }
    }

    /// Gets a reference to the underlying reader.
    pub fn get_reader_ref(&self) -> &R {
        &self.r
    }

    /// Gets a mutable reference to the underlying reader.
    pub fn get_reader_mut(&mut self) -> &mut R {
        &mut self.r
    }

    /// Gets a reference to the underlying writer.
    pub fn get_writer_ref(&self) -> &W {
        &self.w
    }

    /// Gets a mutable reference to the underlying writer.
    pub fn get_writer_mut(&mut self) -> &mut W {
        &mut self.w
    }

    /// Unwraps this `Duplex`, returning the underlying reader and writer.
    pub fn into_inner(self) -> (R, W) {
        (self.r, self.w)
    }
}

impl<R: Read, W> Read for Duplex<R, W> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, Error> {
        self.r.read(buf)
    }
}

impl<R: AsyncRead, W> AsyncRead for Duplex<R, W> {}

impl<R, W: Write> Write for Duplex<R, W> {
    fn write(&mut self, buf: &[u8]) -> Result<usize, Error> {
        self.w.write(buf)
    }

    fn flush(&mut self) -> Result<(), Error> {
        self.w.flush()
    }
}

impl<R, W: AsyncWrite> AsyncWrite for Duplex<R, W> {
    fn shutdown(&mut self) -> Poll<(), Error> {
        self.w.shutdown()
    }
}
