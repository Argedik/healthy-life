use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use serde::{Serialize, Deserialize};
use reqwasm::http::Request;
use web_sys::HtmlInputElement;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
struct FridgeItem {
    number: u32,
    image_url: String,
    title: String,
}

#[function_component(FridgeContent)]
pub fn fridge_content_view() -> Html {
    let spreadsheet_id = "YOUR_SPREADSHEET_ID_HERE"; // Aynı ID backend ile uyuşmalı
    let items = use_state(|| Vec::<FridgeItem>::new());
    let editing_item = use_state(|| None as Option<FridgeItem>);
    let new_number = use_state(|| 0u32);
    let new_image_url = use_state(|| "".to_string());
    let new_title = use_state(|| "".to_string());

    let fetch_items = {
        let items = items.clone();
        let spreadsheet_id = spreadsheet_id.to_string();
        Callback::from(move |_| {
            let items = items.clone();
            let url = format!("http://127.0.0.1:8081/fridge_items?spreadsheet_id={}", spreadsheet_id);
            spawn_local(async move {
                if let Ok(response) = Request::get(&url).send().await {
                    if response.ok() {
                        if let Ok(fetched_items) = response.json::<Vec<FridgeItem>>().await {
                            items.set(fetched_items);
                        }
                    }
                }
            });
        })
    };

    {
        let fetch_items = fetch_items.clone();
        use_effect(move || {
            fetch_items.emit(());
            || ()
        });
    }

    // Create (Add New Item)
    let on_add_click = {
        let spreadsheet_id = spreadsheet_id.to_string();
        let number = *new_number;
        let image_url = (*new_image_url).clone();
        let title = (*new_title).clone();
        let fetch_items = fetch_items.clone();
        Callback::from(move |_| {
            let fetch_items = fetch_items.clone();
            let url = format!("http://127.0.0.1:8081/fridge_items?spreadsheet_id={}", spreadsheet_id);
            let item = FridgeItem {
                number,
                image_url: image_url.clone(),
                title: title.clone(),
            };
            spawn_local(async move {
                let res = Request::post(&url)
                    .header("Content-Type","application/json")
                    .body(serde_json::to_string(&item).unwrap())
                    .send().await;
                if let Ok(r) = res {
                    if r.ok() {
                        fetch_items.emit(());
                    }
                }
            });
        })
    };

    // Delete
    let on_delete = {
        let fetch_items = fetch_items.clone();
        let spreadsheet_id = spreadsheet_id.to_string();
        Callback::from(move |row: u32| {
            let fetch_items = fetch_items.clone();
            let url = format!("http://127.0.0.1:8081/fridge_items/delete?spreadsheet_id={}&row={}", spreadsheet_id, row);
            spawn_local(async move {
                let res = Request::delete(&url).send().await;
                if let Ok(r) = res {
                    if r.ok() {
                        fetch_items.emit(());
                    }
                }
            });
        })
    };

    // Update
    let on_save_update = {
        let fetch_items = fetch_items.clone();
        let editing_item = editing_item.clone();
        let spreadsheet_id = spreadsheet_id.to_string();
        Callback::from(move |item: FridgeItem| {
            let fetch_items = fetch_items.clone();
            let editing_item = editing_item.clone();
            let url = format!("http://127.0.0.1:8081/fridge_items/update?spreadsheet_id={}&row={}", spreadsheet_id, item.number);
            spawn_local(async move {
                let res = Request::put(&url)
                    .header("Content-Type","application/json")
                    .body(serde_json::to_string(&item).unwrap())
                    .send().await;
                if let Ok(r) = res {
                    if r.ok() {
                        editing_item.set(None);
                        fetch_items.emit(());
                    }
                }
            });
        })
    };

    let on_edit = {
        let editing_item = editing_item.clone();
        Callback::from(move |item: FridgeItem| {
            editing_item.set(Some(item));
        })
    };

    let editing = (*editing_item).clone();

    let html_content = include_str!("fridge_content.html");

    let item_list = items.iter().map(|item| {
        html! {
            <div class="fridge-item">
                <div class="item-number">{ item.number }</div>
                <div class="item-image">
                    <img src={item.image_url.clone()} alt={item.title.clone()} />
                </div>
                <div class="item-title">{ &item.title }</div>
                <button onclick={on_edit.reform({
                    let item = item.clone();
                    move |_| item.clone()
                })}>{"Düzenle"}</button>
                <button onclick={on_delete.reform(move |_| item.number)}>{"Sil"}</button>
            </div>
        }
    }).collect::<Html>();

    // New item form değişiklikleri
    let on_number_input = {
        let new_number = new_number.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            if let Ok(val) = input.value().parse::<u32>() {
                new_number.set(val);
            }
        })
    };

    let on_image_input = {
        let new_image_url = new_image_url.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            new_image_url.set(input.value());
        })
    };

    let on_title_input = {
        let new_title = new_title.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            new_title.set(input.value());
        })
    };

    // Edit form inputları
    let (edit_number, edit_image_url, edit_title) = if let Some(editing) = editing.clone() {
        (editing.number.to_string(), editing.image_url, editing.title)
    } else {
        ("".to_string(), "".to_string(), "".to_string())
    };

    let edit_number_state = use_state(|| edit_number.clone());
    let edit_image_state = use_state(|| edit_image_url.clone());
    let edit_title_state = use_state(|| edit_title.clone());

    let on_edit_number_input = {
        let edit_number_state = edit_number_state.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            edit_number_state.set(input.value());
        })
    };

    let on_edit_image_input = {
        let edit_image_state = edit_image_state.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            edit_image_state.set(input.value());
        })
    };

    let on_edit_title_input = {
        let edit_title_state = edit_title_state.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            edit_title_state.set(input.value());
        })
    };

    let on_edit_save_click = {
        let on_save_update = on_save_update.clone();
        let editing_item = editing_item.clone();
        let edit_number_state = edit_number_state.clone();
        let edit_image_state = edit_image_state.clone();
        let edit_title_state = edit_title_state.clone();
        Callback::from(move |_| {
            if let Ok(num) = edit_number_state.parse::<u32>() {
                let updated_item = FridgeItem {
                    number: num,
                    image_url: (*edit_image_state).clone(),
                    title: (*edit_title_state).clone(),
                };
                on_save_update.emit(updated_item);
                editing_item.set(None);
            }
        })
    };

    let on_edit_cancel_click = {
        let editing_item = editing_item.clone();
        Callback::from(move |_| {
            editing_item.set(None);
        })
    };

    html! {
        <div>
            { Html::from_html_unchecked(html_content.into()) }
            <div class="fridge-content-list">
                {item_list}
            </div>
            <hr/>
            <h3>{"Yeni Öğe Ekle"}</h3>
            <label>{"Numara: "}<input type="number" value={(*new_number).to_string()} oninput={on_number_input}/></label>
            <label>{"Resim URL: "}<input type="text" value={(*new_image_url).clone()} oninput={on_image_input}/></label>
            <label>{"Başlık: "}<input type="text" value={(*new_title).clone()} oninput={on_title_input}/></label>
            <button onclick={on_add_click}>{"Ekle"}</button>

            {if editing.is_some() {
                html! {
                    <div class="edit-form">
                        <h3>{"Düzenle"}</h3>
                        <label>{"Numara: "}<input type="number" value={(*edit_number_state).clone()} oninput={on_edit_number_input}/></label>
                        <label>{"Resim URL: "}<input type="text" value={(*edit_image_state).clone()} oninput={on_edit_image_input}/></label>
                        <label>{"Başlık: "}<input type="text" value={(*edit_title_state).clone()} oninput={on_edit_title_input}/></label>
                        <button onclick={on_edit_save_click}>{"Kaydet"}</button>
                        <button onclick={on_edit_cancel_click}>{"İptal"}</button>
                    </div>
                }
            } else {
                html! {}
            }}

        </div>
    }
}
