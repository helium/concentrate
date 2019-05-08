use llg;
use std::ops::Drop;
use std::sync::atomic::{AtomicBool, Ordering};

/// A common error type for this crate.
#[derive(Debug)]
pub enum Error {
    Busy,
    HAL,
}

/// Converts `libloragw` return codes into a Result.
fn into_result(code: ::std::os::raw::c_int) -> Result<(), Error> {
    match code {
        0 => Ok(()),
        -1 => Err(Error::HAL),
        _ => panic!("unexpected return code: {}", code),
    }
}

pub struct Gateway {}

// Ensures we only have 0 or 1 gateway instances opened at a time.
//
// TODO: This is not a great solution, since another process has its
// own count.
static GW_IS_OPEN: AtomicBool = AtomicBool::new(false);

impl Gateway {
    pub fn open() -> Result<Self, Error> {
        // We can only 'open' one instance
        if GW_IS_OPEN.compare_and_swap(false, true, Ordering::Acquire) {
            return Err(Error::Busy);
        }
        Ok(Gateway {})
    }

    /// Connect to the LoRa concentrator, reset it and configure it
    /// according to previously set parameters.
    pub fn start(&self) -> Result<(), Error> {
        unsafe { into_result(llg::lgw_start()) }
    }

    /// Stop the LoRa concentrator and disconnect it.
    pub fn stop(&self) -> Result<(), Error> {
        unsafe { into_result(llg::lgw_stop()) }
    }
}

impl Drop for Gateway {
    fn drop(&mut self) {
        GW_IS_OPEN.store(false, Ordering::Release);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use lazy_static::lazy_static;
    use std::sync::Mutex;

    lazy_static! {
        static ref TEST_MUTEX: Mutex<()> = Mutex::new(());
    }

    #[test]
    fn test_open_close_succeeds() {
        let _lock = TEST_MUTEX.lock().unwrap();
        assert!(!GW_IS_OPEN.load(Ordering::Relaxed));
        {
            let _gw = Gateway::open().unwrap();
            assert!(GW_IS_OPEN.load(Ordering::Relaxed));
            // _gw `drop`ped here
        }
        assert!(!GW_IS_OPEN.load(Ordering::Relaxed));
    }

    #[test]
    fn test_double_open_fails() {
        let _lock = TEST_MUTEX.lock().unwrap();
        assert!(!GW_IS_OPEN.load(Ordering::Relaxed));
        let _gw1 = Gateway::open().unwrap();
        assert!(GW_IS_OPEN.load(Ordering::Relaxed));
        assert!(Gateway::open().is_err());
    }
}
