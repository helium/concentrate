use lfc_sys;
use lfc_sys::lfc;
use lfc_sys::lfc_user_cfg as LfcUserCfg;
use std::ffi::c_void;


struct LongFiCore {
    cb_data: usize,
    key: [u8; 16],
    c_handle: Option<lfc>
}

impl LongFiCore {
    pub fn new() -> LongFiCore {
        let mut ret = LongFiCore {
            c_handle: None,
            key: [0; 16],
            cb_data: 0,
        };
        ret.c_handle = Some( lfc {
           seq: 0,
           cfg: LfcUserCfg {
             cb_data: ret.cb_data as *mut c_void,
             key: ret.key[0] as *mut c_void,
             oui: 0x0,
             did: 0x1,
             key_len: 16,
           }
        });
        ret
    }
}