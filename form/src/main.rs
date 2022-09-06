
use std::ops::Deref;
use yew::prelude::*;
use gloo::{console::log, net::http::Request};
use web_sys::{HtmlInputElement};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
pub struct UserData{
    pub email:String,
    pub password:String,
}


#[function_component(Form)]
pub fn form()-> Html{
    let user_state = use_state(|| UserData::from(UserData{
        email:String::from("null"),
        password:String::from("null"),
    }));

    let clone_user_state = user_state.clone();
    let email = Callback::from(move |event: Event|{
        
        let input = event.target_unchecked_into::<HtmlInputElement>().value();
        let mut data = clone_user_state.deref().clone();
        data.email=input;
        clone_user_state.set(data);
    });

    let clone_user_state = user_state.clone();
    let password = Callback::from(move |event: Event|{
        
        let input = event.target_unchecked_into::<HtmlInputElement>().value();
        let mut data = clone_user_state.deref().clone();
        data.password=input;
        clone_user_state.set(data);
    });

    let clone_user_state = user_state.clone();
    let submit=Callback::from( move |_|{
        let data = clone_user_state.clone();
        wasm_bindgen_futures::spawn_local(async move {
            let data = data.deref().clone();
        
            let request= Request::post("http://127.0.0.1:3000/login")
            .json(&data)
            .unwrap()
            .send()
            .await
            .unwrap();
        let status_code=request.status();
        if status_code>=200 && status_code <=400 
        {
            if request.headers().has("token") {
                log!("User is admin")
            }
            else{
                log!("user is an employee")
            }
        }
        else{
            log!(request.text().await.unwrap());
        }
        });
    });

    html!(
        <div>
            
        <label for="email">{ "Email" }</label>
        <input type="email" id="email" onchange={email}/>
        <br />
        <label for="password">{ "Password" }</label>
        <input type="password" id="password" onchange={password} />
        <br />
        <button onclick={submit}>{ "submit" }</button>
        </div>
    )
}
fn main() {
    yew::start_app::<Form>();
}
