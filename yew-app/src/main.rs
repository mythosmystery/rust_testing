mod components;
use components::navbar::Navbar;
use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <div>
            <Navbar />
        </div>
    }
}

fn main() {
    yew::start_app::<App>();
}
