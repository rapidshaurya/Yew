use yew::{prelude::*};
use crate::start::Route;
use yew_router::{prelude::{use_history, History}};

#[function_component(Screenshot)]
pub fn screenshot() -> Html {
    let history = use_history().unwrap();
    
    let onclick = Callback::once(move |_| {history.push(Route::Home)});
    html! {
        <div>
            <h1>{ "Screenshot" }</h1>
            <button {onclick}>{ "Go Home" }</button>
        </div>
    }
}
