use yew::prelude::*;
use stylist::{style, yew::styled_component}; // using style for adding css
#[styled_component(App)] // use this macro for using css!()
fn app() -> Html {
    let names= vec!["Ankit", "Faizal", "Rahul", "aditya"];
    let class_name= "devNames";
    let add_style= style!(
        r#"
          h1{
            color: red;
          }
          li{
            color: yellow;
          }
        "#
    ).unwrap();
    html! {
        <>
        <div class={add_style}>
           <h1>{"Dev names"}</h1>
           <ul class={class_name}>
            {names.iter().map(|name| html!{<li>{name}</li>}).collect::<Html>()}
           </ul>
        </div>
        <p class={css!("color:white;")}>{"Comapny name: xyz"}</p>
        </>
    }
}

fn main() {
    yew::start_app::<App>();
}
