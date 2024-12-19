use yew::prelude::*;

#[function_component(healthy-diets)]
pub fn healthy-diets_component() -> Html {
    let html_content = include_str!("healthy-diets.html"); 
    html! {
        <div>
            { Html::from_html_unchecked(html_content.into()) }
        </div>
    }
}
