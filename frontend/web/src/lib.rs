use yew::prelude::*;
mod components;

#[function_component(App)]
fn app() -> Html {
  html! {
    <> 
      <components::NavBar />
      <components::HomePage />
    </>
  }
}

fn main (){
  yew::Renderer::<App>::new().render();
}