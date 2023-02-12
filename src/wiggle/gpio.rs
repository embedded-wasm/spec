//! GPIO API adaptation from [Gpio] to wiggle generated [super::api::gpio::Gpio] interface

use crate::{Error, Gpio, wiggle::api::types};

use embedded_hal::digital::{PinState};

/// Wrapper for wiggle-generated GPIO api
impl <D: Gpio> super::api::gpio::Gpio for D {
    /// Initialise the provided GPIO pin in input or output mode
    fn init(&mut self, port: i32, pin: i32, mode: types::Mode) -> Result<i32, Error> {
        log::debug!("GPIO init port: {} pin: {} mode: {:?}", port, pin, mode);

        Gpio::init(self, port, pin, mode == types::Mode::Output)
    }

    /// Deinitialise the specified GPIO pin
    fn deinit(&mut self, dev: i32) -> Result<(), Error> {
        log::debug!("GPIO deinit handle: {}", dev);

        Gpio::deinit(self, dev)
    }

    /// Write to a GPIO pin
    fn set(&mut self, dev: i32, value: types::Value) -> Result<(), Error> {
        log::debug!("GPIO write handle: {} val: {:?}", dev, value);

        let state = match value {
            types::Value::High => PinState::High,
            types::Value::Low => PinState::Low,
        };

        Gpio::set(self, dev, state)
    }

    // Read from a GPIO pin
    fn get(&mut self, dev: i32) -> Result<types::Value, Error> {
        log::debug!("GPIO read handle: {}", dev);

        let r = Gpio::get(self, dev)?;

        match r {
            PinState::High => Ok(types::Value::High),
            PinState::Low => Ok(types::Value::Low),
        }
    }
}
