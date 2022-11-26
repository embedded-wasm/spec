//! Platform UART API abstraction (and wrappers).
//!
//! Provides a platform UART trait with wiggle and c wrappers

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
