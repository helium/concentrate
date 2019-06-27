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
mod concentrator;
mod types;
pub use concentrator::*;
pub use error::*;
pub use types::*;
