//! Embedded WASM Interface Specifications.
//!
//! This crate provides the WITX interface specification for embedded wasm platforms and HALs,
//! with bindings generated using wiggle for rust, and bindgen for C.
//!
//! https://github.com/embedded-wasm/spec

#![cfg_attr(not(feature = "std"), no_std)]
#![feature(associated_type_defaults)]


mod gpio;
pub use gpio::{Gpio, NullGpio};

mod i2c;
pub use i2c::{I2c, NullI2c};

mod spi;
pub use spi::{Spi, NullSpi};

mod uart;
pub use uart::{Uart, NullUart};

#[cfg(feature = "wiggle")]
pub mod wiggle;

#[cfg(feature = "bindgen")]
pub mod bindgen;


/// [Engine] abstraction provides low-level APIs for use by runtimes
pub trait Engine {
    type Gpio: Gpio = gpio::NullGpio;
    type I2c: I2c = i2c::NullI2c;
    type Spi: Spi = spi::NullSpi;
    type Uart: Uart = uart::NullUart;

    /// Fetch [Gpio] driver if available 
    fn gpio(&mut self) -> Option<&mut Self::Gpio> { return None }

    /// Fetch [I2c] driver if available 
    fn i2c(&mut self) -> Option<&mut Self::I2c> { return None }

    /// Fetch [Spi] driver if available 
    fn spi(&mut self) -> Option<&mut Self::Spi> { return None }

    /// Fetch [Uart] driver if available 
    fn uart(&mut self) -> Option<&mut Self::Uart> { return None }
}


/// Embedded WASM Error type
///
/// Note these values _must_ correspond to those defined in the WASM/WITX spec
#[derive(Clone, PartialEq, Debug)]
pub enum Error {
    InvalidArg,
    Unexpected,
    Failed,
    NoDevice,
    Unsupported,
}

