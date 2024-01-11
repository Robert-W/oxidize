# Builder container
FROM rust:latest as builder

WORKDIR /app

# No clean way to currently cache dependencies
COPY Cargo.toml Cargo.lock .env /app/
COPY migrations /app/migrations
COPY src /app/src

RUN cargo build --release

# Release container
FROM gcr.io/distroless/cc-debian12

COPY --from=builder /app/target/release/oxidize /srv/oxidize

WORKDIR /srv

CMD ["./oxidize"]
