//! Platform I2C API abstraction (and wrappers).
//!
//! Provides a platform I2C trait with wiggle and c wrappers

use core::ops::{Deref, DerefMut};

use embedded_hal::spi::blocking::Operation;

use super::Error;

#[cfg(feature = "wiggle")]
use wiggle::GuestPtr;

#[cfg(feature = "wiggle")]
use super::api::types;

#[cfg(feature = "bindgen")]
use super::api::{self, Driver};

/// SPI context abstraction.
///
/// This hides runtime implementation details to simplify implementing I2C contexts.
/// Hopefully one day generation is improved so we don't _need_ this any more
pub trait Spi {
    fn init(
        &mut self,
        dev: u32,
        baud: u32,
        mosi: i32,
        miso: i32,
        sck: i32,
        cs: i32,
    ) -> Result<i32, Error>;

    fn deinit(&mut self, handle: i32) -> Result<(), Error>;

    fn write<'a>(&mut self, handle: i32, data: &[u8]) -> Result<(), Error>;

    fn transfer<'a>(&mut self, handle: i32, data: &mut [u8]) -> Result<(), Error>;

    fn exec<'a>(&mut self, handle: i32, ops: &[Operation<u8>]) -> Result<(), Error>;
}

/// Wrapper for wiggle-generated SPI APIs
#[cfg(feature = "wiggle")]
impl<T: Spi> crate::api::spi::Spi for T {
    fn init(
        &mut self,
        dev: u32,
        baud: u32,
        mosi: i32,
        miso: i32,
        sck: i32,
        cs: i32,
    ) -> Result<i32, Error> {
        log::debug!(
            "Opening SPI device: {} (baud: {} mosi: {} miso: {} sck: {} cs: {})",
            dev,
            baud,
            mosi,
            miso,
            sck,
            cs
        );
        Spi::init(self, dev, baud, mosi, miso, sck, cs)
    }

    fn deinit(&mut self, handle: i32) -> Result<(), Error> {
        log::debug!("Closing SPI handle: {}", handle);
        Spi::deinit(self, handle)
    }

    fn write<'a>(&mut self, handle: i32, data: &types::Wbytes<'a>) -> Result<(), Error> {
        let d = data.ptr.as_array(data.len);
        let d1 = d.as_slice().unwrap();

        log::debug!("Write SPI {} data: {:02x?}", handle, d1.deref());
        Spi::write(self, handle, d1.deref())
    }

    fn transfer<'a>(&mut self, handle: i32, data: &types::Rbytes<'a>) -> Result<(), Error> {
        let d = data.ptr.as_array(data.len);
        let mut d1 = d.as_slice_mut().unwrap();

        log::debug!("Transfer SPI {} data: {:02x?}", handle, d1.deref());
        Spi::transfer(self, handle, d1.deref_mut())
    }

    fn exec<'a>(&mut self, _handle: i32, ops: &GuestPtr<'a, [types::Op]>) -> Result<(), Error> {
        let _num_ops = ops.len();
        // TODO: idk yet how guest types etc. are going to work here
        todo!()
    }
}

/// Driver adaptor to C/wasm3 SPI API
#[cfg(feature = "bindgen")]
impl<T: Spi> Driver<api::spi_drv_t> for T {
    fn driver(&self) -> api::spi_drv_t {
        api::spi_drv_t {
            init: Some(wasm3::spi_init::<T>),
            deinit: Some(wasm3::spi_deinit::<T>),
            write: Some(wasm3::spi_write::<T>),
            transfer: Some(wasm3::spi_transfer::<T>),
            exec: None,
        }
    }
}

/// C handlers for WASM3 SPI API
#[cfg(feature = "bindgen")]
pub(super) mod wasm3 {

    use log::warn;

    use core::ffi::c_void;
    use core::slice;

    use super::Spi;

    pub extern "C" fn spi_init<T: Spi>(
        ctx: *mut c_void,
        dev: u32,
        baud: u32,
        mosi: i32,
        miso: i32,
        sck: i32,
        cs: i32,
    ) -> i32 {
        let ctx: &mut T = unsafe { &mut *(ctx as *mut T) };
        match Spi::init(ctx, dev, baud, mosi, miso, sck, cs) {
            Ok(i) => i,
            // TODO: not sure how to manage this yet
            Err(e) => {
                warn!("spi_init failed: {:?}", e);
                return -1;
            }
        }
    }

    pub extern "C" fn spi_deinit<T: Spi>(ctx: *mut c_void, handle: i32) -> i32 {
        let ctx: &mut T = unsafe { &mut *(ctx as *mut T) };
        match Spi::deinit(ctx, handle) {
            Ok(_) => 0,
            // TODO: not sure how to manage this yet
            Err(e) => {
                warn!("spi_deinit failed: {:?}", e);
                return -1;
            }
        }
    }

    pub extern "C" fn spi_write<T: Spi>(
        ctx: *mut c_void,
        handle: i32,
        data_out: *mut u8,
        length_out: u32,
    ) -> i32 {
        let ctx: &mut T = unsafe { &mut *(ctx as *mut T) };
        let data = unsafe { slice::from_raw_parts_mut(data_out, length_out as usize) };

        match Spi::write(ctx, handle, data) {
            Ok(_) => 0,
            // TODO: not sure how to manage this yet
            Err(e) => {
                warn!("spi_write failed: {:?}", e);
                return -1;
            }
        }
    }

    pub extern "C" fn spi_transfer<T: Spi>(
        ctx: *mut c_void,
        handle: i32,
        data: *mut u8,
        length: u32,
    ) -> i32 {
        let ctx: &mut T = unsafe { &mut *(ctx as *mut T) };
        let data = unsafe { slice::from_raw_parts_mut(data, length as usize) };

        match Spi::transfer(ctx, handle, data) {
            Ok(_) => 0,
            // TODO: not sure how to manage this yet
            Err(e) => {
                warn!("spi_transfer failed: {:?}", e);
                return -1;
            }
        }
    }
}
