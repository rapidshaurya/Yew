use yew::prelude::*;
use gloo::console::log;
use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize)]
struct User{
    username: String,
    age: u8
}
#[function_component(App)]
fn app() -> Html {
    log!("User Details"); // use log!() for console.log()
    log!(serde_json::to_string_pretty(   // for printing in console.log() object we use have to convert it
        &User{
            username: String::from("Rammu"),
            age:23
        }
    ).unwrap());
    html!{
        <a>{"how are you?"}</a>
    }
    
}
fn main(){
 yew::start_app::<App>();
}