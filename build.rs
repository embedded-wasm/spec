#[cfg(feature="bind_c")]
use std::env;

#[cfg(feature="bind_c")]
use std::path::PathBuf;

fn main() {
    // Rerun on file changes
    println!("cargo:rerun-if-changed=./witx/*.witx");
    println!("cargo:rerun-if-changed=lib/*");
    println!("cargo:rerun-if-changed=build.rs");

    // Setup WASI root
    // https://github.com/bytecodealliance/wasmtime/issues/3519
    let dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    println!("cargo:rustc-env=WASI_ROOT={}/witx", dir);

    // Export spec dir for other projects
    println!("cargo:ROOT={}", dir);

    // Generate C bindings if enabled
    #[cfg(feature="bind_c")]
    {
        // Setup binding generation
        let mut builder = bindgen::Builder::default()
            .use_core()
            .ctypes_prefix("::cty")
            .header("inc/wasm_embedded/i2c.h")
            .header("inc/wasm_embedded/spi.h")
            .header("inc/wasm_embedded/uart.h")
            .header("inc/wasm_embedded/gpio.h")
            .allowlist_type("wasme.*")
            .allowlist_type("i2c.*")
            .allowlist_type("spi.*")
            .allowlist_type("uart.*")
            .allowlist_type("gpio.*");

        // Patches to help bindgen with cross compiling
        // See: https://github.com/rust-lang/rust-bindgen/issues/1229#issuecomment-366522257
        builder = match std::env::var("TARGET").as_deref() {
            Ok("armv7-unknown-linux-gnueabihf") => {
                println!("cargo:rustc-env=CC=arm-linux-gnueabihf-gcc");
                builder
                    .clang_arg("-target")
                    .clang_arg("arm-linux-gnueabihf")
                    .clang_arg("-I/usr/arm-linux-gnueabihf/include/")
            }
            Ok("aarch64-unknown-linux-gnu") => {
                println!("cargo:rustc-env=CC=aarch64-linux-gnu-gcc");
                builder
                    .clang_arg("-target")
                    .clang_arg("aarch64-linux-gnu")
                    .clang_arg("-I/usr/aarch64-linux-gnu/include/")
            }
            Ok("thumbv7em-none-eabihf") => {
                println!("cargo:rustc-env=CC=arm-none-eabi-gcc");
                builder
                    .use_core()
                    .clang_arg("-target")
                    .clang_arg("arm-none-eabihf")
                    // TODO: this seems... fragile
                    .clang_arg("-I/usr/lib/gcc/arm-none-eabi/8.3.1/include/")
            }
            _ => builder,
        };

        let bindings = builder.generate().expect("Unable to generate bindings");

        // Write the bindings to the $OUT_DIR/bindings.rs file.
        let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
        bindings
            .write_to_file(out_path.join("bindings.rs"))
            .expect("Couldn't write bindings!");
    }
}
