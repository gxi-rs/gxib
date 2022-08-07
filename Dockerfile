FROM rust:latest

RUN rustup target add wasm32-unknown-unknown
RUN cargo install wasm-bindgen-cli

# install binaryen
RUN curl -L https://github.com/WebAssembly/binaryen/releases/download/version_109/binaryen-version_109-x86_64-linux.tar.gz > binaryen.tar.gz
RUN tar -zxvf binaryen.tar.gz binaryen-version_109
RUN cp ./binaryen-version_109/* /usr -r
RUN rm -rf binaryen.tar.gz binaryen-version_109

# install gxib at the end
RUN cargo install gxib
