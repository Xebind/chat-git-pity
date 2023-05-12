# use the latest stable release
FROM rust:1.69.0 as build

# setup the app directory
WORKDIR /app/frontend

# install dependencies
RUN rustup target add wasm32-unknown-unknown
RUN cargo install trunk wasm-bindgen-cli

COPY . .

RUN  trunk build --release
RUN  cargo build --release

EXPOSE 8080

CMD ["trunk", "serve", "--release", "--address", "0.0.0.0", "--port" , "8080"]

