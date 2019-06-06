//! This crate provides a high-level interface which serves as
//! building-block for creating LoRa gateways using the
//! [SX1301](https://www.semtech.com/products/wireless-rf/lora-gateways/sx1301)
//! concentrator chip.

#![deny(missing_docs)]

#[macro_use]
extern crate quick_error;
extern crate libloragw_sys;
#[macro_use]
extern crate log;
#[cfg(test)]
#[cfg_attr(test, macro_use)]
extern crate lazy_static;

#[macro_use]
mod error;
mod types;
pub use error::*;
use libloragw_sys as llg;
use std::convert::{TryFrom, TryInto};
use std::ops;
use std::sync::atomic::{AtomicBool, Ordering};
pub use types::*;

// Ensures we only have 0 or 1 gateway instances opened at a time.
// This is not a great solution, since another process has its
// own count.
static GW_IS_OPEN: AtomicBool = AtomicBool::new(false);

/// A LoRa concentrator.
pub struct Concentrator;

impl Concentrator {
    /// Open the spidev-connected concentrator.
    pub fn open() -> Result<Self> {
        trace!("opening concentrator");
        // We can only 'open' one instance
        if GW_IS_OPEN.compare_and_swap(false, true, Ordering::Acquire) {
            error!("concentrator busy");
            return Err(Error::Busy);
        }
        Ok(Concentrator {})
    }

    /// Configure the gateway board.
    pub fn config_board(&mut self, conf: &BoardConf) -> Result {
        trace!("conf: {:?}", conf);
        unsafe { hal_call!(lgw_board_setconf(conf.into())) }?;
        Ok(())
    }

    /// Configure an RF chain.
    pub fn config_rx_rf(&mut self, conf: &RxRFConf) -> Result {
        trace!("{:?}", conf);
        unsafe { hal_call!(lgw_rxrf_setconf(conf.radio as u8, conf.into())) }?;
        Ok(())
    }

    /// Configure an IF chain + modem (must configure before start).
    pub fn config_channel(&mut self, chain: u8, conf: &ChannelConf) -> Result {
        trace!("chain: {}, conf: {:?}", chain, conf);
        unsafe { hal_call!(lgw_rxif_setconf(chain, conf.into())) }?;
        Ok(())
    }

    /// Configure the Tx gain LUT.
    pub fn config_tx_gain(&mut self, gains: &[TxGain]) -> Result {
        if gains.is_empty() || gains.len() > 16 {
            error!(
                "gain table must contain 1 to 16 entries, {} provided",
                gains.len()
            );
            return Err(Error::Size);
        }
        trace!("gains: {:?}", gains);
        let mut lut = TxGainLUT::default();
        lut.lut[..gains.len()].clone_from_slice(gains);
        lut.size = gains.len() as u8;
        debug!("gains: {:?}", lut);
        unsafe {
            hal_call!(lgw_txgain_setconf(
                &mut lut as *mut TxGainLUT as *mut llg::lgw_tx_gain_lut_s
            ))
        }?;
        Ok(())
    }

    /// according to previously set parameters.
    pub fn start(&mut self) -> Result {
        trace!("starting");
        unsafe { hal_call!(lgw_start()) }?;
        Ok(())
    }

    /// Stop the LoRa concentrator and disconnect it.
    pub fn stop(&mut self) -> Result {
        trace!("stopping");
        unsafe { hal_call!(lgw_stop()) }?;
        Ok(())
    }

    /// Perform a non-blocking read of up to 16 packets from
    /// concentrator's FIFO.
    pub fn receive(&mut self) -> Result<Option<Vec<RxPacket>>> {
        trace!("receive");
        let mut tmp_buf: [llg::lgw_pkt_rx_s; 16] = [Default::default(); 16];
        let len = unsafe { hal_call!(lgw_receive(tmp_buf.len() as u8, tmp_buf.as_mut_ptr())) }?;
        if len > 0 {
            debug!("read {} packets out of concentrator", len);
            let mut out = Vec::new();
            for tmp in tmp_buf[..len].iter() {
                out.push(RxPacket::try_from(tmp)?);
            }
            Ok(Some(out))
        } else {
            Ok(None)
        }
    }

    /// Transmit `packet` over the air.
    pub fn transmit(&mut self, packet: TxPacket) -> Result {
        debug!("transmitting {:?}", packet);
        unsafe { hal_call!(lgw_send(packet.try_into()?)) }?;
        Ok(())
    }
}

impl ops::Drop for Concentrator {
    fn drop(&mut self) {
        GW_IS_OPEN.store(false, Ordering::Release);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;

    lazy_static! {
        static ref TEST_MUTEX: Mutex<()> = Mutex::new(());
    }

    #[test]
    fn test_open_close_succeeds() {
        let _lock = TEST_MUTEX.lock().unwrap();
        assert!(!GW_IS_OPEN.load(Ordering::Relaxed));
        {
            let _gw = Concentrator::open().unwrap();
            assert!(GW_IS_OPEN.load(Ordering::Relaxed));
            // _gw `drop`ped here
        }
        assert!(!GW_IS_OPEN.load(Ordering::Relaxed));
    }

    #[test]
    fn test_double_open_fails() {
        let _lock = TEST_MUTEX.lock().unwrap();
        assert!(!GW_IS_OPEN.load(Ordering::Relaxed));
        let _gw1 = Concentrator::open().unwrap();
        assert!(GW_IS_OPEN.load(Ordering::Relaxed));
        assert!(Concentrator::open().is_err());
    }

}
