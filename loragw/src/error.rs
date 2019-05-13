use quick_error::quick_error;

/// A common result type for this crate.
pub type Result<T = ()> = ::std::result::Result<T, Error>;

/// A common error type for this crate.
quick_error! {
    #[derive(Debug)]
    pub enum Error {
        Busy {
            description("concentrator device is already in use")
        }
        HAL {
            description("concentrator HAL returned a generic error")
        }
        Size {
            description("provided buffer is too large")
        }
    }
}

/// Converts `libloragw` return codes into a Result.
pub(crate) fn into_result(code: ::std::os::raw::c_int) -> Result<usize> {
    match code {
        -1 => Err(Error::HAL),
        val if val >= 0 => Ok(val as usize),
        _ => panic!("unexpected return code: {}", code),
    }
}
