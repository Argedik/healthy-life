use yew::prelude::*;

#[function_component(SpecialMenu)]
pub fn special_meal_component() -> Html {
    let html_content = include_str!("special_meal.html"); 
    html! {
        <div>
            { Html::from_html_unchecked(html_content.into()) }
        </div>
    }
}
