use anyhow::Result;
use crate::google_sheets_client::get_sheets_client;
use crate::models::fridge_item::FridgeItem;
use google_sheets4::api::ValueRange;

// Verileri sheetten çek
pub async fn get_fridge_items_from_sheet(spreadsheet_id: &str) -> Result<Vec<FridgeItem>> {
    let hub = get_sheets_client().await?;
    let range = "Sheet1!A2:C";

    let (_, resp) = hub.spreadsheets().values_get(spreadsheet_id, range)
        .doit().await?;

    let values = match resp.values {
        Some(vals) => vals,
        None => return Ok(vec![]),
    };

    let mut items = Vec::new();
    for row in values {
        if row.len() < 3 {
            continue;
        }
        let number = row[0].parse::<u32>().unwrap_or(0);
        let image_url = row[1].clone();
        let title = row[2].clone();
        items.push(FridgeItem {
            number,
            image_url,
            title,
        });
    }

    Ok(items)
}

// Yeni bir satır ekle (Create)
pub async fn add_fridge_item(spreadsheet_id: &str, item: FridgeItem) -> Result<()> {
    let hub = get_sheets_client().await?;
    let range = "Sheet1!A2:C";

    let values = vec![
        vec![item.number.to_string(), item.image_url, item.title]
    ];

    let value_range = ValueRange {
        values: Some(values),
        ..ValueRange::default()
    };

    hub.spreadsheets().values_append(value_range, spreadsheet_id, range)
        .value_input_option("RAW")
        .doit().await?;

    Ok(())
}

// Var olan bir satırı güncelle (Update)
// Bu örnekte satır numarasını biliyor olmamız gerekir. Mesela Sheet1!A3:C3 aralığını güncelliyoruz.
pub async fn update_fridge_item(spreadsheet_id: &str, row: u32, item: FridgeItem) -> Result<()> {
    let hub = get_sheets_client().await?;
    let range = format!("Sheet1!A{}:C{}", row, row);

    let values = vec![
        vec![item.number.to_string(), item.image_url, item.title]
    ];

    let value_range = ValueRange {
        values: Some(values),
        ..ValueRange::default()
    };

    hub.spreadsheets().values_update(value_range, spreadsheet_id, &range)
        .value_input_option("RAW")
        .doit().await?;

    Ok(())
}

// Bir satırı silmek yerine clear ediyoruz (Delete benzeri).
pub async fn clear_fridge_item(spreadsheet_id: &str, row: u32) -> Result<()> {
    let hub = get_sheets_client().await?;
    let range = format!("Sheet1!A{}:C{}", row, row);

    hub.spreadsheets().values_clear(Default::default(), spreadsheet_id, &range)
        .doit().await?;

    Ok(())
}
