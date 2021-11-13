
#![cfg_attr(not(feature = "std"), no_std)]

pub mod api;

pub mod gpio;
pub mod spi;
pub mod i2c;

/// EWASM Error type
///
/// Must correspond to WASM spec
#[derive(Clone, PartialEq, Debug)]
pub enum Error {
    InvalidArg,
    Unexpected,
    Failed,
    NoDevice
}
