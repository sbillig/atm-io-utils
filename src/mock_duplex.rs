use std::collections::VecDeque;
use std::cmp::min;
use std::io::{Write, Read, Error};

use tokio_io::{AsyncRead, AsyncWrite};
use futures::{Poll, Async};

/// A duplex which pulls all read data from a queue and puts all written data
/// into a queue.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MockDuplex {
    reads: VecDeque<u8>,
    writes: VecDeque<u8>,
}

impl MockDuplex {
    /// Create a new, empty `MockDuplex`.
    pub fn new() -> MockDuplex {
        MockDuplex {
            reads: VecDeque::new(),
            writes: VecDeque::new(),
        }
    }

    /// Add data to the fifo queue from which `read` takes data.
    pub fn add_read_data(&mut self, bytes: &[u8]) {
        for byte in bytes {
            self.reads.push_back(*byte);
        }
    }

    /// Pulls as many bytes as possible from the fifo queue into which `write`
    /// places data, and puts them into the supplied `buf`. Returns how many
    /// bytes were drained.
    pub fn drain_write_data(&mut self, buf: &mut [u8]) -> usize {
        let mut i = 0;

        for byte in self.writes.drain(0..buf.len()) {
            buf[i] = byte;
            i += 1;
        }

        return i;
    }

    /// Consumes this `MockDuplex`, returning the remaining read data and write
    /// data.
    pub fn into_inner(self) -> (VecDeque<u8>, VecDeque<u8>) {
        (self.reads, self.writes)
    }
}

impl Read for MockDuplex {
    /// Takes data which was previously added via `add_read_data` and fills the
    /// given buffer with it.
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, Error> {
        let mut i = 0;

        for byte in self.reads.drain(0..buf.len()) {
            buf[i] = byte;
            i += 1;
        }

        return Ok(i);
    }
}

impl AsyncRead for MockDuplex {}

impl Write for MockDuplex {
    /// Puts data into a fifo queue which can be consumed via
    /// `drain_write_data`.
    fn write(&mut self, buf: &[u8]) -> Result<usize, Error> {
        for byte in buf {
            self.writes.push_back(*byte);
        }

        return Ok(buf.len());
    }

    fn flush(&mut self) -> Result<(), Error> {
        Ok(())
    }
}

impl AsyncWrite for MockDuplex {
    fn shutdown(&mut self) -> Poll<(), Error> {
        Ok(Async::Ready(()))
    }
}
