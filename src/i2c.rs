//! Platform I2C API abstraction (and wrappers).
//!
//! Provides a platform I2C trait with wiggle and c wrappers

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
