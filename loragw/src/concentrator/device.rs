use super::types::*;
use crate::error::*;
use crate::libloragw_sys as llg;
use std::cell::Cell;
use std::convert::{TryFrom, TryInto};
use std::marker::PhantomData;
use std::ops;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time;

// Ensures we only have 0 or 1 gateway instances opened at a time.
// This is not a great solution, since another process has its
// own count.
static GW_IS_OPEN: AtomicBool = AtomicBool::new(false);

/// A LoRa concentrator.
pub struct Concentrator {
    /// Used to prevent `self` from auto implementing `Sync`.
    ///
    /// This is necessary because the `libloragw` makes liberal use of
    /// globals and is not thread-safe.
    _prevent_sync: PhantomData<Cell<()>>,
}

impl Concentrator {
    /// Open the spidev-connected concentrator.
    pub fn open() -> Result<Self> {
        // We can only 'open' one instance
        if GW_IS_OPEN.compare_and_swap(false, true, Ordering::Acquire) {
            error!("concentrator busy");
            return Err(Error::Busy);
        }
        Ok(Concentrator {
            _prevent_sync: PhantomData,
        })
    }

    /// Configure the gateway board.
    pub fn config_board(&self, conf: &BoardConf) -> Result {
        debug!("conf: {:?}", conf);
        unsafe { hal_call!(lgw_board_setconf(conf.into())) }?;
        Ok(())
    }

    /// Configure an RF chain.
    pub fn config_rx_rf(&self, conf: &RxRFConf) -> Result {
        debug!("{:?}", conf);
        unsafe { hal_call!(lgw_rxrf_setconf(conf.radio as u8, conf.into())) }?;
        Ok(())
    }

    /// Configure an IF chain + modem (must configure before start).
    pub fn config_channel(&self, chain: u8, conf: &ChannelConf) -> Result {
        debug!("chain: {}, conf: {:?}", chain, conf);
        unsafe { hal_call!(lgw_rxif_setconf(chain, conf.into())) }?;
        Ok(())
    }

    /// Configure the Tx gain LUT.
    pub fn config_tx_gain(&self, gains: &[TxGain]) -> Result {
        if gains.is_empty() || gains.len() > 16 {
            error!(
                "gain table must contain 1 to 16 entries, {} provided",
                gains.len()
            );
            return Err(Error::Size);
        }
        debug!("gains: {:?}", gains);
        let mut lut = TxGainLUT::default();
        lut.lut[..gains.len()].clone_from_slice(gains);
        lut.size = gains.len() as u8;
        unsafe {
            hal_call!(lgw_txgain_setconf(
                &mut lut as *mut TxGainLUT as *mut llg::lgw_tx_gain_lut_s
            ))
        }?;
        Ok(())
    }

    /// according to previously set parameters.
    pub fn start(&self) -> Result {
        info!("starting concentrator");
        unsafe { hal_call!(lgw_start()) }?;
        Ok(())
    }

    /// Stop the LoRa concentrator and disconnect it.
    pub fn stop(&self) -> Result {
        info!("stopping concentrator");
        unsafe { hal_call!(lgw_stop()) }?;
        Ok(())
    }

    /// Perform a non-blocking read of up to 16 packets from
    /// concentrator's FIFO.
    pub fn receive(&self) -> Result<Option<Vec<RxPacket>>> {
        let mut tmp_buf: [llg::lgw_pkt_rx_s; 16] = [Default::default(); 16];
        let len = unsafe { hal_call!(lgw_receive(tmp_buf.len() as u8, tmp_buf.as_mut_ptr())) }?;
        if len > 0 {
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
    pub fn transmit(&self, packet: TxPacket) -> Result {
        unsafe { hal_call!(lgw_send(packet.try_into()?)) }?;
        Ok(())
    }

    /// Returns value of internal counter when latest event (e.g. GPS pulse) was captured.
    pub fn last_trigger(&self) -> Result<time::Duration> {
        let mut cnt_us = 0u32;
        unsafe { hal_call!(lgw_get_trigcnt(&mut cnt_us)) }?;
        Ok(time::Duration::from_micros(u64::from(cnt_us)))
    }
}

impl ops::Drop for Concentrator {
    fn drop(&mut self) {
        info!("closing concentrator");
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