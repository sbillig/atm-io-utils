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

/// This macro is to `retry` what `tokio_io::try_nb` is to `std::try`. It works
/// like `try_nb`, but it will reevaluate the expression until it does not
/// evaluate to an `Err` of kind `Interrupted`.
#[macro_export]
macro_rules! retry_nb {
    ($e:expr) => (
        loop {
            match $e {
                Ok(t) => break t,
                Err(ref e) if e.kind() == ::std::io::ErrorKind::WouldBlock => {
                    return Ok(::futures::Async::NotReady)
                }
                Err(ref e) if e.kind() == ::std::io::ErrorKind::Interrupted => {}
                Err(e) => return Err(e.into()),
            }
        }
    )
}
