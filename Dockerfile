FROM rust:latest

RUN rustup target add wasm32-unknown-unknown
RUN cargo install wasm-bindgen-cli

# install binaryen
RUN curl -L https://github.com/WebAssembly/binaryen/releases/download/version_103/binaryen-version_103-x86_64-linux.tar.gz > binaryen.tar.gz
RUN tar -zxvf binaryen.tar.gz binaryen-
RUN cp ./binaryen-/* /usr -r
RUN rm -rf binaryen.tar.gz binaryen- 

# install gxib at the end
RUN cargo install gxib
