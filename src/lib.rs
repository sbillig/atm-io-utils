//! Utilities for working with `std::io` and `futures_io`.
#![deny(missing_docs)]

extern crate futures_core;
extern crate futures_io;

mod duplex;
mod macros;

pub use duplex::*;
pub use macros::*;
