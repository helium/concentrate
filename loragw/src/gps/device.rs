use crate::error::*;
use std::cell::Cell;
use std::fs::File;
use std::marker::PhantomData;
use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};

// Ensures we only have 0 or 1 GPS instances opened at a time.
// This is not a great solution, since another process has its
// own count.
static GPS_IS_OPEN: AtomicBool = AtomicBool::new(false);

/// A serial-attached GPS.
pub struct GPS(
    /// Used to prevent `self` from auto implementing `Sync`. This is
    /// necessary because the `libloragw` makes liberal use of globals
    /// and is not thread-safe.
    PhantomData<Cell<()>>,
);

impl GPS {
    /// Open the serial-attached GPS.
    pub fn open<P>(path: P, baud: u32) -> Result<(Self, File)>
    where
        P: AsRef<Path>,
    {
        use std::ffi::CString;
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

        Ok((GPS(PhantomData), tty))
    }
}
