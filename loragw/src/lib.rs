mod error;
mod types;
pub use error::*;
use llg;
use log;
use std::mem;
use std::ops;
use std::sync::atomic::{AtomicBool, Ordering};
pub use types::*;

// Ensures we only have 0 or 1 gateway instances opened at a time.
// This is not a great solution, since another process has its
// own count.
static GW_IS_OPEN: AtomicBool = AtomicBool::new(false);

pub struct Gateway;

impl Gateway {
    pub fn open() -> Result<Self> {
        log::trace!("opening concentrator");
        // We can only 'open' one instance
        if GW_IS_OPEN.compare_and_swap(false, true, Ordering::Acquire) {
            log::error!("concentrator busy");
            return Err(Error::Busy);
        }
        Ok(Gateway {})
    }

    /// Configure the gateway board.
    pub fn config_board(&self, conf: BoardConf) -> Result {
        log::trace!("conf: {:?}", conf);
        into_result(unsafe { llg::lgw_board_setconf(mem::transmute(conf)) })?;
        Ok(())
    }

    /// Configure an RF chain.
    pub fn config_rx_rf(&self, chain: u8, conf: RxRFConf) -> Result {
        log::trace!("chain: {}, conf: {:?}", chain, conf);
        into_result(unsafe { llg::lgw_rxrf_setconf(chain, mem::transmute(conf)) })?;
        Ok(())
    }

    /// Configure an IF chain + modem (must configure before start).
    pub fn config_rx_if(&self, chain: u8, conf: RxIFConf) -> Result {
        log::trace!("chain: {}, conf: {:?}", chain, conf);
        into_result(unsafe { llg::lgw_rxif_setconf(chain, mem::transmute(conf)) })?;
        Ok(())
    }

    /// Configure the Tx gain LUT.
    pub fn config_tx_gain(&self, lut: &mut TxGainLUT) -> Result {
        log::trace!("lut: {:?}", lut);
        into_result(unsafe {
            llg::lgw_txgain_setconf(lut as *mut TxGainLUT as *mut llg::lgw_tx_gain_lut_s)
        })?;
        Ok(())
    }

    /// according to previously set parameters.
    pub fn start(&self) -> Result {
        log::trace!("starting");
        into_result(unsafe { llg::lgw_start() })?;
        Ok(())
    }

    /// Stop the LoRa concentrator and disconnect it.
    pub fn stop(&self) -> Result {
        log::trace!("stopping");
        unsafe { into_result(llg::lgw_stop()) }?;
        Ok(())
    }

    /// Perform a non-blocking of up to 8 packets from concentrator's
    /// FIFO.
    pub fn receive(&self) -> Result<Vec<RxPacket>> {
        log::trace!("receive");
        let mut rx_buf: [llg::lgw_pkt_rx_s; 8] = [Default::default(); 8];
        let len = into_result(unsafe { llg::lgw_receive(8, rx_buf.as_mut_ptr()) })?;
        Ok(rx_buf[..len as usize]
            .iter()
            .map(|&pkt| pkt.into())
            .collect())
    }

    pub fn send(&self, _packet: TxPacket) -> Result {
        log::trace!("send");
        unimplemented!()
    }

    pub fn status(&self) -> Result {
        log::trace!("status");
        unimplemented!()
    }
}

impl ops::Drop for Gateway {
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
