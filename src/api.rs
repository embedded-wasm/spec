//! API module provides wiggle and bindgen based platform API definitions.

#![allow(
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals,
    clippy::all
)]

use crate::Error;

#[cfg(feature = "bindgen")]
pub use cty::c_char;

// Load WITX interface specifications if enabled
// https://docs.rs/wiggle/0.28.0/wiggle/macro.from_witx.html
#[cfg(feature = "wiggle")]
wiggle::from_witx!({
    witx: [
        "./witx/common.witx",
        "./witx/spi.witx",
        "./witx/i2c.witx",
        "./witx/gpio.witx",
        "./witx/device.witx",
    ],
    errors: { errno => Error },
});

#[cfg(feature = "wiggle")]
pub use types::{Errno, UserErrorConversion};

#[cfg(feature = "wiggle")]
impl wiggle::GuestErrorType for types::Errno {
    fn success() -> Self {
        types::Errno::Ok
    }
}

// Include generated bindings if enabled
#[cfg(feature = "bindgen")]
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

/// Helper trait to generate typed driver objects
#[cfg(feature = "bindgen")]
pub trait Driver<Inner>: Sized {
    /// Driver method creates a typed adaptor for wasm3 use
    fn driver(&self) -> Inner;
}

#[cfg(feature = "wasmtime")]
use wasmtime::Linker;

#[cfg(feature = "wasmtime")]
pub trait Link<T, U> {
    fn link(
        l: &mut Linker<T>,
        get_cx: impl Fn(&mut T) -> &mut U + Send + Sync + Copy + 'static,
    ) -> Result<(), ()>;
}
