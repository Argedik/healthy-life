FROM rust:latest AS builder

WORKDIR /app
COPY . .

RUN rustup target add wasm32-unknown-unknown
RUN cargo install wasm-pack
RUN wasm-pack build --target web

FROM nginx:alpine
COPY --from=builder /app/pkg /usr/share/nginx/html/pkg
COPY ./static /usr/share/nginx/html

EXPOSE 80

CMD ["nginx", "-g", "daemon off;"]