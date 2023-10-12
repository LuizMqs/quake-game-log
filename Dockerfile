FROM rust:latest AS builder

WORKDIR /app

COPY . .

RUN cargo build --release

CMD ["cargo", "run", "--release"]
