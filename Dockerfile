FROM rust:1.67 as builder

WORKDIR /app

# Clone the repository
CMD git clone https://github.com/Toni-d-e-v/idena-indexer-rs .

# Rename the example environment file to .env
CMD mv .env.example .env

# Install diesel_cli
CMD cargo install diesel_cli --no-default-features --features "postgres"

# Run database migrations
CMD diesel migration run

# Build the project
CMD cargo build --release

# Run the application
CMD ["./target/release/idena-indexer-rs"]
