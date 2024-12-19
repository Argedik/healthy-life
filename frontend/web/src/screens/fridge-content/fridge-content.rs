use yew::prelude::*;

#[function_component(fridge-content)]
pub fn fridge-content_component() -> Html {
    let html_content = include_str!("fridge-content.html"); 
    html! {
        <div>
            { Html::from_html_unchecked(html_content.into()) }
        </div>
    }
}
