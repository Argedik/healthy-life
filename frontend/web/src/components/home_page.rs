use yew::prelude::*;
  // <div class={css!("color: blue;")}>{"Hello blue!"}</div

#[function_component(HomePage)]
pub fn home_page() -> Html {
  html! {
    <div>
      <h1>{ "Sağlık Uygulamasına Hoş Geldiniz!" }</h1>
      <div>{"Hello blue!"}</div>
    </div>
  }
}