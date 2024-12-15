use yew::prelude::*;
  // <div class={css!("color: blue;")}>{"Hello blue!"}</div

#[function_component(HomePage)]
pub fn home_page_component() -> Html {
  let html_content = include_str!("home_page.html"); 

  html! {
      <div>
          { Html::from_html_unchecked(html_content.into()) }
      </div>
  }
}