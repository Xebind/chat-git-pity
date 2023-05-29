# use the latest stable release
FROM rust:1.69.0 as build

# setup the app directory
WORKDIR /app

# install dependencies
RUN rustup target add wasm32-unknown-unknown
RUN cargo install  wasm-bindgen-cli

# TODO : check what locked does and if the bug on 28/5 "trunk cannot compile" was solved
RUN cargo install --locked trunk


# copy the source code
COPY ./frontend ./frontend
COPY ./shared ./shared

WORKDIR /app/frontend

RUN trunk build --release
RUN cargo build --release

EXPOSE 3000

CMD ["trunk", "serve", "--release", "--address", "0.0.0.0", "--port" , "3000"]

