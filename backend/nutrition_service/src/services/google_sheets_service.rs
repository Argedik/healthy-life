use serde_json::json;
use crate::models::FridgeItem;
use reqwest::Client;
use std::fs;
use google_service_auth::{ServiceAccountAuthenticator};

pub struct GoogleSheetsService {
    spreadsheet_id: String,
    client: Client,
    token: String,
}

impl GoogleSheetsService {
    pub async fn new(spreadsheet_id: &str) -> anyhow::Result<Self> {
        // credentials.json path
        let creds = fs::read_to_string("utils/credentials.json")?;
        let authenticator = ServiceAccountAuthenticator::from_json(&creds)?;
        let token = authenticator
            .authorize(&["https://www.googleapis.com/auth/spreadsheets"])
            .await?;

        Ok(Self {
            spreadsheet_id: spreadsheet_id.to_string(),
            client: Client::new(),
            token: token.to_string(),
        })
    }

    pub async fn get_fridge_items(&self) -> anyhow::Result<Vec<FridgeItem>> {
        let range = "Sheet1!A:C"; // A=number, B=image_url, C=title
        let url = format!(
            "https://sheets.googleapis.com/v4/spreadsheets/{}/values/{}",
            self.spreadsheet_id, range
        );

        let resp: serde_json::Value = self.client
            .get(&url)
            .bearer_auth(&self.token)
            .send()
            .await?
            .json()
            .await?;

        if let Some(values) = resp.get("values").and_then(|v| v.as_array()) {
            // İlk satır header varsayılırsa onu atla
            // ama biz direk ilk satırda veri varsa headersiz devam edebiliriz.
            // Burada varsayılan olarak ilk satırın da veri olduğunu düşünüyoruz.
            // Eğer ilk satır başlık ise values[1..] şeklinde iterate edebilirsiniz.

            let mut items = Vec::new();
            for row in values.iter().skip(1) {
                if let Some(arr) = row.as_array() {
                    let number = arr.get(0).and_then(|v| v.as_str()).unwrap_or("0").parse::<u32>().unwrap_or(0);
                    let image_url = arr.get(1).and_then(|v| v.as_str()).unwrap_or("").to_string();
                    let title = arr.get(2).and_then(|v| v.as_str()).unwrap_or("").to_string();

                    items.push(FridgeItem {
                        number,
                        image_url,
                        title,
                    });
                }
            }

            Ok(items)
        } else {
            Ok(vec![])
        }
    }

    // CRUD’un diğer işlemleri (POST, PUT, DELETE) de benzer şekilde yazılabilir
    // Şimdilik sadece GET örneği yeterli
}
