use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <>
            <h4>{ "My first yew program" }</h4>
            <h1>{ "WELCOME TO YEW TUTORIAL" }</h1>
            
            <p>{ "Yew is a modern Rust framework for creating multi-threaded front-end web apps using WebAssembly." }</p>
            <a href="https://yew.rs/docs/tutorial">{ "Yew tutorial link" }</a>
            <h3>{ "Prerequisites" }</h3>
            <ol>
               <li>
                   <a href="https://trunkrs.dev/">{ "trunk" }</a>
               </li>
               <li>
                   <a href="https://www.rust-lang.org/"> {"rust"} </a>
               </li>
               <li>{ "wasm32-unknown-unknown"}</li>
            </ol>
            <p>
               { "Command for running this code: trunk serve" }
            </p>
            <p>
               { "Before running my code you must install rust, trunk, webAssembly for rust. links are already provided" }
            </p>
        </>
    }
}

fn main() {
    yew::start_app::<App>();
}
