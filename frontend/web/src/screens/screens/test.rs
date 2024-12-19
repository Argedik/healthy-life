use yew::prelude::*;

#[function_component(test)]
pub fn test_component() -> Html {
    let html_content = include_str!("test.html"); 
    html! {
        <div>
            { Html::from_html_unchecked(html_content.into()) }
        </div>
    }
}
