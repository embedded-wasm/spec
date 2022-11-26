//! Platform GPIO API abstraction (and wrappers).
//!
//! Provides a platform GPIO trait with wiggle and c wrappers

use embedded_hal::digital::PinState;

use super::Error;

/// Platform GPIO API abstraction.
///
/// This hides runtime implementation details to simplify implementing GPIO contexts.
/// Hopefully one day generation is improved so we don't _need_ this any more
#[cfg_attr(feature = "mocks", mockall::automock)]
pub trait Gpio {
    /// Initialise a GPIO by port and pin, returning a handle
    fn init(&mut self, port: i32, pin: i32, output: bool) -> Result<i32, Error>;

    /// Deinitialise a GPIO by handle
    fn deinit(&mut self, handle: i32) -> Result<(), Error>;

    /// Set a GPIO state by handle
    fn set(&mut self, handle: i32, state: PinState) -> Result<(), Error>;

    /// Fetch a GPIO state by handle
    fn get(&mut self, handle: i32) -> Result<PinState, Error>;
}
