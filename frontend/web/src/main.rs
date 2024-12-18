use yew::prelude::*;
mod components;

use components::{NavBar, HomePage, Card};
use components::fridge_content::FridgeContent;

#[function_component(App)]
fn app() -> Html {
  html! {
    <>
      // <NavBar />
      // <HomePage />
      <FridgeContent />
      // <Card />
    </>
  }
}

fn main() {
  yew::Renderer::<App>::new().render();
}