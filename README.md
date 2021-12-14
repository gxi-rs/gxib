# Gxib

**build tool for the gxi-rs project**

## Install

```bash
cargo install gxib
```

## Usage

Run in project root dir.

## Web

### Requirements

- install `wasm32-unknown-unknown` architecture

  ```bash
  rustup target add wasm32-unknown-unknown
  ```

- [Wasm Bindgen CLi](https://rustwasm.github.io/wasm-bindgen/reference/cli.html)

  ```bash
  cargo install wasm-bindgen-cli
  ```

- [Binaryen](https://www.google.com/search?q=install+binaryen)
  for reducing wasm bundle size with `wasm-opt -Oz`

## Run

```bash
gxib web
```

## Roadmap

- [x] Basic desktop gtk builds
- [x] Basic wasm builds
- [ ] Desktop hot reload
- [ ] Web and Desktop hot refresh without losing state
