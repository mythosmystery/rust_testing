use yew::prelude::*;

enum Msg {
    Toggle,
}

struct Model {
    dark_mode: bool,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self { dark_mode: false }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Toggle => {
                self.dark_mode = !self.dark_mode;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        // This gives us a component's "`Scope`" which allows us to send messages, etc to the component.
        let link = ctx.link();
        html! {
            <div class={if self.dark_mode {"dark"} else {"light"}}>
                <button onclick={link.callback(|_| Msg::Toggle)}>{"Toggle"}</button>
            </div>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}
