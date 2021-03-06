# Embedded WASM Specification

A set of [.witx](https://github.com/WebAssembly/WASI/blob/main/docs/witx.md) specifications for embedded device interfaces, with the goal of providing a common language and platform independent runtime to support cross device/platform/architecture embedded applications.

This repository provides the reference specification as well as language specific helpers/adaptors to assist with platform implementations.

For a more detailed overview you may like to check out the [book](https://ryan.kurte.nz/embedded-wasm).

## Status

[![ci](https://github.com/embedded-wasm/spec/actions/workflows/ci.yml/badge.svg)](https://github.com/embedded-wasm/spec/actions/workflows/ci.yml)
[![Crates.io](https://img.shields.io/crates/v/wasm-embedded-spec.svg)](https://crates.io/crates/wasm-embedded-spec)
[![Docs.rs](https://docs.rs/wasm-embedded-spec/badge.svg)](https://docs.rs/wasm-embedded-spec)


Extremely alpha. Currently working to refactor / split out components from a frightening monolith, plenty of exploration required yet and the `wasi` call convention and `witx` format are moving targets.


## Layout

- [./witx](./witx) contains the `.witx` API specifications
- [./src](./src) provides a rust library generated using these specs (including platform abstractions)
- [./lib](./lib) contains C headers for platform implementations
- [./tests](./tests) contains test definitions for implementation by platforms and HALs
  
