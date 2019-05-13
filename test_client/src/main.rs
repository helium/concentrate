use loragw::{Gateway};
use env_logger;

fn main() {
    env_logger::init();
    let mut concentrator = Gateway::open().unwrap();
    concentrator.start().unwrap();
}
