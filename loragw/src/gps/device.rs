use super::framing::Frame;
use crate::error::*;
use crate::libloragw_sys;
use std::{
    cell::Cell,
    ffi::CString,
    fs::File,
    marker::PhantomData,
    path::Path,
    sync::atomic::{AtomicBool, Ordering},
};

// Ensures we only have 0 or 1 GPS instances opened at a time.
// This is not a great solution, since another process has its
// own count.
static GPS_IS_OPEN: AtomicBool = AtomicBool::new(false);

/// A serial-attached GPS.
pub struct GPS<'a> {
    concentrator: Option<&'a crate::Concentrator>,
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
                _no_sync: PhantomData,
            },
            tty,
        ))
    }

    /// Parse and update internal state using a GPS `Frame`.
    pub fn parse(&self, frame: Frame) {
        let _msg_kind = match frame {
            Frame::Nmea(msg) => self.parse_nmea(msg),
            Frame::Ublox(msg) => self.parse_ublox(msg),
        };
    }
}

impl<'a> GPS<'a> {
    fn parse_nmea(&self, msg: CString) -> libloragw_sys::gps_msg {
        let msg = msg.as_bytes_with_nul();
        unsafe { libloragw_sys::lgw_parse_nmea(msg.as_ptr(), msg.len() as i32) }
    }

    fn parse_ublox(&self, msg: Vec<u8>) -> libloragw_sys::gps_msg {
        // an output param we're going to ignore.
        let mut msg_size = 0usize;
        unsafe {
            libloragw_sys::lgw_parse_ubx(
                msg.as_ptr(),
                msg.len() as usize,
                &mut msg_size as *mut usize,
            )
        }
    }
}
