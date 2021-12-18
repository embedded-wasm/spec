//! Platform GPIO API abstraction (and wrappers).
//!
//! Provides a platform GPIO trait with wiggle and c wrappers

use embedded_hal::digital::PinState;

use super::Error;

#[cfg(feature = "wiggle")]
use crate::api::types;

#[cfg(feature = "bindgen")]
use crate::api::{self, Driver};

/// Platform GPIO API abstraction.
///
/// This hides runtime implementation details to simplify implementing GPIO contexts.
/// Hopefully one day generation is improved so we don't _need_ this any more
pub trait Gpio {
    /// Initialise a GPIO by port and pin, returning a handle
    fn init(&mut self, port: u32, pin: u32, output: bool) -> Result<i32, Error>;

    /// Deinitialise a GPIO by handle
    fn deinit(&mut self, handle: i32) -> Result<(), Error>;

    /// Set a GPIO state by handle
    fn set(&mut self, handle: i32, state: PinState) -> Result<(), Error>;

    /// Fetch a GPIO state by handle
    fn get(&mut self, handle: i32) -> Result<PinState, Error>;
}

/// Wrapper for wiggle-generated I2C api
#[cfg(feature = "wiggle")]
impl<T: Gpio> crate::api::gpio::Gpio for T {
    /// Initialise the provided GPIO pin in input or output mode
    fn init(&mut self, port: u32, pin: u32, mode: types::Mode) -> Result<i32, Error> {
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

/// Driver adaptor to C/wasm3 I2C API
#[cfg(feature = "bindgen")]
impl<T: Gpio> Driver<api::gpio_drv_t> for T {
    fn driver(&self) -> api::gpio_drv_t {
        api::gpio_drv_t {
            init: Some(wasm3::gpio_init::<T>),
            deinit: Some(wasm3::gpio_deinit::<T>),
            set: Some(wasm3::gpio_set::<T>),
            get: Some(wasm3::gpio_get::<T>),
        }
    }
}

#[cfg(feature = "bindgen")]
pub(super) mod wasm3 {
    use core::ffi::c_void;

    use super::Gpio;
    use embedded_hal::digital::PinState;

    pub extern "C" fn gpio_init<T: Gpio>(
        ctx: *const c_void,
        port: u32,
        pin: u32,
        output: u32,
    ) -> i32 {
        let ctx: &mut T = unsafe { &mut *(ctx as *mut T) };
        match Gpio::init(ctx, port, pin, output != 0) {
            Ok(i) => i,
            // TODO: not sure how to manage this yet
            Err(e) => {
                log::debug!("gpio_init error: {:?}", e);
                return -1;
            }
        }
    }

    pub extern "C" fn gpio_deinit<T: Gpio>(ctx: *const c_void, handle: i32) -> i32 {
        let ctx: &mut T = unsafe { &mut *(ctx as *mut T) };
        match Gpio::deinit(ctx, handle) {
            Ok(_) => 0,
            // TODO: not sure how to manage this yet
            Err(e) => {
                log::debug!("gpio_deinit error: {:?}", e);
                return -1;
            }
        }
    }

    pub extern "C" fn gpio_get<T: Gpio>(ctx: *const c_void, handle: i32, value: *mut u32) -> i32 {
        let ctx: &mut T = unsafe { &mut *(ctx as *mut T) };

        match Gpio::get(ctx, handle) {
            Ok(PinState::High) => {
                unsafe { *value = 1 };
                0
            }
            Ok(PinState::Low) => {
                unsafe { *value = 0 };
                0
            }
            // TODO: not sure how to manage this yet
            Err(e) => {
                log::debug!("gpio_get error: {:?}", e);
                return -1;
            }
        }
    }

    pub extern "C" fn gpio_set<T: Gpio>(ctx: *const c_void, handle: i32, value: u32) -> i32 {
        let ctx: &mut T = unsafe { &mut *(ctx as *mut T) };

        let state = if value == 0 {
            PinState::Low
        } else {
            PinState::High
        };

        match Gpio::set(ctx, handle, state) {
            Ok(_) => 0,
            // TODO: not sure how to manage this yet
            Err(e) => {
                log::debug!("gpio_set error: {:?}", e);
                return -1;
            }
        }
    }
}
