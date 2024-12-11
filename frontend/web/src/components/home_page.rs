use yew::prelude::*;
use stylist::yew::styled_component;

#[styled_component]
#[function_component(HomePage)]
pub fn home_page() -> Html {
  html! {
    <div>
      <h1>{ "Sağlık Uygulamasına Hoş Geldiniz!asd" }</h1>
      <div class={css!("color: blue;")}>{"Hello blue!"}</div
    </div>
  }
}