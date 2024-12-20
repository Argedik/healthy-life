use yew::prelude::*;

#[function_component(DailyMenu)]
pub fn daily_menu_component() -> Html {
    let html_content = include_str!("daily_menu.html"); 
    html! {
        <div>
            { Html::from_html_unchecked(html_content.into()) }
        </div>
    }
}
