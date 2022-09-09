
use yewdux::prelude::*;
use yew::{prelude::*};
use gloo::{console::log, net::http::{Request}};
use serde::{Serialize, Deserialize};
use web_sys::{HtmlInputElement};
use stylist::{style, yew::styled_component}; // using style for adding css
use crate::start::Route;
use yew_router::{prelude::{use_history, History}};


#[derive( Default, Clone, PartialEq, Eq, Store, Serialize, Deserialize)]
struct UserData{
    email:String,
    password:String,
}

#[styled_component(LoginForm)]
pub fn login_form() -> Html {
    
    let history = use_history().unwrap();
    let (_state, dispatch) = use_store::<UserData>();
    
    
    let email =  dispatch.reduce_mut_callback_with(|state, event:Event| {
        let email = event.target_unchecked_into::<HtmlInputElement>().value();
        state.email = email;
        
    });

    

    let password =  dispatch.reduce_mut_callback_with(|state, event:Event| {
        let password = event.target_unchecked_into::<HtmlInputElement>().value();
        state.password = password;
    });

    let history = history.clone(); 
    let submit = dispatch.reduce_mut_callback_with(move |state, _| {
            
        let state = state.clone();
        
        let history = history.clone();
        wasm_bindgen_futures::spawn_local(async move {
        
            let request= Request::post("http://127.0.0.1:3000/login")
            .json(&state)
            .unwrap()
            .send()
            .await.unwrap();
            
            
    
        let status_code=request.status();
        if status_code>=200 && status_code <=400 
        {
            if request.headers().has("token") {
                log!("User is admin");
                gloo::dialogs::alert("Admin can't use screenshot monitor");
            }
            else{
                log!("user is an employee");
                
                history.push(Route::Screenshot);
            }
        }
        else{
            log!(request.text().await.unwrap());
        }
        });

    });
    
    let add_style= style!(
        r#"
          .topnav{
            display:flex;
          }
          .login{
            width:calc(70%)
            
          }
          .signup{
            
            margin: 1em;
            
          }


          * {
            box-sizing: border-box;
          }
          input[type=email], input[type=password] {
            width: 100%;
            display: inline-block;
            border: 1px solid #ccc;
            box-sizing: border-box;
          }

          .forgetpassword{
            color:blue; 
            cursor:pointer; 
            font-size: 11px; 
            float:right;
          }
          button{
            margin: 5px 5px 5px 5px;
          }
        "#
    ).unwrap();
     

    html! {
        <div class ={add_style}>
            <h1>{ "Login" }</h1>
            <div class="topnav">
              <div class="login">
              <label for="email" >{ "Email" }</label>
              <input type="email" id="email"  onchange={email}/>
              <br />
              <label for="password">{ "Password" }</label>
              <a class="forgetpassword" href="https://screenshotmonitor.com/auth/recover?email=">{ "Forget password" }</a>
              <input type="password" id="password" onchange={password} />
              <br />
              <button onclick={submit}>{ "Login" }</button>
              </div>
              <div class="signup">
                <p>{ "Don't have account" }</p>
                <a href="https://screenshotmonitor.com/signup">{ "Sign Up" }</a>
              </div>
            </div>
        </div>
    }
}