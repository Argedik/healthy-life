mod google_sheets_client;
use google_sheets_client::get_sheets_client;

#[tokio::main]
async fn main() {
    if let Err(e) = get_sheets_client().await {
        eprintln!("Error: {}", e);
    }
}
