use yew::prelude::*;
mod components;
mod screens;

use components::{NavBar, HomePage, Card};
use screens::fridge_content::FridgeContent;

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