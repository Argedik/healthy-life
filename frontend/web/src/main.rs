use yew::prelude::*;
mod components;
mod screens;
use crate::screens::FridgeCrud;

use components::{NavBar, HomePage, Card};
use screens::fridge_content::FridgeContent;

#[function_component(App)]
fn app() -> Html {
  html! {
    <>
      // <NavBar />
      // <HomePage />
      <FridgeCrud />
      // <FridgeContent />
      // <Card />
    </>
  }
}

fn main() {
  yew::Renderer::<App>::new().render();
}