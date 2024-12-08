# Rust'ın en son resmi Docker imajını kullanıyoruz.
FROM rust:latest

# Çalışma dizini
WORKDIR /app

#  WebAssembly hedefini ekliyoruz.
RUN rustup target add wasm32-unknown-unknown

# Trunk'ı kuruyoruz.
RUN cargo install trunk cargo-watch

# Bağımlılıkları önbelleğe almak için Cargo.toml ve Cargo.lock dosyalarını kopyala
COPY Cargo.toml Cargo.lock ./

# Bağımlılıkları indirme
RUN mkdir src
RUN echo "fn main() {}" > src/main.rs

#Bağımlılıkları indiriyoruz.
RUN cargo fetch

# Tüm projeyş kopyala
COPY . .

# Port açıyoruz.
EXPOSE 8080

# Projeyi anlık olarak dinleyebilen kodlar
CMD ["cargo", "watch", "-w", "src", "-s", "trunk serve --address 0.0.0.0 --port 8080 --public-url /"]
