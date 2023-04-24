# use the latest stable release
FROM rust:1.69.0

# setup the app directory
WORKDIR /app

# install dependencies
RUN apt update && apt install lld clang -y

# install protobuf
RUN curl -LO https://github.com/protocolbuffers/protobuf/releases/download/v3.15.8/protoc-3.15.8-linux-x86_64.zip
RUN unzip protoc-3.15.8-linux-x86_64.zip -d /usr/local
RUN export PATH="$PATH:/usr/local/bin"

# copy the source code
COPY . .

# build the app
RUN cargo build --release

# expose the port
EXPOSE 8080
EXPOSE 9090

# run the app
CMD ["./target/release/chatgipity-backend", "--help"]
