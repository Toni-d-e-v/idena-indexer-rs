FROM rust:1.67 as builder

WORKDIR /app

# Clone the repository
RUN git clone https://github.com/Toni-d-e-v/idena-indexer-rs .

# Rename the example environment file to .env
RUN mv .env.example .env

# Install diesel_cli
RUN cargo install diesel_cli --no-default-features --features "postgres"

# Run database migrations
RUN diesel migration run

# Build the project
RUN cargo build --release

# Run the application
CMD ["./target/release/idena-indexer-rs"]
