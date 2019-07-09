use super::framing::Frame;
use super::types::Times;
use crate::error::*;
use crate::libloragw_sys::{self, timespec, tref};
use std::{
    cell::Cell,
    ffi::CString,
    fs::File,
    marker::PhantomData,
    path::Path,
    ptr,
    sync::atomic::{AtomicBool, Ordering},
    time::{Duration, SystemTime},
};

// Ensures we only have 0 or 1 GPS instances opened at a time.
// This is not a great solution, since another process has its
// own count.
static GPS_IS_OPEN: AtomicBool = AtomicBool::new(false);

/// A serial-attached GPS.
pub struct GPS<'a> {
    concentrator: Option<&'a crate::Concentrator>,
    /// Time reference used for GPS <-> timestamp conversion.
    gps_time_ref: tref,
    /// Used to prevent `self` from auto implementing `Sync`. This is
    /// necessary because the `libloragw` makes liberal use of globals
    /// and is not thread-safe.
    _no_sync: PhantomData<Cell<()>>,
}

impl<'a> GPS<'a> {
    /// Open the serial-attached GPS.
    pub fn open<P>(
        path: P,
        baud: u32,
        concentrator: Option<&'a crate::Concentrator>,
    ) -> Result<(Self, File)>
    where
        P: AsRef<Path>,
    {
        use std::os::raw::c_char;
        use std::os::unix::ffi::OsStringExt;
        use std::os::unix::io::FromRawFd;

        // We can only 'open' one instance
        if GPS_IS_OPEN.compare_and_swap(false, true, Ordering::Acquire) {
            error!("GPS busy");
            return Err(Error::Busy);
        }

        let tty_path = CString::new(path.as_ref().as_os_str().to_owned().into_vec())
            .expect("Paths never have null bytes");
        let gps_family = CString::new("ubx7").expect("non c-string GPS family");

        let tty = unsafe {
            let mut fd = -1;
            hal_call!(lgw_gps_enable(
                tty_path.as_bytes_with_nul().as_ptr() as *mut c_char,
                gps_family.as_bytes_with_nul().as_ptr() as *mut c_char,
                baud,
                &mut fd
            ))?;
            File::from_raw_fd(fd)
        };

        Ok((
            GPS {
                concentrator,
                gps_time_ref: tref::default(),
                _no_sync: PhantomData,
            },
            tty,
        ))
    }

    /// Parse and update internal state using a GPS `Frame`.
    pub fn parse(&mut self, frame: Frame) -> Result {
        let msg_kind = match frame {
            Frame::Nmea(msg) => self.parse_nmea(msg),
            Frame::Ublox(msg) => self.parse_ublox(msg),
        };

        if msg_kind == libloragw_sys::gps_msg_UBX_NAV_TIMEGPS {
            self.sync()
        } else {
            Ok(())
        }
    }

    /// Convert concentrator timestamp counter value to UTC time.
    ///
    /// This function is typically used when a packet is received to
    /// transform the internal counter-based timestamp in an absolute
    /// timestamp with an accuracy in the order of a couple
    /// microseconds (ns resolution).
    pub fn systemtime_from_timestamp(&self, timestamp: Duration) -> Result<SystemTime> {
        let mut timespec = timespec::default();
        unsafe {
            hal_call!(lgw_cnt2utc(
                self.gps_time_ref,
                timestamp.as_micros() as u32,
                &mut timespec
            ))?
        };
        Ok(SystemTime::from(timespec))
    }
}

impl<'a> GPS<'a> {
    fn parse_nmea(&self, msg: CString) -> libloragw_sys::gps_msg {
        let msg = msg.as_bytes_with_nul();
        unsafe { libloragw_sys::lgw_parse_nmea(msg.as_ptr() as *const i8, msg.len() as i32) }
    }

    fn parse_ublox(&self, msg: Vec<u8>) -> libloragw_sys::gps_msg {
        // an output param we're going to ignore.
        let mut msg_size = 0usize;
        unsafe {
            libloragw_sys::lgw_parse_ubx(
                msg.as_ptr() as *const i8,
                msg.len() as usize,
                &mut msg_size as *mut usize,
            )
        }
    }

    // Get the GPS solution (space & time).
    fn times(&self) -> Result<Times> {
        let mut times = Times::default();
        unsafe {
            hal_call!(lgw_gps_get(
                &mut times.utc,
                &mut times.gps,
                ptr::null_mut(),
                ptr::null_mut()
            ))?
        };
        Ok(times)
    }

    fn sync(&mut self) -> Result {
        let trig_cnt;

        if let Some(concentrator) = self.concentrator {
            trig_cnt = concentrator.last_trigger()?.as_micros() as u32;
        } else {
            return Ok(());
        };

        let times = self.times()?;

        unsafe {
            hal_call!(lgw_gps_sync(
                &mut self.gps_time_ref,
                trig_cnt,
                times.utc,
                times.gps
            ))?
        };
        Ok(())
    }
}
