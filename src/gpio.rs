//! Low-level GPIO API abstraction.
//!
//! Provides a common GPIO trait for adaptation between specifications and implementations.

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

/// Blanket implementation of Gpio for mutable references to Gpio
impl <T: Gpio> Gpio for &mut T {
    fn init(&mut self, port: i32, pin: i32, output: bool) -> Result<i32, Error> {
        <T as Gpio>::init(self, port, pin, output)
    }

    fn deinit(&mut self, handle: i32) -> Result<(), Error> {
        <T as Gpio>::deinit(self, handle)
    }

    fn set(&mut self, handle: i32, state: PinState) -> Result<(), Error> {
        <T as Gpio>::set(self, handle, state)
    }

    fn get(&mut self, handle: i32) -> Result<PinState, Error> {
        <T as Gpio>::get(self, handle)
    }
}

/// Blanket implementation of Gpio for dyn Gpio types
impl <T: Gpio> Gpio for Box<T> {
    fn init(&mut self, port: i32, pin: i32, output: bool) -> Result<i32, Error> {
        <T as Gpio>::init(self, port, pin, output)
    }

    fn deinit(&mut self, handle: i32) -> Result<(), Error> {
        <T as Gpio>::deinit(self, handle)
    }

    fn set(&mut self, handle: i32, state: PinState) -> Result<(), Error> {
        <T as Gpio>::set(self, handle, state)
    }

    fn get(&mut self, handle: i32) -> Result<PinState, Error> {
        <T as Gpio>::get(self, handle)
    }
}

/// Null GPIO implementation to simplify engine definitions
pub struct NullGpio;

impl Gpio for NullGpio {
    fn init(&mut self, _port: i32, _pin: i32, _output: bool) -> Result<i32, Error> {
        return Err(Error::Unsupported)
    }

    fn deinit(&mut self, _handle: i32) -> Result<(), Error> {
        return Err(Error::Unsupported)
    }

    fn set(&mut self, _handle: i32, _state: PinState) -> Result<(), Error> {
        return Err(Error::Unsupported)
    }

    fn get(&mut self, _handle: i32) -> Result<PinState, Error> {
        return Err(Error::Unsupported)
    }
}
