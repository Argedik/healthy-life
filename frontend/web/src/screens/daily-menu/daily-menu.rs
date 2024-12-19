use yew::prelude::*;

#[function_component(daily-menu)]
pub fn daily-menu_component() -> Html {
    let html_content = include_str!("daily-menu.html"); 
    html! {
        <div>
            { Html::from_html_unchecked(html_content.into()) }
        </div>
    }
}
