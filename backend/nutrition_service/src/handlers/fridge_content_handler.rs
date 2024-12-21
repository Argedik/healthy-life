use axum::response::Html;
use crate::services::google_sheets_service::get_fridge_items_from_sheet;

// Bu ID'yi kendinize göre güncelleyin. 
// Örneğin: "1AbCdEfG..." gibi Google Sheets Spreadsheet ID'nizi buraya koyun.
const SPREADSHEET_ID: &str = "YOUR_SPREADSHEET_ID_HERE";

pub async fn show_fridge_content() -> Html<String> {
    let items = match get_fridge_items_from_sheet(SPREADSHEET_ID).await {
        Ok(i) => i,
        Err(_) => vec![]
    };

    // Basit inline CSS ile tasarım.
    // Tasarımda solda numara, ortada image, sağda title gösteriyoruz.
    // Üstte başlık: "Dolapta ne var?"
    // items kadar satır üretiliyor.

    let mut html = String::new();
    html.push_str(r#"
<!DOCTYPE html>
<html lang="tr">
<head>
<meta charset="UTF-8">
<title>Dolapta Ne Var?</title>
<style>
body {
    font-family: Arial, sans-serif;
    background: #f5f4e9;
    margin: 0; padding: 0;
}
header {
    background: #8abf97;
    color: #fff;
    padding: 20px;
    text-align: center;
    font-size: 28px;
    font-weight: bold;
}
.container {
    margin: 20px;
}
.item {
    display: flex;
    align-items: center;
    background: #d2e3d3;
    margin-bottom: 10px;
    border-radius: 8px;
    padding: 10px;
}
.item-number {
    font-size: 24px;
    font-weight: bold;
    background: #8abf97;
    color: white;
    width: 40px;
    height: 40px;
    border-radius: 50%;
    display:flex;
    justify-content:center;
    align-items:center;
    margin-right: 20px;
}
.item-img img {
    max-height: 60px;
    max-width: 60px;
    margin-right: 20px;
    border-radius: 8px;
    background: #fff;
}
.item-title {
    font-size: 20px;
    flex:1;
}
</style>
</head>
<body>
<header>Dolapta Ne Var?</header>
<div class="container">
"#);

    for item in items {
        html.push_str(&format!(r#"
<div class="item">
    <div class="item-number">{}</div>
    <div class="item-img"><img src="{}" alt="{}"></div>
    <div class="item-title">{}</div>
</div>
"#, item.number, item.image_url, item.title, item.title));
    }

    html.push_str(r#"
</div>
</body>
</html>
"#);

    Html(html)
}
