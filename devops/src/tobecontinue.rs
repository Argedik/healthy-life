use actix_web::{get, App, HttpRequest, HttpServer, Responder};
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};

#[get("/")]
async fn index(_req: HttpRequest) -> impl Responder {
    "Welcome to Secure Site!"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // OpenSSL için TLS yapılandırmasını oluştur
    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();

    // Özel anahtar (Let's Encrypt privkey.pem)
    builder
        .set_private_key_file("/etc/letsencrypt/live/argedik.com/privkey.pem", SslFiletype::PEM)
        .unwrap();

    // Sertifika (Let's Encrypt fullchain.pem)
    builder
        .set_certificate_chain_file("/etc/letsencrypt/live/argedik.com/fullchain.pem")
        .unwrap();

    // HTTPS Sunucusunu 443 portunda çalıştır
    HttpServer::new(|| App::new().service(index))
        .bind_openssl("0.0.0.0:443", builder)? // HTTPS portu
        .run()
        .await
}
