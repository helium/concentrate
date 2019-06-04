//! Navigation messages.

mod timegps;
pub use timegps::*;

/// Navigation Results Messages
///
/// Includes:
/// - Position
/// - Speed
/// - Time
/// - Acceleration
/// - Heading
/// - DOP
/// - SVs used
#[allow(missing_docs)]
#[derive(Debug, Clone)]
pub enum Nav {
    TimeGps(TimeGps),
}

named_attr!(
    #[allow(missing_docs)],
    pub navmsg<&[u8], Nav>,
    do_parse!(tag!([0x01]) >>
              navmsg: alt!(
                  time_gps => { | msg | Nav::TimeGps(msg) }
              ) >>
              (navmsg)
    )
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_nav_timegps() {
        let msg = vec![0x01, 0x20, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        navmsg(&msg).unwrap();
    }
}
