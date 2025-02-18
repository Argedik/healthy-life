use yew::prelude::*;
use serde::{Deserialize, Serialize};
use reqwasm::http::Request;
use wasm_bindgen_futures::spawn_local; // async/await kullanımı için
use gloo::console::{log, error};      // console.log / console.error

// Veri modeli
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct FridgeItem {
    pub id: Option<u32>,
    pub title: String,
    pub image_url: String,
}

#[function_component(FridgeCrud)]
pub fn fridge_crud_component() -> Html {
    // --- State tanımları ---
    let items = use_state(|| Vec::<FridgeItem>::new());
    let loaded = use_state(|| false);           // Veriler ilk kez yüklendi mi?
    let new_title = use_state(|| "".to_string());
    let new_image = use_state(|| "".to_string());
    let backend_url = "http://127.0.0.1:8080/fridge_items";

    // --- Bileşen ilk render olduğunda verileri çek ---
    {
        let items = items.clone();
        let loaded = loaded.clone();
        use_effect(move || {
            // Eğer daha önce yüklenmemişse sunucudan çek
            if !*loaded {
                spawn_local(async move {
                    match Request::get(backend_url).send().await {
                        Ok(resp) if resp.ok() => {
                            if let Ok(data) = resp.json::<Vec<FridgeItem>>().await {
                                items.set(data);
                            }
                            loaded.set(true);
                        }
                        Ok(_) => {
                            error!("Sunucudan geçerli yanıt alınamadı!");
                        }
                        Err(err) => {
                            error!(format!("Fetch error: {:?}", err));
                        }
                    }
                });
            }
            || () // cleanup
        });
    }

    // --- Ekle (POST) Butonu ---
    let on_add = {
        let items = items.clone();
        let title_state = new_title.clone();
        let image_state = new_image.clone();

        Callback::from(move |_| {
            let title = (*title_state).clone();
            let image_url = (*image_state).clone();

            if title.trim().is_empty() || image_url.trim().is_empty() {
                error!("Title ve Image URL boş olamaz!");
                return;
            }

            let new_item = FridgeItem {
                id: None,
                title,
                image_url,
            };
            let items = items.clone();

            spawn_local(async move {
                match Request::post(backend_url)
                    .header("Content-Type", "application/json")
                    .body(serde_json::to_string(&new_item).unwrap())
                    .send()
                    .await
                {
                    Ok(resp) if resp.ok() => {
                        if let Ok(created_item) = resp.json::<FridgeItem>().await {
                            let mut current = (*items).clone();
                            current.push(created_item);
                            items.set(current);
                            log!("Yeni kayıt eklendi!");
                        }
                    }
                    _ => {
                        error!("Ekleme işlemi başarısız!");
                    }
                }
            });

            // formu temizle
            title_state.set("".into());
            image_state.set("".into());
        })
    };

    // --- Sil Butonu ---
    let on_delete = {
        let items = items.clone();
        Callback::from(move |id: u32| {
            let items = items.clone();
            spawn_local(async move {
                let delete_url = format!("{}/{}", backend_url, id);
                match Request::delete(&delete_url).send().await {
                    Ok(resp) if resp.ok() => {
                        let mut current = (*items).clone();
                        current.retain(|x| x.id != Some(id));
                        items.set(current);
                        log!("Kayıt silindi!");
                    }
                    _ => {
                        error!("Silme işlemi başarısız!");
                    }
                }
            });
        })
    };

    // --- Input eventleri ---
    let on_title_input = {
        let title_state = new_title.clone();
        Callback::from(move |e: InputEvent| {
            if let Some(input) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                title_state.set(input.value());
            }
        })
    };
    let on_image_input = {
        let image_state = new_image.clone();
        Callback::from(move |e: InputEvent| {
            if let Some(input) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                image_state.set(input.value());
            }
        })
    };

    // --- HTML Arayüz ---
    // Ayrıca fridge_crud.html dosyasını istersen include_str! ile gömebilirsin.
    html! {
        <div class="fridge-crud-wrapper">
            <h2>{"Fridge CRUD (Yeni Klasör Örneği)"}</h2>

            // Ekleme Formu
            <div>
                <input
                    type="text"
                    placeholder="Title"
                    value={(*new_title).clone()}
                    oninput={on_title_input}
                />
                <input
                    type="text"
                    placeholder="Image URL"
                    value={(*new_image).clone()}
                    oninput={on_image_input}
                />
                <button onclick={on_add}>{"EKLE"}</button>
            </div>

            // Liste
            <hr/>
            {
                (*items).iter().map(|item| {
                    let delete_id = item.id.unwrap_or_default();
                    let image_url = item.image_url.clone();

                    html! {
                        <div class="fridge-item">
                            <div>{"ID: "} {delete_id} {" | "} { &item.title }</div>
                            <img class="item-image" src={image_url} alt="image" />
                            <button onclick={
                                let on_delete = on_delete.clone();
                                move |_| on_delete.emit(delete_id)
                            }>{"Sil"}</button>
                        </div>
                    }
                }).collect::<Html>()
            }
        </div>
    }
}
