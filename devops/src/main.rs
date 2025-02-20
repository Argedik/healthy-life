use actix_web::{web, App, HttpResponse, HttpServer, Responder};
// TLS tiplerini actix_tls'nin rustls_0_20 modülünden içe aktarıyoruz.
use actix_tls::accept::rustls_0_20::reexports::ServerConfig;
use rustls_pki_types::{CertificateDer, PrivateKeyDer};
use tokio_rustls::TlsAcceptor;
use rustls_pemfile::{certs, pkcs8_private_keys};
use std::{fs::File, io::BufReader, sync::Arc};

async fn index() -> impl Responder {
    HttpResponse::Ok().body("Güvenli HTTPS Sunucusu Çalışıyor!")
}

fn load_rustls_config() -> ServerConfig {
    // Sertifika ve private key dosyalarının yolları (Let’s Encrypt dosya yolları)
    let cert_file = &mut BufReader::new(
        File::open("/etc/letsencrypt/live/argedik.com/fullchain.pem")
            .expect("Sertifika dosyası bulunamadı"),
    );
    let key_file = &mut BufReader::new(
        File::open("/etc/letsencrypt/live/argedik.com/privkey.pem")
            .expect("Private key dosyası bulunamadı"),
    );

    // Sertifikaları oku; rustls-pemfile 1.0, dosyayı Vec<Vec<u8>> döner.
    let cert_chain = certs(cert_file)
        .expect("Sertifikalar okunamadı")
        .into_iter()
        .map(CertificateDer::from)
        .collect();

    // PKCS8 private key'leri oku:
    let mut keys = pkcs8_private_keys(key_file)
        .expect("Private key okunamadı");
    let key = PrivateKeyDer::from(keys.remove(0));

    // TLS yapılandırmasını oluştur:
    ServerConfig::builder()
        .with_safe_defaults()
        .with_no_client_auth()
        .with_single_cert(cert_chain, key)
        .expect("TLS yapılandırması oluşturulamadı")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // TLS yapılandırmasını yükle ve Arc içerisine al:
    let tls_config = Arc::new(load_rustls_config());
    let tls_acceptor = TlsAcceptor::new(tls_config);

    println!("HTTP (port 80) ve HTTPS (port 443) sunucuları başlatılıyor...");

    // HTTP sunucusu (plain HTTP) – port 80
    let server_http = HttpServer::new(|| {
        App::new().route("/", web::get().to(index))
    })
    .bind("0.0.0.0:80")?
    .run();

    // HTTPS sunucusu (TLS destekli) – port 443
    let server_https = HttpServer::new(|| {
        App::new().route("/", web::get().to(index))
    })
    .bind_rustls("0.0.0.0:443", tls_acceptor)?
    .run();

    futures::try_join!(server_http, server_https)?;
    Ok(())
}
