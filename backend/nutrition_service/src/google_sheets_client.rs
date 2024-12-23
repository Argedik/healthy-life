use google_sheets4::{Sheets, yup_oauth2, hyper_rustls};
use yup_oauth2::{InstalledFlowAuthenticator, InstalledFlowReturnMethod, read_application_secret};
use std::path::PathBuf;
use hyper_util::client::legacy::{connect::HttpConnector, Client};
use axum::body::Body;

pub async fn get_sheets_client() -> google_sheets4::Result<Sheets<Client<hyper_rustls::HttpsConnector<HttpConnector>, Body>>> {
    // Dokümanda client secret nasıl elde edileceği gösterilmiyor, "some means" diyor.
    // Biz burada JSON dosyasından okuyoruz.
    let secret_path = PathBuf::from("client_secret.apps.googleusercontent.com.json");
    let application_secret = read_application_secret(secret_path).await?;

    let auth = InstalledFlowAuthenticator::builder(
        application_secret,
        InstalledFlowReturnMethod::HTTPRedirect,
    )
    .build()
    .await
    .unwrap(); 

    // HTTPS istemcisi oluşturuluyor
    let https = hyper_rustls::HttpsConnectorBuilder::new()
        .with_native_roots()?       // Varsayılan işletim sistemi sertifikalarını kullan
        .https_or_http()           // HTTPS veya HTTP bağlantısına izin ver
        .enable_http1()            // HTTP1 kullanımını etkinleştir
        .build();

    // let client = Client::builder().build(https);
    let client = Client::builder(hyper_util::rt::TokioExecutor::new()).build(https);

    // Google Sheets API nesnesi (hub) döndürüyoruz
    let hub = Sheets::new(client, auth);
    Ok(hub)
}
