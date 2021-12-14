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

/// Embedded WASM Error type
///
/// Note these values _must_ correspond to WASM/WITX spec
#[derive(Clone, PartialEq, Debug)]
pub enum Error {
    InvalidArg,
    Unexpected,
    Failed,
    NoDevice,
}
