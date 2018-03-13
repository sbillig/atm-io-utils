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
