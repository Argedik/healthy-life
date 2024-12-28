// use google_sheets4::{Sheets, yup_oauth2, hyper_rustls};
use yup_oauth2::{InstalledFlowAuthenticator, InstalledFlowReturnMethod, read_application_secret};
use std::path::PathBuf;
// use hyper_util::client::legacy::{connect::HttpConnector, Client};
// use axum::body::Body;
// use hyper_rustls::HttpsConnectorBuilder;


//-> google_sheets4::Result<Sheets<Client<hyper_rustls::HttpsConnector<HttpConnector>, Body>>>
pub async fn get_sheets_client() -> Result<(), Box<dyn std::error::Error>>{
    // Dokümanda client secret nasıl elde edileceği gösterilmiyor, "some means" diyor.
    // Biz burada JSON dosyasından okuyoruz.
    let secret_path = PathBuf::from("client_secret.apps.googleusercontent.com.json");
    println!("test {:?}", secret_path);

    let application_secret = read_application_secret(secret_path).await?;
    println!("application_secret: {:?}", application_secret);

    let auth = InstalledFlowAuthenticator::builder(application_secret, InstalledFlowReturnMethod::HTTPRedirect,).expect("failed to create authenticator");
    println!("auth: {}", auth);

    let https = HttpsConnectorBuilder::new();
    print!(https);

    // let auth = InstalledFlowAuthenticator::builder(
    //     application_secret,
    //     InstalledFlowReturnMethod::HTTPRedirect,
    // )
    // .build()
    // .await
    // .unwrap(); 

    // // HTTPS istemcisi oluşturuluyor
    // let https = HttpsConnectorBuilder::new()
    //     .with_native_roots()?      // Varsayılan işletim sistemi sertifikalarını kullan
    //     .https_or_http()           // HTTPS veya HTTP bağlantısına izin ver
    //     .enable_http1()            // HTTP1 kullanımını etkinleştir
    //     .build();

    // // let client = Client::builder().build(https);
    // let client = Client::builder(hyper_util::rt::TokioExecutor::new()).build(https);

    // // Google Sheets API nesnesi (hub) döndürüyoruz
    // let hub = Sheets::new(client, auth);
    Ok(())
}
