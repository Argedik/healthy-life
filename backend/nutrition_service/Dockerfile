FROM rust:latest

WORKDIR /app
COPY . .

RUN cargo build --release

CMD ["./target/release/nutrition_service"]