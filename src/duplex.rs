use futures_core::Poll;
use futures_core::task::Context;
use futures_io::{AsyncRead, AsyncWrite, Error};

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

impl<R: AsyncRead, W> AsyncRead for Duplex<R, W> {
    fn poll_read(&mut self, cx: &mut Context, buf: &mut [u8]) -> Poll<usize, Error> {
        self.r.poll_read(cx, buf)
    }
}

impl<R, W: AsyncWrite> AsyncWrite for Duplex<R, W> {
    fn poll_write(&mut self, cx: &mut Context, buf: &[u8]) -> Poll<usize, Error> {
        self.w.poll_write(cx, buf)
    }

    fn poll_flush(&mut self, cx: &mut Context) -> Poll<(), Error> {
        self.w.poll_flush(cx)
    }

    fn poll_close(&mut self, cx: &mut Context) -> Poll<(), Error> {
        self.w.poll_close(cx)
    }
}
