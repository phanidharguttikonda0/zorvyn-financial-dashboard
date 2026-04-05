# BUILDER STAGE
FROM rust:slim-bookworm AS builder

WORKDIR /usr/src/app

COPY Cargo.toml Cargo.lock ./
COPY src src
COPY migrations migrations
COPY public public

# Execute release build
RUN cargo build --release

# RUNTIME STAGE
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

WORKDIR /usr/src/app

# Pull binary safely out of builder container
COPY --from=builder /usr/src/app/target/release/zorvyn-finance-tracker /usr/local/bin/zorvyn-finance-tracker

EXPOSE 7878

CMD ["zorvyn-finance-tracker"]
