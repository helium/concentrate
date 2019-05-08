use lgs;
use std::ops::Drop;
use std::sync::atomic::{AtomicIsize, Ordering};

pub struct Gateway {}

// Ensures we only have 0 or 1 gateway instances opened at a
// time.
//
// TODO: This is not a great solution, since another process has its
// own count.
static GW_OPEN_CNT: AtomicIsize = AtomicIsize::new(0);

impl Gateway {
    pub fn open() -> Result<Self, ()> {
        // We can only have 0 or 1 instances
        if GW_OPEN_CNT.fetch_add(1, Ordering::SeqCst) > 0 {
            GW_OPEN_CNT.fetch_sub(1, Ordering::SeqCst);
            return Err(());
        }
        // TODO: initialize hardware
        Ok(Gateway {})
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
