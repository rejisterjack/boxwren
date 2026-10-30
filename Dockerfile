# Reproducible run of `boxwren demo`. This image is not the product.
FROM rust:1.97-bookworm

RUN rustup target add wasm32-wasip1 wasm32-wasip2

WORKDIR /src
COPY . .
RUN cargo build -p boxwren-cli --release

CMD ["./target/release/boxwren", "demo"]
