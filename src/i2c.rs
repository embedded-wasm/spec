//! Low-level I2C API abstraction.
//!
//! Provides a common I2C trait for adaptation between specifications and implementations.

use super::Error;

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

/// Blanket implementation of I2c for references to I2c types
impl<T: I2c> I2c for &mut T {
    fn init(&mut self, dev: u32, baud: u32, sda: i32, sck: i32) -> Result<i32, Error> {
        <T as I2c>::init(self, dev, baud, sda, sck)
    }

    fn deinit(&mut self, handle: i32) -> Result<(), Error> {
        <T as I2c>::deinit(self, handle)
    }

    fn write(&mut self, handle: i32, addr: u16, data: &[u8]) -> Result<(), Error> {
        <T as I2c>::write(self, handle, addr, data)
    }

    fn read(&mut self, handle: i32, addr: u16, buff: &mut [u8]) -> Result<(), Error> {
        <T as I2c>::read(self, handle, addr, buff)
    }

    fn write_read(
        &mut self,
        handle: i32,
        addr: u16,
        data: &[u8],
        buff: &mut [u8],
    ) -> Result<(), Error> {
        <T as I2c>::write_read(self, handle, addr, data, buff)
    }
}

/// Null I2c implementation to simplify engine instantiation
pub struct NullI2c;

impl I2c for NullI2c {
    fn init(&mut self, _dev: u32, _baud: u32, _sda: i32, _sck: i32) -> Result<i32, Error> {
        return Err(Error::Unsupported)
    }

    fn deinit(&mut self, _handle: i32) -> Result<(), Error> {
        return Err(Error::Unsupported)
    }

    fn write(&mut self, _handle: i32, _addr: u16, _data: &[u8]) -> Result<(), Error> {
        return Err(Error::Unsupported)
    }

    fn read(&mut self, _handle: i32, _addr: u16, _buff: &mut [u8]) -> Result<(), Error> {
        return Err(Error::Unsupported)
    }

    fn write_read(
        &mut self,
        _handle: i32,
        _addr: u16,
        _data: &[u8],
        _buff: &mut [u8],
    ) -> Result<(), Error> {
        return Err(Error::Unsupported)
    }
}
