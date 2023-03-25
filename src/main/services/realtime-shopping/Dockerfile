FROM lukemathwalker/cargo-chef:latest-rust-slim-buster AS chef
RUN apt-get update
RUN apt-get install musl-tools -y
RUN rustup target add x86_64-unknown-linux-musl
WORKDIR /app

FROM chef AS planner 
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS cacher
COPY --from=planner /app/recipe.json recipe.json
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/app/target \
    cargo chef cook --release --target x86_64-unknown-linux-musl --recipe-path recipe.json

FROM chef AS builder
COPY ./ . 
COPY --from=cacher /app/target target
COPY --from=cacher $CARGO_HOME $CARGO_HOME
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    cargo build --target x86_64-unknown-linux-musl --release

FROM alpine:latest AS runtime
WORKDIR /app
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/realtime-shopping ./
EXPOSE 8080
CMD ["/app/realtime-shopping"]
