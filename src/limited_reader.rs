//! A wrapper around a reader that limits how many bytes can be read from it.

use std::cmp::min;

use futures_core::Poll;
use futures_core::task::Context;
use futures_io::{AsyncRead, Error as FutIoErr};

/// Wraps a reader and limits the number of bytes that can be read from it. Once the limit has been
/// reached, further calls to poll_read will return `Ok(Ready(0))`.
pub struct LimitedReader<R> {
    inner: R,
    remaining: usize,
}

impl<R> LimitedReader<R> {
    /// Create a new `LimitedReader`, wrapping the given reader.
    pub fn new(inner: R, limit: usize) -> LimitedReader<R> {
        LimitedReader {
            inner: inner,
            remaining: limit,
        }
    }
}

impl<R: AsyncRead> AsyncRead for LimitedReader<R> {
    fn poll_read(&mut self, cx: &mut Context, buf: &mut [u8]) -> Poll<usize, FutIoErr> {
        let upper = min(self.remaining, buf.len());
        self.inner.poll_read(cx, &mut buf[..upper])
    }
}
