FROM rust:1.78.0 AS builder
WORKDIR /usr/src/pokedex
COPY . .
RUN cargo install --path .

FROM ubuntu:24.04
ENV ROCKET_ADDRESS=0.0.0.0
RUN apt-get update && apt-get install -y openssl ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/pokedex /usr/local/bin/pokedex
CMD ["pokedex"]
EXPOSE 8000
