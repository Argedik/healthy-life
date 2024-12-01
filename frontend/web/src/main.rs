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