//! Embedded WASM Interface Specifications.
//!
//! This crate provides the WITX interface specification for embedded wasm platforms and HALs,
//! with bindings generated using wiggle for rust, and bindgen for C.
//!
//! https://github.com/embedded-wasm/spec

#![cfg_attr(not(feature = "std"), no_std)]

use encdec::{Encode, Decode, DecodeOwned};
use sha2::{Digest, Sha512};


pub mod api;

pub mod gpio;
pub mod i2c;
pub mod spi;
pub mod uart;

/// Embedded WASM Error type
///
/// Note these values _must_ correspond to WASM/WITX spec
#[derive(Clone, PartialEq, Debug)]
pub enum Error {
    InvalidArg,
    Unexpected,
    Failed,
    NoDevice,
    Unsupported,
}

#[cfg(feature="wiggle")]
impl From<Error> for crate::api::types::Errno {
    fn from(e: Error) -> crate::api::types::Errno {
        use crate::api::types::Errno;

        match e {
            Error::InvalidArg => Errno::InvalidArg,
            Error::Unexpected => Errno::Unexpected,
            Error::Failed => Errno::Failed,
            Error::NoDevice => Errno::NoDevice,
            Error::Unsupported => Errno::Unsupported,
        }

    }
}


/// Applet manifest, links app and manifest checksums with overall applet signature
#[derive(Clone, Debug, PartialEq, Encode, DecodeOwned)]
pub struct Manifest {
    /// Manifest version (must be 1)
    pub version: u16,
    /// Manifest flags
    pub flags: u16,

    /// Application binary length
    pub app_len: u32,
    /// Application binary checksum (sha512)
    pub app_csum: [u8; 32],

    /// Metadata binary length
    pub meta_len: u32,
    /// Metadata binary checksum
    pub meta_csum: [u8; 32],

    /// Manifest signing public key
    pub key: [u8; 32],

    /// Signature over manifest data
    pub sig: [u8; 32],
}

const MANIFEST_VERSION: u16 = 0x0001;

impl Manifest {

    /// Create a new [ManifestBuilder] instance
    pub fn new() -> ManifestBuilder {
        ManifestBuilder{ m: Manifest { 
            version: MANIFEST_VERSION,
            flags: 0,
            app_len: 0,
            app_csum: [0u8; 32],
            meta_len: 0,
            meta_csum: [0u8; 32],
            key: [0u8; 32],
            sig: [0u8; 32],
        }}
    }
}

/// Helper for constructing binary [Manifest] objects
#[derive(Clone, PartialEq, Debug)]
pub struct ManifestBuilder {
    m: Manifest,
}

impl ManifestBuilder {
    /// Set manifest flags
    pub fn flags(&mut self, flags: u16) -> &mut Self {
        self.m.flags = flags;

        self
    }

    /// Add app binary to manifest as bytes
    pub fn app_bin(&mut self, d: &[u8]) -> &mut Self {
        let mut h = Sha512::new();
        h.update(d);
        let h = h.finalize();

        self.m.app_len = d.len() as u32;
        self.m.app_csum.copy_from_slice(&h);

        self
    }

    /// Add app binary to manifest via file
    pub fn app_file(&mut self, f: &str) -> Result<&mut Self, std::io::Error> {
        let d = std::fs::read(f)?;
        Ok(self.app_bin(&d))
    }

    /// Add metadata binary to manifest as bytes
    pub fn meta_bin(&mut self, d: &[u8]) -> &mut Self {
        let mut h = Sha512::new();
        h.update(d);
        let h = h.finalize();

        self.m.meta_len = d.len() as u32;
        self.m.meta_csum.copy_from_slice(&h);

        self
    }

    /// Add metadata binary to manifest via file
    pub fn meta_file(&mut self, f: &str) -> Result<&mut Self, std::io::Error> {
        let d = std::fs::read(f)?;
        Ok(self.meta_bin(&d))
    }

    /// Complete manifest construction
    pub fn build(self) -> Result<Manifest, ()> {
        
        // TODO: ensure validity?

        // TODO: sign?

        Ok(self.m)
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum ManifestError {
    MissingAppChecksum,
    MissingMetaInfo,
}

