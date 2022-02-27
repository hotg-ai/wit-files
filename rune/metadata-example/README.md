# Metadata Example

An example WebAssembly module that shows off how to register node metadata with
the Rune runtime.

## Getting Started

To build this, you first need to configure your Rust toolchain to cross-compile
to WebAssembly.

```console
$ rustup target add wasm32-unknown-unknown
```

Next, you will need to compile this crate as a WebAssembly module.

```console
$ cargo build --target wasm32-unknown-unknown
```

The WebAssembly module will then be available under the `target/` directory at
the repository's root.

```console
$ ls ../../target/wasm32-unknown-unknown/debug -l
drwxr-xr-x    - michael 27 Feb 21:14 build
drwxr-xr-x    - michael 27 Feb 21:14 deps
drwxr-xr-x    - michael 27 Feb 21:14 examples
drwxr-xr-x    - michael 27 Feb 21:14 incremental
.rw-r--r--  352 michael 27 Feb 21:14 libmetadata_example.d
.rw-r--r-- 130k michael 27 Feb 21:14 libmetadata_example.rlib
.rw-r--r--  349 michael 27 Feb 21:14 metadata_example.d
.rwxr-xr-x 1.7M michael 27 Feb 21:14 metadata_example.wasm
```

The easiest way to find out what host functions your WebAssembly module can
access is by viewing the API docs.

```console
$ cargo doc --document-private-items --open
 ...
 Documenting wit-bindgen-rust v0.1.0 (https://github.com/hotg-ai/wit-bindgen?branch=troubleshoot-bad-paths#709eea1d)
 Documenting metadata-example v0.1.0 (/home/consulting/Documents/hotg-ai/wit-files/rune/metadata-example)
    Finished dev [unoptimized + debuginfo] target(s) in 4.30s
     Opening /home/consulting/Documents/hotg-ai/wit-files/target/doc/metadata_example/index.html
```
