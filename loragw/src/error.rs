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
    }
}

/// Converts `libloragw` return codes into a Result.
pub(crate) fn into_result(code: ::std::os::raw::c_int) -> Result<()> {
    match code {
        0 => Ok(()),
        -1 => Err(Error::HAL),
        _ => panic!("unexpected return code: {}", code),
    }
}
