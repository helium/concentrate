use quick_error::quick_error;
use toml;

quick_error! {
    /// A common error type for this crate.
    #[derive(Debug)]
    pub enum Error {
        /// Concentrator-specific error.
        Concentrator (err: loragw::Error) {
            from()
            description(err.description())
        }
        /// IO failure.
        IO(err: ::std::io::Error) {
            from()
            description(err.description())
        }
        /// Configuration file error.
        Config(err: toml::de::Error) {
            from()
            description(err.description())
        }
        /// Catch-all error.
        Generic(err: String) {
            from()
            description(err)
        }
    }
}

/// A common result type for this crate.
pub type Result<T = ()> = ::std::result::Result<T, Error>;
