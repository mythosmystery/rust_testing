use yew::{function_component, html};

#[function_component(Navbar)]
pub fn navbar() -> Html {
    html! {
        <nav class="navbar">
            <div class="navbar-title">{"Hunter Barton"}</div>
            <div class="navbar-links">
                <a class="navbar-link" href="#">{"Home"}</a>
                <a class="navbar-link" href="#about">{"About"}</a>
                <a class="navbar-link" href="#contact">{"Contact"}</a>
            </div>
        </nav>
    }
}
