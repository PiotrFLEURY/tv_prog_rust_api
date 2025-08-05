FROM rust:latest AS builder

WORKDIR /sources

COPY . .

RUN cargo build --release

FROM debian:latest

WORKDIR /app

COPY --from=builder /sources/target/release/tv_prog_rust_api /app/

# Install necessary dependencies
RUN apt-get update && \
    apt-get install -y libssl-dev ca-certificates && \
    rm -rf /var/lib/apt/lists/*

EXPOSE 3000

CMD ["./tv_prog_rust_api"]

