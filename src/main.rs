use gloo::console::log;
use yew::prelude::*;

const CARGO_PKG_NAME: &str = env!("CARGO_PKG_NAME");

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
            <main>
            <h1>{CARGO_PKG_NAME}</h1>
            <p> {"current count: "} {*state} </p>
            <button onclick={incr_counter}> {"+"} </button>
            <button onclick={decr_counter}> {"-"} </button>
            </main>
            <footer>
                {"Skapad av  "}
                <a href="https://samake.se" target="_blank">
                    {"Samuel \"sermuns\" Åkesson"}
                </a>
            </footer>
        </>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    yew::Renderer::<App>::new().render();
}
