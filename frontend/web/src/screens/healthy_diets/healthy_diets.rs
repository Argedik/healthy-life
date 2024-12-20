use yew::prelude::*;

#[function_component(HealthyDiets)]
pub fn healthy_diets_component() -> Html {
    let html_content = include_str!("healthy_diets.html"); 
    html! {
        <div>
            { Html::from_html_unchecked(html_content.into()) }
        </div>
    }
}
