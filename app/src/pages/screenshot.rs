use yew::{prelude::*};
use crate::start::Route;
use yew_router::{prelude::{use_history, History}};
use gloo::{console::log, storage::{LocalStorage, Storage}};

use crate::pages::get_common_data::GetCommonData;
#[function_component(Screenshot)]
pub fn screenshot() -> Html {
    let history = use_history().unwrap();
    
    let token: String = LocalStorage::get("token").unwrap_or_default();
    let history3 = history.clone();
    if token.clone().trim() != ""{
        log!("token: ", token);
    }
    else{
        history3.push(Route::Home);
    }
    
    html! {
        <div>
            <h1>{ "Screenshot" }</h1>
            <GetCommonData   />
        </div>
    }
}
