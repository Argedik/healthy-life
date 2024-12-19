use anyhow::Result;
use google_sheets4::oauth2::{InstalledFlowAuthenticator, InstalledFlowReturnMethod};
use google_sheets4::Sheets;
use std::path::PathBuf;
use tokio::sync::OnceCell;

static SHEETS_CLIENT: OnceCell<Sheets> = OnceCell::const_new();

pub async fn get_sheets_client() -> Result<&'static Sheets> {
    SHEETS_CLIENT
        .get_or_try_init(|| async {
            // Kimlik doğrulama dosyamızın yolunu belirtelim
            let secret_path = PathBuf::from("./client_secret.apps.googleusercontent.com.json");
            let auth = InstalledFlowAuthenticator::builder(
                oauth2::read_application_secret(secret_path).await?,
                InstalledFlowReturnMethod::HTTPRedirect,
            )
            .persist_tokens_to_disk("./tokencache.json")
            .build()
            .await?;

            let hub = Sheets::new(
                hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()),
                auth
            );

            Ok(hub)
        })
        .await
}
