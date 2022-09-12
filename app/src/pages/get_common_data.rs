use std::{ops::Deref};

use stylist::{Style};
use yew::{prelude::*};
use yewdux::prelude::*;
use crate::start::Route;
use yew_router::{prelude::{use_history, History}};
use gloo::{console::log, net::http::{Request}, storage::{LocalStorage, Storage}};
use serde::{Serialize, Deserialize};
use serde_json::{Value};
use crate::pages::login::Token;

const EXT_CSS:&str = include_str!("interface.css");

#[derive( Default, Clone, PartialEq, Eq, Store, Serialize, Deserialize)]
pub struct Company{
    pub company: Value,
}



#[function_component(GetCommonData)]
pub fn get_common_data() -> Html {
    let token: String = LocalStorage::get("token").unwrap_or_default();
    let company = use_state(||Company::default());
    let mut name = String::new();
    if token.trim()!="" {
        let token:Token = serde_json::from_str(&token).unwrap();
        let company2 =company.clone();
        name = token.name.clone();
        wasm_bindgen_futures::spawn_local(async move {
        
            let request= Request::get("http://127.0.0.1:3000/get-data")
            .header("token", &token.jwt)
            .send()
            .await.unwrap();
    
        let status_code=request.status();
        if status_code>=200 && status_code <=400 
        {
            
            let data:Value = request.json().await.unwrap();
            let mut a = company2.deref().clone();
            a.company=data;
            company2.set(a);
            
        }
        else{
            log!(request.text().await.unwrap_or_default());
        }
        });
    }
    
    

    
   

        let company_state = company.deref().clone().company;
        let temp = company_state.get("company");
        let add_style = Style::new(EXT_CSS).unwrap();

        if temp !=None {
            let history = use_history().unwrap();
            let temp = temp.unwrap().as_array().unwrap();
            let history2 = history.clone();
                let onclick = Callback::once(move |_| 
                                                                {
                                                                   LocalStorage::delete("token");
                                                                   history2.push(Route::Home);
                                                                });
            html!(
                <div class={add_style}>
                   <div class="topnav">
                   <a>
                    {name}
                   </a>
                   <a>
                    <select>
                      {temp.into_iter().map(|company| {
                        html!{<option value={company.get("id").unwrap().to_string()}>{ company.get("name").unwrap() }</option>}
                      }).collect::<Html>()}
                    </select>
                   </a>
                   <a class="split">
                      <button {onclick}>{ "Log Out" }</button>
                   </a>
                   <a class="split">
                        <img src="img/Settings.svg" alt="rust image"/> 
                   </a>
                   
                   </div>
                   <Nav2 />
                </div>
                
            )
        }
        else{
            html!()
        }   
}

#[function_component(Nav2)]
pub fn nav2()->Html{

    let pause = Callback::from(|_|{
        log!("stop screenshot");
    });
    let play = Callback::from(|_|{
        log!("play screenshot");
    });
    html!(
        <div class="nav2">
            <input type="text" />

            <img src="img/play.svg" alt="play image" onclick={play}/> 
         
         
            <img src="img/pause.svg" alt="pause image" onclick={pause}/> 
    
           
        </div>
    )
}