use yew_router::prelude::*;
use yew::prelude::*;

use crate::pages::LoginForm;
use crate::pages::Screenshot;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/screenshot")]
    Screenshot,
    #[at("/login")]
    Login,
    #[not_found]
    #[at("/404")]
    NotFound,
    
}


#[function_component(Home)]
fn home() -> Html {

    let history = use_history().unwrap();
    let onclick = Callback::once(move |_| history.push(Route::Login));
    html! {
        <div>
            <h1>{ "Home" }</h1>
            <button {onclick}>{ "Go to Login" }</button>
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
        Route::Screenshot => html! {
            <Screenshot />
        },
        Route::NotFound => html! { <NotFound /> },
        Route::Login=> html! {<LoginForm />},
    }
}

#[function_component(MyApp)]
pub fn my_app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={Switch::render(switch)} />
        </BrowserRouter>

    }
}