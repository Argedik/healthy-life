use google_sheets4::api::ValueRange;
use google_sheets4::Result;
use crate::google_sheets_client::get_sheets_client;
use crate::models::fridge_item::FridgeItem;
use serde_json::Value;

pub async fn get_fridge_items_from_sheet(spreadsheet_id: &str) -> Result<Vec<FridgeItem>> {
    let hub = get_sheets_client().await?;
    let range = "Sayfa1!A2:C";
    let (_resp, value_range) = hub.spreadsheets().values_get(spreadsheet_id, range).doit().await?;

    let values = match value_range.values {
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
        items.push(FridgeItem { number, image_url, title });
    }

    Ok(items)
}

pub async fn add_fridge_item(spreadsheet_id: &str, item: FridgeItem) -> Result<()> {
    let hub = get_sheets_client().await?;
    let range = "Sayfa1!A2:C";

    let values = vec![
        vec![
            Value::String(item.number.to_string()),
            Value::String(item.image_url),
            Value::String(item.title),
        ]
    ];
    
    let value_range = ValueRange {
        values: Some(values),
        ..ValueRange::default()
    };

    let (_resp, _append_result) = hub.spreadsheets().values_append(value_range, spreadsheet_id, range)
        .value_input_option("RAW")
        .doit().await?;

    Ok(())
}

pub async fn update_fridge_item(spreadsheet_id: &str, row: u32, item: FridgeItem) -> Result<()> {
    let hub = get_sheets_client().await?;
    let range = format!("Sayfa1!A{}:C{}", row + 1, row + 1);

    let values = vec![
        vec![
            Value::String(item.number.to_string()),
            Value::String(item.image_url),
            Value::String(item.title),
        ]
    ];

    let value_range = ValueRange {
        values: Some(values),
        ..ValueRange::default()
    };

    let (_resp, _update_result) = hub.spreadsheets().values_update(value_range, spreadsheet_id, &range)
        .value_input_option("RAW")
        .doit().await?;

    Ok(())
}

pub async fn clear_fridge_item(spreadsheet_id: &str, row: u32) -> Result<()> {
    let hub = get_sheets_client().await?;
    let range = format!("Sayfa1!A{}:C{}", row + 1, row + 1);
    let (_resp, _clear_result) = hub.spreadsheets().values_clear(Default::default(), spreadsheet_id, &range)
        .doit().await?;

    Ok(())
}
