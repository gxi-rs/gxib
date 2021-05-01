# Web Pipeline
*Builds wasm apps*

## Process

* `cargo build --target wasm32-unknown-unknown`
* move `target/wasm32-unknown-unknown/{debug/release}/<name>.wasm` to `dist`
* `wasm-bindgen <name>.wasm`

*If release*
* `wasm-opt -Oz <name>.wasm -o <name>.wasm`
  
* create `index.html` which loads `<name>.wasm`
 