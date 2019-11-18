use crate::error::AppResult;
use std::ffi::CString;

pub fn connect() -> AppResult {
    let path = CString::new("/dev/spidev0.0").unwrap();
    loragw::Concentrator::open()?.connect(&path)?;
    Ok(())
}
