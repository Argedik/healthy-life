use yew::prelude::*;
use serde::{Deserialize, Serialize};
use reqwasm::http::Request;
use wasm_bindgen_futures::spawn_local; // Asenkron işlemler için
use gloo::console::{log, error};      // Tarayıcı konsoluna loglama ve hata yazdırma

// --- Veri Modeli ---
// Bu yapı, veritabanından gelen ve gönderilen "buzdolabı" kayıtlarını temsil eder.
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct FridgeItem {
    pub id: Option<u32>,  // Backend, yeni kayıt eklerken id'yi atayacak
    pub title: String,
    pub image_url: String,
}

#[function_component(FridgeCrud)]
pub fn fridge_crud_component() -> Html {
    // --- State Tanımları ---
    // items: Sunucudan çekilen tüm buzdolabı kayıtlarını tutar.
    let items = use_state(|| Vec::<FridgeItem>::new());
    // loaded: Verilerin ilk kez çekilip çekilmediğini kontrol eder.
    let loaded = use_state(|| false);
    // new_title ve new_image: Kullanıcıdan alınan yeni kayıt bilgilerini tutar.
    let new_title = use_state(|| "".to_string());
    let new_image = use_state(|| "".to_string());
    // Backend URL: CRUD işlemlerinde kullanılacak endpoint.
    let backend_url = "http://127.0.0.1:8080/fridge_items";

    // --- Bileşen İlk Render Olduğunda Verileri Çekme ---
    {
        let items = items.clone();
        let loaded = loaded.clone();
        use_effect(move || {
            // Eğer veriler daha önce yüklenmediyse, backend'e GET isteği gönder.
            if !*loaded {
                spawn_local(async move {
                    match Request::get(backend_url).send().await {
                        Ok(resp) if resp.ok() => {
                            // Gelen JSON veriyi FridgeItem vektörüne parse ediyoruz.
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
            || () // Cleanup fonksiyonu (gerekmiyorsa boş bırakıyoruz)
        });
    }

    // --- Yeni Kayıt Ekleme (POST) ---
    let on_add = {
        let items = items.clone();
        let title_state = new_title.clone();
        let image_state = new_image.clone();

        Callback::from(move |_| {
            // Inputlardan gelen değerleri alıyoruz.
            let title = (*title_state).clone();
            let image_url = (*image_state).clone();

            // Boş giriş kontrolü
            if title.trim().is_empty() || image_url.trim().is_empty() {
                error!("Title ve Image URL boş olamaz!");
                return;
            }

            // Yeni kayıt oluşturuyoruz (id: None, çünkü backend id atayacak)
            let new_item = FridgeItem {
                id: None,
                title,
                image_url,
            };
            let items = items.clone();

            // POST isteği gönderiyoruz.
            spawn_local(async move {
                match Request::post(backend_url)
                    .header("Content-Type", "application/json")
                    .body(serde_json::to_string(&new_item).unwrap())
                    .send()
                    .await
                {
                    Ok(resp) if resp.ok() => {
                        // Başarılı olursa, backend tarafından dönen yeni kaydı state'e ekle.
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

            // İşlem sonrası input alanlarını temizle.
            title_state.set("".into());
            image_state.set("".into());
        })
    };

    // --- Kayıt Silme (DELETE) ---
    let on_delete = {
        let items = items.clone();
        Callback::from(move |id: u32| {
            let items = items.clone();
            spawn_local(async move {
                // DELETE isteği için URL oluşturulur.
                let delete_url = format!("{}/{}", backend_url, id);
                match Request::delete(&delete_url).send().await {
                    Ok(resp) if resp.ok() => {
                        // Başarılı olursa, state'teki ilgili kaydı çıkar.
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

    // --- Input Event Handler'ları ---
    // Title input alanındaki değişiklikleri state'e aktarmak için.
    let on_title_input = {
        let title_state = new_title.clone();
        Callback::from(move |e: InputEvent| {
            if let Some(input) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                title_state.set(input.value());
            }
        })
    };
    // Image URL input alanındaki değişiklikleri state'e aktarmak için.
    let on_image_input = {
        let image_state = new_image.clone();
        Callback::from(move |e: InputEvent| {
            if let Some(input) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                image_state.set(input.value());
            }
        })
    };

    // --- HTML Arayüzü (Görsel Tasarım) ---
    html! {
        <div class="fridge-crud-wrapper">
            <h2>{"Fridge CRUD (Yeni Klasör Örneği)"}</h2>

            // Kayıt ekleme formu: Title ve Image URL giriş alanları ve ekleme butonu.
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

            // Ayırıcı çizgi
            <hr/>
            // Mevcut kayıtların listesi
            {
                (*items).iter().map(|item| {
                    // Her bir kayıt için; id, title ve resim gösterimi ile silme butonu.
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
