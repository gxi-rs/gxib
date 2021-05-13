# Gxib

*build tool for the gxi-rs project*

## Install

```bash
cargo install gxib
```

## Usage

Run in project root dir.

### Desktop

*Dev Environment Requirements*

* [Gtk 3](https://www.gtk.org/docs/installations/)

*Running* 
```bash
gxib desktop
```

### Web
*Dev Environment Requirements*

* install `wasm32-unknown-unknown` architecture
  ```bash
  rustup target add wasm32-unknown-unknown
  ```
* [Wasm Bindgen CLi](https://rustwasm.github.io/wasm-bindgen/reference/cli.html) 
  ```bash
  cargo install wasm-bindgen-cli
  ```  
* [Binaryen](https://github.com/WebAssembly/binaryen) 
  for reducing wasm bundle size with `wasm-opt -Oz`
  ```bash
  cargo install cargo-wasi
  ```

*Running*
```bash
gxib web
```

## Other Args

Run help to list other commands and args

```bash
gxib help
```

## Roadmap

* [X] Basic desktop gtk builds
* [X] Basic wasm builds
* [ ] Desktop hot reload
* [ ] Web and Desktop hot refresh without losing state
