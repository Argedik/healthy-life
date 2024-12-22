use google_sheets4::{Sheets, yup_oauth2, hyper, hyper_rustls, hyper_util};
use yup_oauth2::{InstalledFlowAuthenticator, InstalledFlowReturnMethod, read_application_secret};
use std::path::PathBuf;
use hyper::Body;

pub async fn get_sheets_client() -> google_sheets4::Result<Sheets<hyper::Client<hyper_rustls::HttpsConnector<hyper::client::connect::HttpConnector>, Body>>> {
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
    .unwrap();  // Dokümanda unwrap kullanılmış. Siz ? ile de hata döndürebilirsiniz.

    let https = hyper_rustls::HttpsConnectorBuilder::new()
    .with_native_roots()       // Sertifikaları native kök sertifikalar ile doğrula
    .https_or_http()           // HTTPS veya HTTP bağlantısına izin ver
    .enable_http1()            // HTTP1 kullanımını etkinleştir
    .build();

    let client = Client::builder().build(https);

    let hub = Sheets::new(client, auth);
    Ok(hub)
}
