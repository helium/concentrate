/// A common result type for this crate.
pub type Result<T = ()> = ::std::result::Result<T, Error>;

quick_error! {
    /// A common error type for this crate.
    #[derive(Debug)]
    pub enum Error {
        /// Device is currently opened in same process.
        Busy {
            description("concentrator device is already in use")
        }
        /// Catch-all error returned by the low-level `libloragw` c
        /// code.
        HAL {
            description("concentrator HAL returned a generic error")
        }
        /// A buffer, primarily transmit payloads, is too large for
        /// the LoRa packet format.
        Size {
            description("provided buffer is too large")
        }
        /// Represents and error when attempting to convert between
        /// this crate's high-level types and those defined in
        /// `libloragw`.
        Data {
            description("failure to convert hardware val to symbolic val")
        }
        /// Invalid input.
        Other (err: String) {
            description(err)
        }
    }
}

/// Wraps a `libloragw-sys` function call and:
/// - wraps the return code in a `Result`
/// - logs name of FFI function on error
macro_rules! hal_call{
    ( $fn:ident ( $($arg:expr),* ) ) => {
        match $crate::libloragw_sys::$fn ( $($arg),* ) {
            -1 => {
                error!("HAL call {} returned an error", stringify!($fn));
                Err($crate::error::Error::HAL)
            }
            val if val >= 0 => Ok(val as usize),
            invalid => panic!("HAL call {} returned invalid value {}", stringify!($fn), invalid),
        }
    }
}
