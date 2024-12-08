use yew::prelude::*;
mod components;

#[function_component(App)]
fn app() -> Html {
  html! {
    <>
      <components::NavBar />
      <components::HomePage />
      <components::Card />
    </>
  }
}

fn main() {
  yew::Renderer::<App>::new().render();
}



// use yew::prelude::*;
// use serde::{Deserialize, Serialize};
// use reqwasm::http::Request;

// #[derive(Serialize, Deserialize, Clone, PartialEq)]
// struct Food {
//     id: i64,
//     name: String,
//     calories: i32,
// }

// #[function_component(App)]
// fn app() -> Html {
//     let foods = use_state(Vec::<Food>::new);

//     {
//         let foods = foods.clone();
//         use_effect_with_deps(
//             move |_| {
//                 wasm_bindgen_futures::spawn_local(async move {
//                     let fetched_foods = Request::get("http://localhost:8080/foods")
//                         .send()
//                         .await
//                         .unwrap()
//                         .json::<Vec<Food>>()
//                         .await
//                         .unwrap();
//                     foods.set(fetched_foods);
//                 });
//                 || ()
//             },
//             (),
//         );
//     }

//     let name = use_state(String::new);
//     let calories = use_state(String::new);

//     let on_add_food = {
//         let name = name.clone();
//         let calories = calories.clone();
//         let foods = foods.clone();
//         Callback::from(move |_| {
//             let name_value = name.to_string();
//             let calories_value: i32 = calories.to_string().parse().unwrap_or(0);

//             wasm_bindgen_futures::spawn_local(async move {
//                 let new_food = Food {
//                     id: 0,
//                     name: name_value.clone(),
//                     calories: calories_value,
//                 };

//                 Request::post("http://localhost:8080/foods")
//                     .header("Content-Type", "application/json")
//                     .body(serde_json::to_string(&new_food).unwrap())
//                     .send()
//                     .await
//                     .unwrap();

//                 let updated_foods = Request::get("http://localhost:8080/foods")
//                     .send()
//                     .await
//                     .unwrap()
//                     .json::<Vec<Food>>()
//                     .await
//                     .unwrap();
//                 foods.set(updated_foods);
//             });
//         })
//     };

//     html! {
//         <div>
//             <h1>{"Besin Uygulaması"}</h1>
//             <input placeholder="Besin Adı" oninput={Callback::from(move |e: InputData| name.set(e.value))} />
//             <input placeholder="Kalori" oninput={Callback::from(move |e: InputData| calories.set(e.value))} />
//             <button onclick={on_add_food}>{"Ekle"}</button>
//         </div>
//     }
// }
