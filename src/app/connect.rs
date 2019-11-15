use crate::error::AppResult;

pub fn connect() -> AppResult {
    loragw::Concentrator::open()?.connect()?;
    Ok(())
}
