FROM rust:latest

RUN rustup target add wasm32-unknown-unknown
RUN cargo install wasm-bindgen-cli

# install binaryen
RUN curl -L https://github.com/WebAssembly/binaryen/releases/download/version_104/binaryen-version_104-x86_64-linux.tar.gz > binaryen.tar.gz
RUN tar -zxvf binaryen.tar.gz binaryen-version_104
RUN cp ./binaryen-version_104/* /usr -r
RUN rm -rf binaryen.tar.gz binaryen-version_104

# install gxib at the end
RUN cargo install gxib
