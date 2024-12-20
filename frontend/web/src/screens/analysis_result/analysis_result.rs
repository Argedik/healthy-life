use yew::prelude::*;

#[function_component(AnalysisResult)]
pub fn analysis_result_component() -> Html {
    let html_content = include_str!("analysis_result.html"); 
    html! {
        <div>
            { Html::from_html_unchecked(html_content.into()) }
        </div>
    }
}
