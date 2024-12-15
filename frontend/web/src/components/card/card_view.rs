use yew::prelude::*;

#[function_component(Card)]
pub fn card_view() -> Html {
    let card_container = include_str!("card.html");
    html! {
        <div>
            { Html::from_html_unchecked(card_container.into()) }
        </div>
    }
}
