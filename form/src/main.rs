
use yew::prelude::*;
use gloo::console::log;

use web_sys::{HtmlInputElement};
#[function_component(Form)]
pub fn form()-> Html{
    let email = Callback::from(|event: Event|{
        let input = event.target_unchecked_into::<HtmlInputElement>().value();
        log!(input)
    });
    
    let password = Callback::from(|event: Event|{
        let input = event.target_unchecked_into::<HtmlInputElement>().value();
        log!(input)
    });
    html!(
        <form>
            
        <label for="email">{ "Email" }</label>
        <input type="email" id="email" onchange={email}/>
        <br />
        <label for="password">{ "Password" }</label>
        <input type="password" id="password" onchange={password}/>
        <br />
        <button>{ "submit" }</button>
        </form>
    )
}
fn main() {
    yew::start_app::<Form>();
}
