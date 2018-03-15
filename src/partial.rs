//! Helpers to test partial and `Pending` io operations.
//!
//! Inspired by (and bluntly stealing from) the [partial-io](https://crates.io/crates/partial-io) crate.

use std::cmp::min;

use futures_core::Poll;
use futures_core::Async::Pending;
use futures_core::task::Context;
use futures_io::{AsyncRead, AsyncWrite, Error, IoVec};

/// The different operations supported by the partial wrappers.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum PartialOp {
    /// Perform the io operation as normal.
    Unlimited,
    /// Perform the io operation, but limit it to a maximum number of bytes.
    Limited(usize),
    /// Emit `Ok(Async::Pending)` and reschedule the task.
    Pending,
}

/// Wraps a reader and modifies its read operations according to the given iterator of `PartialOp`s.
#[derive(Debug)]
pub struct PartialRead<R, Ops> {
    reader: R,
    ops: Ops,
}

impl<R, Ops> PartialRead<R, Ops> {
    /// Create a new `PartialRead`, wrapping the given `R` and modifying its io operations via the
    /// given `Ops`.
    pub fn new(reader: R, ops: Ops) -> PartialRead<R, Ops> {
        PartialRead { reader, ops }
    }

    /// Gets a reference to the underlying `R`.
    pub fn get_ref(&self) -> &R {
        &self.reader
    }

    /// Gets a mutable reference to the underlying `R`.
    pub fn get_mut(&mut self) -> &mut R {
        &mut self.reader
    }

    /// Consumes this `PartialRead`, returning the underlying reader.
    pub fn into_inner(self) -> R {
        self.reader
    }
}

impl<R, Ops> AsyncRead for PartialRead<R, Ops>
    where R: AsyncRead,
          Ops: Iterator<Item = PartialOp>
{
    fn poll_read(&mut self, cx: &mut Context, buf: &mut [u8]) -> Poll<usize, Error> {
        match self.ops.next() {
            None |
            Some(PartialOp::Unlimited) => self.reader.poll_read(cx, buf),
            Some(PartialOp::Pending) => {
                cx.waker().wake();
                Ok(Pending)
            }
            Some(PartialOp::Limited(n)) => {
                let len = min(n, buf.len());
                self.reader.poll_read(cx, &mut buf[..len])
            }
        }
    }
}

impl<W, Ops> AsyncWrite for PartialRead<W, Ops>
    where W: AsyncWrite
{
    fn poll_write(&mut self, cx: &mut Context, buf: &[u8]) -> Poll<usize, Error> {
        self.reader.poll_write(cx, buf)
    }

    fn poll_flush(&mut self, cx: &mut Context) -> Poll<(), Error> {
        self.reader.poll_flush(cx)
    }

    fn poll_close(&mut self, cx: &mut Context) -> Poll<(), Error> {
        self.reader.poll_close(cx)
    }

    fn poll_vectored_write(&mut self, cx: &mut Context, vec: &[&IoVec]) -> Poll<usize, Error> {
        self.reader.poll_vectored_write(cx, vec)
    }
}

/// Wraps a reader and modifies its read operations according to the given iterator of `PartialOp`s.
#[derive(Debug)]
pub struct PartialWrite<W, Ops> {
    writer: W,
    ops: Ops,
}

impl<W, Ops> PartialWrite<W, Ops> {
    /// Create a new `PartialWrite`, wrapping the given `W` and modifying its io operations via the
    /// given `Ops`.
    pub fn new(writer: W, ops: Ops) -> PartialWrite<W, Ops> {
        PartialWrite { writer, ops }
    }

    /// Gets a reference to the underlying `W`.
    pub fn get_ref(&self) -> &W {
        &self.writer
    }

    /// Gets a mutable reference to the underlying `W`.
    pub fn get_mut(&mut self) -> &mut W {
        &mut self.writer
    }

    /// Consumes this `PartialWrite`, returning the underlying writer.
    pub fn into_inner(self) -> W {
        self.writer
    }
}

impl<W, Ops> AsyncWrite for PartialWrite<W, Ops>
    where W: AsyncWrite,
          Ops: Iterator<Item = PartialOp>
{
    fn poll_write(&mut self, cx: &mut Context, buf: &[u8]) -> Poll<usize, Error> {
        match self.ops.next() {
            None |
            Some(PartialOp::Unlimited) => self.writer.poll_write(cx, buf),
            Some(PartialOp::Pending) => {
                cx.waker().wake();
                Ok(Pending)
            }
            Some(PartialOp::Limited(n)) => {
                let len = min(n, buf.len());
                self.writer.poll_write(cx, &buf[..len])
            }
        }
    }

    fn poll_flush(&mut self, cx: &mut Context) -> Poll<(), Error> {
        match self.ops.next() {
            Some(PartialOp::Pending) => {
                cx.waker().wake();
                Ok(Pending)
            }
            _ => self.writer.poll_flush(cx),
        }
    }

    fn poll_close(&mut self, cx: &mut Context) -> Poll<(), Error> {
        match self.ops.next() {
            Some(PartialOp::Pending) => {
                cx.waker().wake();
                Ok(Pending)
            }
            _ => self.writer.poll_close(cx),
        }
    }
}

impl<W, Ops> AsyncRead for PartialWrite<W, Ops>
    where W: AsyncRead
{
    fn poll_read(&mut self, cx: &mut Context, buf: &mut [u8]) -> Poll<usize, Error> {
        self.writer.poll_read(cx, buf)
    }
}

#[cfg(feature = "quickcheck")]
mod qs {
    use super::*;

    use quickcheck::{Arbitrary, Gen, empty_shrinker};

    impl Arbitrary for PartialOp {
        fn arbitrary<G: Gen>(g: &mut G) -> Self {
            let rnd = g.next_f32();
            if rnd < 0.2 {
                PartialOp::Pending
            } else if rnd < 0.4 {
                PartialOp::Unlimited
            } else {
                if g.size() <= 1 {
                    PartialOp::Limited(1)
                } else {
                    let max = g.size();
                    PartialOp::Limited(g.gen_range(1, max))
                }
            }
        }

        fn shrink(&self) -> Box<Iterator<Item = Self>> {
            match *self {
                PartialOp::Limited(n) => {
                    Box::new(n.shrink().filter(|k| k != &0).map(PartialOp::Limited))
                }
                _ => empty_shrinker(),
            }
        }
    }
}

#[cfg(feature = "quickcheck")]
pub use self::qs::*;
