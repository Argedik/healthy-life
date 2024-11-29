use dioxus:::prelude::*;

fn main() {
  dioxus_desktop::launch(app);
}

fn app(cx: Scope) -> Element {
  cx.render(rsx!(
    div {
      h1 { "Mobil Sağlık Uygulaması" }
    }
  ))
}