use yew::prelude::*;

#[function_component(Card)]
pub fn card() -> Html {
    html! {
        <div class="card">
            <div class="image-container">
                <p>{ "Yemek resmi" }</p>
            </div>
            <div class="info-container">
                <p>{ "Resim Açıklaması" }</p>
                <p>{ "Puanı, Maliyeti" }</p>
            </div>
        </div>
    }
}
