FROM rust:slim-buster as cargo-build

RUN apt-get update
RUN apt-get install musl-tools -y
RUN rustup target add x86_64-unknown-linux-musl
WORKDIR /app
COPY ./ .
RUN cargo build --target x86_64-unknown-linux-musl --release

FROM alpine:latest

WORKDIR /app
COPY --from=cargo-build /app/target/x86_64-unknown-linux-musl/release/realtime-shopping ./
EXPOSE 8080
CMD ["/app/realtime-shopping"]
