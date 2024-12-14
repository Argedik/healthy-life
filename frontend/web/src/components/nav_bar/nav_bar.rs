use yew::prelude::*;

#[function_component(NavBar)]
pub fn nav_bar_component() -> Html {
  html! {
    <nav>
      <ul>
        <li>
          <a href="/"> {"Ana Sayfa"} </a>
        </li>
        <li>
          <a href="/stats"> {"İstatistikler"} </a>
        </li>
      </ul>
    </nav>
  }
}