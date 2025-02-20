# syntax=docker/dockerfile:1

################################################
# 1. AŞAMA: Derleme (builder)                 #
################################################
FROM rust:latest AS builder

WORKDIR /app

# Cargo.toml ve src içeriğini kopyala
COPY Cargo.toml .
COPY src ./src

# Bağımlılıkları (derleme öncesi) getirelim
RUN cargo fetch

# Bağımlılıkları indirmek ve derlemek için
RUN cargo build --release

################################################
# 2. AŞAMA: Minimal runtime imajı             #
################################################
FROM debian:bullseye-slim

WORKDIR /app

# Derlenmiş binary'yi builder'dan kopyala
COPY --from=builder /app/target/release/nutrition_service .

# Uygulamanın dinleyeceği port
EXPOSE 8080

# Container ayağa kalktığında çalışacak komut
CMD ["./nutrition_service"]
