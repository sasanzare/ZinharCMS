FROM rust:1.87-bookworm

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY migrations ./migrations
COPY src ./src

RUN cargo build

EXPOSE 8080
CMD ["cargo", "run"]
