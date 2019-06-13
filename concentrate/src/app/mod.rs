use std::fmt;

mod listen;
mod send;
mod serve;

pub use self::listen::*;
pub use self::send::*;
pub use self::serve::*;

fn print_at_level<T: fmt::Debug>(print_level: u8, pkt: &T) {
    if print_level > 1 {
        println!("{:#?}\n", pkt);
    } else if print_level == 1 {
        println!("{:?}\n", pkt);
    }
}
