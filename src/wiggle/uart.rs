use std::ops::{Deref, DerefMut};

use crate::{Error, Uart, wiggle::api::types};

/// Wrapper for wiggle-generated UART api
impl <D: Uart> super::api::uart::Uart for D {
    fn init(&mut self, port: u32, baud: u32, tx: i32, rx: i32) -> Result<i32, Error> {
        log::debug!(
            "Opening UART port: {} (baud: {} tx: {} rx: {})",
            port,
            baud,
            tx,
            rx
        );

        Uart::init(self, port, baud, tx, rx)
    }

    fn deinit(&mut self, handle: i32) -> Result<(), Error> {
        log::debug!("Closing UART handle: {}", handle);

        Uart::deinit(self, handle)
    }

    /// Write to an Uart device
    fn write(&mut self, handle: i32, flags: i32, data: &types::Rbytes) -> Result<(), Error> {
        let d = data.ptr.as_array(data.len);
        let d1 = d.as_slice_mut().unwrap().unwrap();

        log::debug!(
            "UART write handle: {} flags: {} data: {:02x?}",
            handle,
            flags,
            d1.deref()
        );

        Uart::write(self, handle, flags as u32, d1.deref())
    }

    /// Read from an Uart device
    fn read(&mut self, handle: i32, flags: i32, buff: &types::Wbytes) -> Result<(), Error> {
        let b = buff.ptr.as_array(buff.len);
        let mut b1 = b.as_slice_mut().unwrap().unwrap();

        log::debug!("UART read handle: {} flags: {}", handle, flags);

        Uart::read(self, handle, flags as u32, b1.deref_mut())
    }
}
