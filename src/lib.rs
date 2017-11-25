//! Utilities for working with `std::io` and `tokio_io`.
#![deny(missing_docs)]

extern crate tokio_io;
extern crate futures;

mod macros;
mod duplex;
mod mock_duplex;

pub use macros::*;
pub use duplex::*;
pub use mock_duplex;
