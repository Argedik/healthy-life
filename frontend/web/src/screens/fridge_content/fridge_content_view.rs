// src/screens/fridge_content/fridge_content_view.rs

use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use serde::Deserialize;
use reqwasm::http::Request;

#[derive(Debug, Clone, PartialEq, Deserialize)]
struct FridgeItem {
    number: u32,
    image_url: String,
    title: String,
}

#[function_component(FridgeContent)]
pub fn fridge_content_view() -> Html {
    let spreadsheet_id = "YOUR_SPREADSHEET_ID";
    let items = use_state(|| Vec::<FridgeItem>::new());
    let items_clone = items.clone();
    let spreadsheet_id_clone = spreadsheet_id.to_string();

    // Eğer etkiler component mount edilirken 1 defa çalışsın istiyorsanız:
    use_effect(move || {
        spawn_local(async move {
            let url = format!("/fridge_items?spreadsheet_id={}", spreadsheet_id_clone);
            if let Ok(response) = Request::get(&url).send().await {
                if response.ok() {
                    if let Ok(fetched_items) = response.json::<Vec<FridgeItem>>().await {
                        items_clone.set(fetched_items);
                    }
                }
            }
        });
        || ()
    });

    let html_content = include_str!("fridge_content.html");
    let rendered_list = items.iter().map(|item| {
        html! {
            <div class="fridge-item">
                <div class="item-number">{ item.number }</div>
                <div class="item-image">
                    <img src={item.image_url.clone()} alt={item.title.clone()} />
                </div>
                <div class="item-title">{ &item.title }</div>
            </div>
        }
    }).collect::<Html>();

    html! {
        <div>
            { Html::from_html_unchecked(html_content.into()) }
            <div class="fridge-content-list">
                {rendered_list}
            </div>
        </div>
    }
}
