#![allow(non_snake_case)]
#![deny(missing_docs)]

//! A collection of types and parsers for u-blox v8 messages.

extern crate byteorder;
#[macro_use]
extern crate nom;

pub mod coding;
pub mod nav;
pub mod primitive;
