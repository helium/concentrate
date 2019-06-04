use crate::number::*;
use nom::{le_i16, le_i32, le_i8, le_u32, le_u8};

/// This message reports the precise GPS time of the most recent
/// navigation solution including validity flags and an accuracy
/// estimate.
#[derive(Debug, Clone)]
pub struct TimeGps {
    /// GPS time of week of the navigation epoch.
    ///
    /// ### Unit
    /// millisecond
    pub iTOW: U4,

    /// Fractional part of iTOW (range: +/- 500000).
    ///
    /// The precise GPS time of week in seconds is:
    /// (iTOW * 1e-3) + (fTOW * 1e-9)
    ///
    /// ### Unit
    /// nanosecond
    pub fTOW: I4,

    /// GPS week number of the navigation epoch.
    ///
    /// ### Unit
    /// week
    pub week: I2,

    /// GPS leap seconds (GPS-UTC).
    ///
    /// ### Unit
    /// second
    pub leapS: I1,

    /// Validity Flags.
    pub valid: X1,

    /// Time Accuracy Estimate.
    ///
    /// ### Unit
    /// nanosecond
    pub tAcc: U4,
}

named_attr!(
    #[allow(missing_docs)],
    pub time_gps<&[u8], TimeGps>,
    do_parse!(tag!([0x20]) >>
              iTOW: le_u32 >>
              fTOW: le_i32 >>
              week: le_i16 >>
              leapS: le_i8 >>
              valid: le_u8 >>
              tAcc: le_u32 >>
              (TimeGps{iTOW, fTOW, week, leapS, valid, tAcc})
    )
);
