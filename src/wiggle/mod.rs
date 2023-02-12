//! Wiggle based Rust API definitions

#![allow(
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals,
    clippy::all
)]

mod uart;
mod gpio;
mod spi;
mod i2c;

/// Wiggle generated APIs
pub mod api {

    use crate::Error;

    // Load WITX interface specifications
    // https://docs.rs/wiggle/0.28.0/wiggle/macro.from_witx.html
    wiggle::from_witx!({
        witx: [
            "./witx/common.witx",
            "./witx/spi.witx",
            "./witx/i2c.witx",
            "./witx/uart.witx",
            "./witx/gpio.witx",
            "./witx/device.witx",
        ],
        errors: { errno => Error },
    });

    pub use types::{Errno, UserErrorConversion};

    impl wiggle::GuestErrorType for types::Errno {
        fn success() -> Self {
            types::Errno::Ok
        }
    }

    /// [UserErrorConversion] implementation on everything, 
    /// because it solves an annoying API requirement
    impl <T> UserErrorConversion for T {
        fn errno_from_error(&mut self, e: Error) -> Result<types::Errno, anyhow::Error> {
            match e {
                Error::InvalidArg => Ok(Errno::InvalidArg),
                Error::Unexpected => Ok(Errno::Unexpected),
                Error::Failed => Ok(Errno::Failed),
                Error::NoDevice => Ok(Errno::NoDevice),
                Error::Unsupported => Ok(Errno::Unsupported),
            }
        }
    }

}


