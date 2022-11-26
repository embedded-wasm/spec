//! Platform I2C API abstraction (and wrappers).
//!
//! Provides a platform I2C trait with wiggle and c wrappers


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
