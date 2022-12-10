FROM rust:1.65 as builder

RUN USER=root cargo new --bin memoizer
WORKDIR /opt/memoizer
COPY ./src ./src
COPY ./Cargo.toml ./Cargo.toml

RUN cargo build --release
RUN rm -rf src

FROM debian:buster-slim
RUN apt-get update && apt-get install -y curl iputils-ping mc && rm -rf /var/lib/apt/lists/*
COPY --from=builder /opt/memoizer/target/release/memoizer /opt/memoizer/memoizer

EXPOSE 1901

ENV \
  MEMOIZER_HOST=127.0.0.1 \
  MEMOIZER_PORT=1901

WORKDIR /opt/memoizer

ENTRYPOINT ./memoizer -b $MEMOIZER_HOST -p $MEMOIZER_PORT