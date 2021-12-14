//! Platform I2C API abstraction (and wrappers).
//!
//! Provides a platform I2C trait with wiggle and c wrappers

use core::ops::{Deref, DerefMut};

use super::Error;

#[cfg(feature = "wiggle")]
use super::api::types;

#[cfg(feature = "bindgen")]
use super::api::{self, Driver};

/// I2C context abstraction.
///
/// This hides runtime implementation details to simplify implementing I2C contexts.
/// Hopefully one day generation is improved so we don't _need_ this any more
pub trait I2c {
    fn init(&mut self, dev: u32, _baud: u32, sda: i32, sck: i32) -> Result<i32, Error>;

    fn deinit(&mut self, handle: i32) -> Result<(), Error>;

    fn write(&mut self, handle: i32, addr: u16, data: &[u8]) -> Result<(), Error>;

    fn read(&mut self, handle: i32, addr: u16, buff: &mut [u8]) -> Result<(), Error>;

    fn write_read(
        &mut self,
        handle: i32,
        addr: u16,
        data: &[u8],
        buff: &mut [u8],
    ) -> Result<(), Error>;
}

/// Wrapper for wiggle-generated I2C api
#[cfg(feature = "wiggle")]
impl<T: I2c> crate::api::i2c::I2c for T {
    fn init(&mut self, port: u32, baud: u32, sda: i32, scl: i32) -> Result<i32, Error> {
        log::debug!(
            "Opening I2C port: {} (baud: {} sda: {} scl: {})",
            port,
            baud,
            sda,
            scl
        );
        I2c::init(self, port, baud, sda, scl)
    }

    fn deinit(&mut self, handle: i32) -> Result<(), Error> {
        log::debug!("Closing I2C handle: {}", handle);
        I2c::deinit(self, handle)
    }

    /// Write to an I2c device
    fn write(&mut self, handle: i32, addr: u16, data: &types::Rbytes) -> Result<(), Error> {
        let d = data.ptr.as_array(data.len);
        let d1 = d.as_slice_mut().unwrap();

        log::debug!(
            "I2C write handle: {} addr: {} data: {:02x?}",
            handle,
            addr,
            d1.deref()
        );

        I2c::write(self, handle, addr, d1.deref())
    }

    /// Read from an I2c device
    fn read(&mut self, handle: i32, addr: u16, buff: &types::Wbytes) -> Result<(), Error> {
        let b = buff.ptr.as_array(buff.len);
        let mut b1 = b.as_slice_mut().unwrap();

        log::debug!("I2C read handle: {} addr: {}", handle, addr);

        I2c::read(self, handle, addr, b1.deref_mut())
    }

    /// Write to and read from an I2c device on the specified peripheral
    fn write_read(
        &mut self,
        handle: i32,
        addr: u16,
        data: &types::Rbytes,
        buff: &types::Wbytes,
    ) -> Result<(), Error> {
        let d = data.ptr.as_array(data.len);
        let d1 = d.as_slice().unwrap();

        let b = buff.ptr.as_array(buff.len);
        let mut b1 = b.as_slice_mut().unwrap();

        log::debug!(
            "I2C write_read dev: {} addr: {} write: {:02x?}",
            handle,
            addr,
            d1.deref()
        );

        I2c::write_read(self, handle, addr, d1.deref(), b1.deref_mut())
    }
}

/// Driver adaptor to C/wasm3 I2C API
#[cfg(feature = "bindgen")]
impl<T: I2c> Driver<api::i2c_drv_t> for T {
    fn driver(&self) -> api::i2c_drv_t {
        api::i2c_drv_t {
            init: Some(wasm3::i2c_init::<T>),
            deinit: Some(wasm3::i2c_deinit::<T>),
            write: Some(wasm3::i2c_write::<T>),
            read: Some(wasm3::i2c_read::<T>),
            write_read: Some(wasm3::i2c_write_read::<T>),
        }
    }
}

#[cfg(feature = "bindgen")]
pub(super) mod wasm3 {

    use core::ffi::c_void;
    use core::slice;

    use log::warn;

    use super::I2c;

    pub extern "C" fn i2c_init<T: I2c>(
        ctx: *mut c_void,
        dev: u32,
        baud: u32,
        sda: i32,
        scl: i32,
    ) -> i32 {
        let ctx: &mut T = unsafe { &mut *(ctx as *mut T) };
        match I2c::init(ctx, dev, baud, sda, scl) {
            Ok(i) => i,
            // TODO: not sure how to manage this yet
            Err(e) => {
                warn!("{:?}", e);
                return -1;
            }
        }
    }

    pub extern "C" fn i2c_deinit<T: I2c>(ctx: *mut c_void, handle: i32) -> i32 {
        let ctx: &mut T = unsafe { &mut *(ctx as *mut T) };
        match I2c::deinit(ctx, handle) {
            Ok(_) => 0,
            // TODO: not sure how to manage this yet
            Err(e) => {
                warn!("{:?}", e);
                return -1;
            }
        }
    }

    pub extern "C" fn i2c_read<T: I2c>(
        ctx: *mut c_void,
        handle: i32,
        address: u16,
        data_in: *mut u8,
        length_in: u32,
    ) -> i32 {
        let ctx: &mut T = unsafe { &mut *(ctx as *mut T) };
        let buff = unsafe { slice::from_raw_parts_mut(data_in, length_in as usize) };

        match I2c::read(ctx, handle, address, buff) {
            Ok(_) => 0,
            // TODO: not sure how to manage this yet
            Err(e) => {
                warn!("{:?}", e);
                return -1;
            }
        }
    }

    pub extern "C" fn i2c_write<T: I2c>(
        ctx: *mut c_void,
        handle: i32,
        address: u16,
        data_out: *mut u8,
        length_out: u32,
    ) -> i32 {
        let ctx: &mut T = unsafe { &mut *(ctx as *mut T) };
        let data = unsafe { slice::from_raw_parts_mut(data_out, length_out as usize) };

        match I2c::write(ctx, handle, address, data) {
            Ok(_) => 0,
            // TODO: not sure how to manage this yet
            Err(e) => {
                warn!("{:?}", e);
                return -1;
            }
        }
    }

    pub extern "C" fn i2c_write_read<T: I2c>(
        ctx: *mut c_void,
        handle: i32,
        address: u16,
        data_out: *mut u8,
        length_out: u32,
        data_in: *mut u8,
        length_in: u32,
    ) -> i32 {
        let ctx: &mut T = unsafe { &mut *(ctx as *mut T) };
        let data = unsafe { slice::from_raw_parts_mut(data_out, length_out as usize) };
        let buff = unsafe { slice::from_raw_parts_mut(data_in, length_in as usize) };

        match I2c::write_read(ctx, handle, address, data, buff) {
            Ok(_) => 0,
            // TODO: not sure how to manage this yet
            Err(e) => {
                warn!("{:?}", e);
                return -1;
            }
        }
    }
}
