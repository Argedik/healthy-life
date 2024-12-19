use yew::prelude::*;

#[function_component(analysis-result)]
pub fn analysis-result_component() -> Html {
    let html_content = include_str!("analysis-result.html"); 
    html! {
        <div>
            { Html::from_html_unchecked(html_content.into()) }
        </div>
    }
}
