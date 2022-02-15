# Rune WIT Files

This repository contains the various WebAssembly Interface Types files used by
the Rune project.

## Getting Started

The underlying idea behind WIT is that an API author will use a `*.wit` file to
define the interface for how a WebAssembly module will interact with the outside
world. Anyone wanting to comply with that interface can take a copy of the
`*.wit` file and run it through a code generator like `wit-bindgen` to generate
the appropriate language bindings.

If you have used Protocol Buffers before, `*.wit` is roughly analogous to a
`*.proto` file.

The `wit-bindgen` CLI tool can be installed [directly from GitHub][wit].

```console
$ cargo install --git https://github.com/bytecodealliance/wit-bindgen wit-bindgen-cli
```

Once `wit-bindgen` is installed, you will need to add this repository as a git
submodule.

```console
$ git submodule add git@github.com:hotg-ai/wit-files.git
```

### Rust Guest

If you are writing a Rust crate that will be compiled to WebAssembly, you can
have the bindings automatically generated by a procedural macro by adding the
`wit-bindgen-rust` crate as a dependency.

```console
$ cargo add --git https://github.com/bytecodealliance/wit-bindgen wit-bindgen-rust
```

Inside your crate, you can now use the `wit_bindgen_rust::import!()` macro to
generate bindings to functions provided by the host.

The `wit_bindgen_rust::export!()` macro will generate a trait that is used
to expose your crate's functionality to the outside world. You will need to
provide an implementation of this trait.

```rs
// lib.rs

wit_bindgen_rust::import!("../wit-files/rune/runtime-v1.wit");

wit_bindgen_rust::export!("../wit-files/rune/rune-v1.wit");

pub struct Exports;

impl rune_v1::RuneV1 for Exports { ... }
```

### JavaScript Host

JavaScript doesn't use macros, so the code for loading a WebAssembly needs to
be generated using `wit-bindgen`.

Using the `rune` example from before, our JavaScript host provides the
`runtime-v1.wit` interface and consumes the `rune-v1.wit` interface exposed by
the WebAssembly module.

```console
$ wit-bindgen js --export rune/runtime-v1.wit --import rune/rune-v1.wit --out-dir rune/js-host/
Generating "rune/js-host/intrinsics.js"
Generating "rune/js-host/rune-v1.d.ts"
Generating "rune/js-host/rune-v1.js"
Generating "rune/js-host/runtime-v1.d.ts"
Generating "rune/js-host/runtime-v1.js"
```

These files can be packaged and published to NPM as-is, although you might want
to create an `index.js` that re-exports everything from a single file.

To see how the generated bindings would be used in practice, check out the
script under `example/index.ts`.

It can be executed using the following:

```console
$ cd examples
$ yarn
$ node --loader ts-node/esm ./index.ts ~/Documents/hotg-ai/proc-blocks/argmax.wasm
[Registered Metadata] {
  name: 'argmax',
  version: '0.11.2',
  description: '',
  repository: '',
  tags: [],
  arguments: [],
  inputs: [ { name: 'input', description: null } ],
  outputs: [
    {
      name: 'max',
      description: 'The index of the element with the highest value'
    }
  ]
}
```

(note: this assumes you have already compiled the arg max proc-block to
WebAssembly)

[wit]: https://github.com/bytecodealliance/wit-bindgen
