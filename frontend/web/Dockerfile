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

# Build komutumuz
# trunk build --release --public-url="./" 
# (İsterseniz trunk serve ile anlık sunum da yapabilirsiniz)
RUN trunk build --release

# 3001 portu (trunk serve içindir, ancak build alıp dosya sunmak istiyorsanız
# statik bir dosya sunucusuna ihtiyaç var. Ör: miniserve)
EXPOSE 3001

CMD ["trunk", "serve", "--release", "--port", "3001", "--address", "0.0.0.0"]