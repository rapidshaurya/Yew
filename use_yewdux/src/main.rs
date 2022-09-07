//yewdux is a simple state management for yew applications.
use yew::prelude::*;
use yewdux::prelude::*;

#[derive(Default, Clone, PartialEq, Eq, Store)]
struct State {
    count: u32,
}

#[function_component(Check)]
fn check() -> Html {
    let (state, dispatch) = use_store::<State>();
    let onclick = dispatch.reduce_mut_callback(|state| state.count += 1);
    html! {
        <div>
        <p>{ "counter value in other function: " }{ state.count }</p>
        <button {onclick}>{"+1"}</button>
        </div>
    }
}



#[function_component(App)]
fn app() -> Html {
    let (state, dispatch) = use_store::<State>();
    let onclick = dispatch.reduce_mut_callback(|state| state.count += 1);

    html! {
        <>
        <p>{ "counter value in main function: " }{ state.count }</p>
        <button {onclick}>{"+1"}</button>
        <Check />
        </>
    }
}

fn main() {
    yew::start_app::<App>();
}