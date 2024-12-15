use yew::prelude::*;
mod components;

use components::{NavBar, HomePage, Card};

#[function_component(App)]
fn app() -> Html {
  html! {
    <>
      // <NavBar />
      <HomePage />
      // <Card />
    </>
  }
}

fn main() {
  yew::Renderer::<App>::new().render();
}