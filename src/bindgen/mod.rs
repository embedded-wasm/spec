//! Bindgen based C API definitions

#![allow(
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals,
    clippy::all
)]

pub use cty::c_char;

// Include generated bindings if enabled
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
