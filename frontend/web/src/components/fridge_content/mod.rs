use yew::prelude::*;
use reqwasm::http::Request;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct FridgeItem {
    number: u32,
    image_url: String,
    title: String,
}

#[function_component(FridgeContent)]
pub fn FridgeContent() -> Html {
    let items = use_state(|| Vec::<FridgeItem>::new());

    {
        let items = items.clone();
        use_effect_with_deps(move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                let resp = Request::get("http://localhost:8081/fridge-items")
                    .send().await.unwrap();
                let data: Vec<FridgeItem> = resp.json().await.unwrap();
                items.set(data);
            });
            || ()
        }, ());
    }

    html! {
        <div class="container">
            <h2>{"Dolapta Ne Var?"}</h2>
            <ul>
            {
                for items.iter().map(|item| {
                    html! {
                        <li>
                            <span class="number">{item.number}</span>
                            <img src={item.image_url.clone()} alt={item.title.clone()} />
                            <span class="title">{item.title.clone()}</span>
                        </li>
                    }
                })
            }
            </ul>
        </div>
    }
}
