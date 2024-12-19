use yew::prelude::*;

#[function_component(special-meal)]
pub fn special-meal_component() -> Html {
    let html_content = include_str!("special-meal.html"); 
    html! {
        <div>
            { Html::from_html_unchecked(html_content.into()) }
        </div>
    }
}
