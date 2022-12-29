//! Embedded WASM Interface Specifications.
//!
//! This crate provides the WITX interface specification for embedded wasm platforms and HALs,
//! with bindings generated using wiggle for rust, and bindgen for C.
//!
//! https://github.com/embedded-wasm/spec

#![cfg_attr(not(feature = "std"), no_std)]

pub mod api;

pub mod gpio;
pub mod i2c;
pub mod spi;
pub mod uart;

/// Embedded WASM Error type
///
/// Note these values _must_ correspond to WASM/WITX spec
#[derive(Clone, PartialEq, Debug)]
pub enum Error {
    InvalidArg,
    Unexpected,
    Failed,
    NoDevice,
    Unsupported,
}

/// Core syscall handler trait
pub trait Core {
    /// Execute the provided syscall
    fn exec(&mut self, cla: u32, ins: u32, flags: u32, cmd: &[u8], resp: &[u8]) -> Result<i32, Error>;
}

#[cfg(feature="wiggle")]
impl From<Error> for crate::api::types::Errno {
    fn from(e: Error) -> crate::api::types::Errno {
        use crate::api::types::Errno;

        match e {
            Error::InvalidArg => Errno::InvalidArg,
            Error::Unexpected => Errno::Unexpected,
            Error::Failed => Errno::Failed,
            Error::NoDevice => Errno::NoDevice,
            Error::Unsupported => Errno::Unsupported,
        }

    }
}
