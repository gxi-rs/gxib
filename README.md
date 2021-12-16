[![Docker](https://github.com/gxi-rs/gxib/actions/workflows/docker-publish.yml/badge.svg)](https://github.com/gxi-rs/gxib/actions/workflows/docker-publish.yml)

# Gxib

**build tool for the gxi-rs project**

## Web

`gxib web` helps compile the project to ready to use `.wasm` files.

Inbuilt web server with hot reload for faster development.

```bash
gxib web -wrs localhost:8080
```

### Docker

contains all dependencies required for web builds

_mounts current dir to /app and exports port 8080_

```bash
docker run \
       -p 8080:8080 \
       -v $(pwd):/app \
       -it ghcr.io/gxi-rs/gxib:latest
```

### Run

```bash
cd /app
gxib web
```

### Dependencies

if you don't want to use the prebuilt docker image, the following dependencies need
to be present in your dev environment.

- install `gxib`

  ```bash
  cargo install gxib
  ```

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

## Roadmap

- [x] Basic desktop gtk builds
- [x] Basic wasm builds
- [ ] Desktop hot reload
- [ ] Web and Desktop hot refresh without losing state
