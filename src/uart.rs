//! Low-Level UART API abstraction.
//!
//! Provides a common UART trait for adaptation between specifications and implementations.

use super::Error;

/// UART context abstraction.
///
/// This hides runtime implementation details to simplify implementing UART contexts.
/// Hopefully one day generation is improved so we don't _need_ this any more
pub trait Uart {
    fn init(&mut self, dev: u32, baud: u32, tx: i32, rx: i32) -> Result<i32, Error>;

    fn deinit(&mut self, handle: i32) -> Result<(), Error>;

    fn write(&mut self, handle: i32, flags: u32, data: &[u8]) -> Result<(), Error>;

    fn read(&mut self, handle: i32, flags: u32, buff: &mut [u8]) -> Result<(), Error>;
}

/// Blanket Uart implementation for &mut T where T: Uart
impl<T: Uart> Uart for &mut T {
    fn init(&mut self, dev: u32, baud: u32, tx: i32, rx: i32) -> Result<i32, Error> {
        <T as Uart>::init(self, dev, baud, tx, rx)
    }

    fn deinit(&mut self, handle: i32) -> Result<(), Error> {
        <T as Uart>::deinit(self, handle)
    }

    fn write(&mut self, handle: i32, flags: u32, data: &[u8]) -> Result<(), Error> {
        <T as Uart>::write(self, handle, flags, data)
    }

    fn read(&mut self, handle: i32, flags: u32, buff: &mut [u8]) -> Result<(), Error> {
        <T as Uart>::read(self, handle, flags, buff)
    }
}

/// Null [Uart] implementation to simplify engine instantiation
pub struct NullUart;

impl Uart for NullUart {
    fn init(&mut self, _dev: u32, _baud: u32, _tx: i32, _rx: i32) -> Result<i32, Error> {
        Err(Error::Unsupported)
    }

    fn deinit(&mut self, _handle: i32) -> Result<(), Error> {
        Err(Error::Unsupported)
    }

    fn write(&mut self, _handle: i32, _flags: u32, _data: &[u8]) -> Result<(), Error> {
        Err(Error::Unsupported)
    }

    fn read(&mut self, _handle: i32, _flags: u32, _buff: &mut [u8]) -> Result<(), Error> {
        Err(Error::Unsupported)
    }
}
