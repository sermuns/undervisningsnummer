use yew::prelude::*;

const CARGO_PKG_NAME: &str = env!("CARGO_PKG_NAME");
const CARGO_PKG_DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");

#[function_component]
fn App() -> Html {
    let state = use_state(|| 0);

    let incr_counter = {
        let state = state.clone();
        Callback::from(move |_| state.set(*state + 1))
    };

    let decr_counter = {
        let state = state.clone();
        Callback::from(move |_| state.set(*state - 1))
    };

    html! {
        <>
            <p> {"current count: "} {*state} </p>
            <button onclick={incr_counter}> {"+"} </button>
            <button onclick={decr_counter}> {"-"} </button>
        </>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    yew::Renderer::<App>::new().render();
}

