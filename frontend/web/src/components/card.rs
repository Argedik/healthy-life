use yew::prelude::*;

#[function_component(Card)]
pub fn card() -> Html {
    // let style = style!(
    //     r#"
    //     body {
    //         background-color: lightblue;
    //         font-family: Arial, sans-serif;
    //     }
    //     .card {
    //         padding: 20px;
    //         border: 1px solid #ccc;
    //         border-radius: 10px;
    //         background-color: white;
    //         max-width: 300px;
    //         margin: auto;
    //     }
    //     "#
    // )
    // .expect("Failed to create style");
    // <div class={style}>

    html! {
        <div>
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
