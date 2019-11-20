use crate::error::AppResult;

#[cfg(feature = "sx1301")]
pub fn connect() -> AppResult {
    loragw::Concentrator::open()?.connect()?;
    Ok(())
}

#[cfg(feature = "sx1302")]
pub fn connect() -> AppResult {
    use ::std::ffi::CString;
    let path = CString::new("/dev/spidev0.0").unwrap();
    loragw::Concentrator::open()?.connect(&path)?;
    Ok(())
}
