use crate::libloragw_sys::timespec;

/// Holds both GPS and GPS-derive UTC times.
#[allow(missing_docs)]
#[derive(Debug, Default)]
pub struct Times {
    pub utc: timespec,
    pub gps: timespec,
}
