FROM rust:1.59 AS builder

WORKDIR /usr/src/rust-defenders
RUN apt-get update && apt-get install libasound2-dev libudev-dev -y
COPY . .
RUN cargo install --path .


FROM debian:buster-slim

RUN apt-get update && apt-get install libasound2-dev libudev-dev -y
COPY --from=builder /usr/local/cargo/bin/rust-defenders /usr/local/bin/rust-defenders
COPY --from=builder /usr/src/rust-defenders/config/config-example.txt config-example.txt

CMD ["rust-defenders"]
