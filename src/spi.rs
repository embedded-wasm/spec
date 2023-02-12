//! Low-level I2C API abstraction.
//!
//! Provides a common I2C trait for adaptation between specifications and implementations.

use super::Error;

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

    fn read<'a>(&mut self, handle: i32, data: &mut [u8]) -> Result<(), Error>;

    fn write<'a>(&mut self, handle: i32, data: &[u8]) -> Result<(), Error>;

    fn transfer<'a>(&mut self, handle: i32, read: &mut [u8], write: &[u8]) -> Result<(), Error>;

    fn transfer_inplace<'a>(&mut self, handle: i32, data: &mut [u8]) -> Result<(), Error>;
}

/// Blanket I2c implementation for references to I2c types
impl<T: Spi> Spi for &mut T {
    fn init(
        &mut self,
        dev: u32,
        baud: u32,
        mosi: i32,
        miso: i32,
        sck: i32,
        cs: i32,
    ) -> Result<i32, Error> {
        <T as Spi>::init(self, dev, baud, mosi, miso, sck, cs)
    }

    fn deinit(&mut self, handle: i32) -> Result<(), Error> {
        <T as Spi>::deinit(self, handle)
    }

    fn read<'a>(&mut self, handle: i32, data: &mut [u8]) -> Result<(), Error> {
        <T as Spi>::read(self, handle, data)
    }

    fn write<'a>(&mut self, handle: i32, data: &[u8]) -> Result<(), Error> {
        <T as Spi>::write(self, handle, data)
    }

    fn transfer<'a>(&mut self, handle: i32, read: &mut [u8], write: &[u8]) -> Result<(), Error> {
        <T as Spi>::transfer(self, handle, read, write)
    }

    fn transfer_inplace<'a>(&mut self, handle: i32, data: &mut [u8]) -> Result<(), Error> {
        <T as Spi>::transfer_inplace(self, handle, data)
    }
}

pub struct NullSpi;

impl Spi for NullSpi {
    fn init(
        &mut self,
        _dev: u32,
        _baud: u32,
        _mosi: i32,
        _miso: i32,
        _sck: i32,
        _cs: i32,
    ) -> Result<i32, Error> {
        Err(Error::Unsupported)
    }

    fn deinit(&mut self, _handle: i32) -> Result<(), Error> {
        Err(Error::Unsupported)
    }

    fn read<'a>(&mut self, _handle: i32, _data: &mut [u8]) -> Result<(), Error> {
        Err(Error::Unsupported)
    }

    fn write<'a>(&mut self, _handle: i32, _data: &[u8]) -> Result<(), Error> {
        Err(Error::Unsupported)
    }

    fn transfer<'a>(&mut self, _handle: i32, _read: &mut [u8], _write: &[u8]) -> Result<(), Error> {
        Err(Error::Unsupported)
    }

    fn transfer_inplace<'a>(&mut self, _handle: i32, _data: &mut [u8]) -> Result<(), Error> {
        Err(Error::Unsupported)
    }
}