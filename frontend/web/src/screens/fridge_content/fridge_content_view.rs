use yew::prelude::*;

#[function_component(FridgeContent)]
pub fn fridge_content_component() -> Html {
    let html_content = include_str!("fridge_content.html"); 
    html! {
        <div>
            { Html::from_html_unchecked(html_content.into()) }
        </div>
    }
}
