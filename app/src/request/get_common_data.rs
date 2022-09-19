
use gloo::{net::http::{Request}, storage::{LocalStorage, Storage}, dialogs::alert, console::log};
use serde::{Serialize, Deserialize};
use serde_json::{Value};

use crate::pages::Token;



/// this is for custom Response
#[derive(Deserialize,Serialize)]
 struct Response{
    pub body:String,
    pub success:bool
}





pub fn get_common_data() {
    let token: Value = LocalStorage::get("token").unwrap();
    let token:Token = serde_json::from_str(&token.to_string()).unwrap();
   log!(token.jwt.clone());
    if token.jwt.clone()!="" {
        
        wasm_bindgen_futures::spawn_local(async move {
        
           let custom_response:Response= match Request::get("http://127.0.0.1:3000/get-data")
            .header("token", &token.jwt)
            .send()
            .await{
                Ok(request)=> {

                    let status_code=request.status();
                    if status_code>=200 && status_code <=400 
                     {
                        let save_data: Value = request.json().await.unwrap();
                        log!(LocalStorage::set("company", save_data).is_ok());
                      
                        Response{
                            body:"Successfull".to_owned(),
                            success:true
                        }
            
                    }
                    else  {
                        Response{
                        body:request.text().await.unwrap_or_default(),
                        success:false
                       }
                    }

                },
                Err(_e) => {
                    Response{
                        body:"Servor Error".to_owned(),
                        success:false
                       }
                }
            };
            if custom_response.success == false{
                alert(&custom_response.body);
            }
    
        
        });
    }
    

}
