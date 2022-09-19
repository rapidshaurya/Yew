
use serde_json::Value;
use yew::{prelude::*};



use crate::start::Route;
use yew_router::{prelude::{use_history, History}};
use gloo::{storage::{LocalStorage, Storage}, console::log, utils::document};

fn update_drop_down(){
  let company =match LocalStorage::get("company"){
    Ok(res)=> res,
    Err(_err)=>Value::Null
  };
  if company !=Value::Null{
    log!("Not null");
    let a =document().get_element_by_id("DropDown").unwrap();
    let temp = company.get("company").unwrap();
    let temp = temp.as_array().unwrap();
    let str=temp.into_iter().map(|company| {
     format!("<option value={}>{}</option>",company.get("id").unwrap().to_string(), company.get("name").unwrap().to_string())
    }).collect::<String>();
    a.set_inner_html(&str);
  }
}


#[function_component(TopNav)]
pub fn top_nav()->Html {
    
    let token: Value = LocalStorage::get("token").unwrap_or_default();    
    let name = token.get("name").unwrap().to_string();
    log!(name.clone());

    wasm_bindgen_futures::spawn_local(async move {

      gloo::timers::future::TimeoutFuture::new(1000).await;
   
      update_drop_down();
      
    });
    
    
  
       
        let company =match LocalStorage::get("company"){
            Ok(res)=> res,
            Err(_err)=>Value::Null
        };
    
        log!(company.to_string());
        let history = use_history().unwrap();
        let history2 = history.clone();
        let onclick = Callback::once(move |_| 
                                                                    {
                                                                       LocalStorage::delete("token");
                                                                       history2.push(Route::Home);
                                                                    });
        if company!=Value::Null {

            let temp = company.get("company").unwrap();
            let temp = temp.as_array().unwrap();
        
                
                html!(
                    <div >
                       <div class="topnav">
                       <a>
                        {name}
                       </a>
                       <a>
                        <select id="DropDown">
                          {temp.into_iter().map(|company| {
                            html!{<option value={company.get("id").unwrap().to_string()}>{ {company.get("name").unwrap()} }</option>}
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
                    </div>
                    
                )

        }else{
          html!(
            <div >
               <div class="topnav">
               <a>
                {name}
               </a>
               <a>
                <select id="DropDown">
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
            </div>
            
        )
        }
        
        

}