//! Utilities for working with `std::io` and `futures_io`.
#![deny(missing_docs)]

extern crate futures_core;
extern crate futures_io;
#[cfg(feature = "quickcheck")]
extern crate quickcheck;

mod duplex;
mod macros;
pub mod partial;

pub use duplex::*;
pub use macros::*;
