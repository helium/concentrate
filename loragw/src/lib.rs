use llg;
use std::ops::Drop;
use std::sync::atomic::{AtomicIsize, Ordering};

/// A common error type for this crate.
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
static GW_OPEN_CNT: AtomicIsize = AtomicIsize::new(0);

impl Gateway {
    pub fn open() -> Result<Self, Error> {
        // We can only have 0 or 1 instances
        if GW_OPEN_CNT.fetch_add(1, Ordering::SeqCst) > 0 {
            GW_OPEN_CNT.fetch_sub(1, Ordering::SeqCst);
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
        assert!(GW_OPEN_CNT.fetch_sub(1, Ordering::SeqCst) >= 0);
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
        assert!(GW_OPEN_CNT.load(Ordering::SeqCst) == 0);
        {
            let _gw = Gateway::open().unwrap();
            assert!(GW_OPEN_CNT.load(Ordering::SeqCst) == 1);
            // _gw `drop`ped here
        }
        assert!(GW_OPEN_CNT.load(Ordering::SeqCst) == 0);
    }

    #[test]
    fn test_double_open_fails() {
        let _lock = TEST_MUTEX.lock().unwrap();
        assert!(GW_OPEN_CNT.load(Ordering::SeqCst) == 0);
        let _gw1 = Gateway::open().unwrap();
        assert!(GW_OPEN_CNT.load(Ordering::SeqCst) == 1);
        assert!(Gateway::open().is_err());
    }
}
