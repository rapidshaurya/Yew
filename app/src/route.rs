use yew_router::prelude::*;
use yew::prelude::*;


#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/secure")]
    Secure,
    #[not_found]
    #[at("/404")]
    NotFound,
}


#[function_component(Secure)]
fn secure() -> Html {
    let history = use_history().unwrap();
    
    let onclick = Callback::once(move |_| history.push(Route::Home));
    html! {
        <div>
            <h1>{ "Secure" }</h1>
            <button {onclick}>{ "Go Home" }</button>
        </div>
    }
}

#[function_component(Home)]
fn home() -> Html {
    
    html! {
        <div>
            <h1>{ "Home" }</h1>
        </div>
    }
}

#[function_component(NotFound)]
fn not_found() -> Html {
    
    html! {
        <div>
            <h1>{ "404" }</h1>
        </div>
    }
}



pub fn switch(routes: &Route) -> Html {
    match routes {
        Route::Home => html! { <Home /> },
        Route::Secure => html! {
            <Secure />
        },
        Route::NotFound => html! { <NotFound /> },
    }
}