use super::*;

#[cfg(feature = "wiggle")]
use crate::api::{display, types};

 #[derive(Debug, Clone, PartialEq)]
 pub struct DisplayInfo {
     w: usize,
     h: usize,
     f: PixelFormat,
 }

 #[derive(Debug, Clone, PartialEq)]
 pub enum PixelFormat {
     Bw,
     Gs8,
     Rgb16,
 }

 /// Display context abstraction.
 /// This hides runtime implementation details to simplify implementing display contexts.
 /// Hopefully one day generation is improved so we don't _need_ this any more
 pub trait Display {
     fn init(&mut self, index: u32) -> Result<i32, Error>;

     fn deinit(&mut self, handle: i32) -> Result<(), Error>;

     fn set_pixel(&mut self, handle: i32, x: u32, y: u32, v: u32) -> Result<(), Error>;

     fn info(&mut self, handle: i32) -> Result<DisplayInfo, Error>;
 }

 /// Wrapper for wiggle-generated I2C api
 #[cfg(feature = "wiggle")]
 impl <T: Display> display::Display for T {
     /// Initialise the provided Display pin in input or output mode
     fn init(&mut self, index: u32) -> Result<i32, Error> {
         println!("Display init index: {}", index);
         todo!()
     }

     /// Deinitialise the specified Display pin
     fn deinit(&mut self, dev: i32) -> Result<(), Error> {
         println!("Display deinit handle: {}", dev);
         todo!()
     }

     fn set_pixel(&mut self, handle: i32, x: u32, y: u32, v: u32) -> Result<(), Error> {
         todo!()
     }

     fn info(&mut self, handle: i32) -> Result<types::DisplayInfo, Error> {
         todo!()
     }
 }

