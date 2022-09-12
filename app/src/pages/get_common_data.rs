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
                      <i class="btn-sm " type="button" data-bs-toggle="dropdown" aria-expanded="false">

                      <img src="img/three-dots-vertical.svg" alt="menu"/>
                      </i>
                      <ul class="dropdown-menu">

                        <li><a class="dropdown-item disabled" href="#">{"Version 0.0.1"}</a></li>
                        <li><a class="dropdown-item Settings" style="cursor: pointer; color:rgb(115, 116, 117);" href="#">{"Settings"}</a></li>
                        <li><a class="dropdown-item" id="log_out" style="cursor: pointer; color:rgb(115, 116, 117);" href="#" onclick={onclick}>{"Log Out"}</a></li>

                      </ul>
                      
                   </a>
                   <a class="split">
                        <img src="img/Settings.svg" alt="settings"/> 
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
    let  pause_state = use_state(|| "pointer-events: none;".to_owned());
    let  start_state = use_state(|| "pointer-events: auto;".to_owned());

    let f1 = pause_state.clone();
    let f2 = start_state.clone();
    let pause = Callback::from(move |_|{
        let str = f1.deref().clone();
        if str.contains("pointer-events: auto;") {
            log!("pause screenshot");
            f1.set("pointer-events: none; filter: invert(52%) sepia(3%) saturate(17%) hue-rotate(346deg) brightness(95%) contrast(85%)".to_owned());
            f2.set("pointer-events: auto; filter: grayscale(100%) brightness(80%) sepia(300%) hue-rotate(50deg) saturate(500%);".to_owned());
        }
    });

    let f1 = pause_state.clone();
    let f2 = start_state.clone();
    let play = Callback::from(move |_|{
        let str = f2.deref().clone();
        if str.contains("pointer-events: auto;") {
            log!("play screenshot");
            f2.set("pointer-events: none; filter: invert(52%) sepia(3%) saturate(17%) hue-rotate(346deg) brightness(95%) contrast(85%)".to_owned());
            f1.set("pointer-events: auto; filter:invert(10%) sepia(96%) saturate(5607%) hue-rotate(359deg) brightness(120%) contrast(114%);".to_owned());
        }
    });
    html!(
        <div class="nav2">
            <input type="text" />

            <img src="img/play.svg" alt="play image" style={start_state.deref().clone()} onclick={play}/> 
         
         
            <img src="img/pause.svg" alt="pause image" style={pause_state.deref().clone()} onclick={pause}/> 
    
           
        </div>
    )
}