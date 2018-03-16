/// Helper macro for working with `io::Result` in a context where
/// `ErrorKind::Interrupted` means to retry an action. This macro corresponds to
/// `std::try`, but it will reevaluate the expression until it does not evaluate
/// to an `Err` of kind `Interrupted`.
#[macro_export]
macro_rules! retry {
    ($e:expr) => (
        loop {
            match $e {
                Ok(t) => break t,
                Err(ref e) if e.kind() == ::std::io::ErrorKind::Interrupted => {}
                Err(e) => return Err(e.into()),
            }
        }
    )
}

/// A variant of try_ready! that checks whether the expression evaluates to 0, and emits a
/// `futures_io::Error` of kind `UnexpectedEof` with the given message if so.
#[macro_export]
macro_rules! read_nz {
    ($e:expr, $msg:expr) => (
        match $e {
            Ok(::futures_core::Async::Ready(0)) => return Err(::futures_io::Error::new(::futures_io::ErrorKind::UnexpectedEof, $msg).into()),
            Ok(::futures_core::Async::Ready(read)) => read,
            Ok(::futures_core::Async::Pending) => return Ok(::futures_core::Async::Pending),
            Err(e) => return Err(From::from(e)),
        }
    )
}

/// A variant of try_ready! that checks whether the expression evaluates to 0, and emits a
/// `futures_io::Error` of kind `WriteZero` with the given message if so.
#[macro_export]
macro_rules! write_nz {
    ($e:expr, $msg:expr) => (
        match $e {
            Ok(::futures_core::Async::Ready(0)) => return Err(::futures_io::Error::new(::futures_io::ErrorKind::WriteZero, $msg).into()),
            Ok(::futures_core::Async::Ready(written)) => written,
            Ok(::futures_core::Async::Pending) => return Ok(::futures_core::Async::Pending),
            Err(e) => return Err(From::from(e)),
        }
    )
}
