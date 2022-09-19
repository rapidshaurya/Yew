use yew::{prelude::*};
use crate::start::Route;
use yew_router::{prelude::{use_history, History}};
use gloo::{ storage::{LocalStorage, Storage}};
use stylist::{Style};
use serde_json::Value;
use crate::pages::{topnav::TopNav, nav2::Nav2};
use crate::request::get_common_data;
const EXT_CSS:&str = include_str!("interface.css");

#[function_component(Screenshot)]
pub fn screenshot() -> Html {
  
    let history = use_history().unwrap();
    let token = match LocalStorage::get("token"){
        Ok(res)=> res,
        Err(_err)=>Value::Null
    };
    if token == Value::Null{
        history.push(Route::Home);
        html!{}
    }
    else{
        let add_style = Style::new(EXT_CSS).unwrap();
        
            get_common_data();
    
        html! {
            <div class={add_style} >
                <h1>{ "Screenshot" }</h1>
                <TopNav />
                <Nav2 />
            </div>
        }
    }
        
        
    
  
}
