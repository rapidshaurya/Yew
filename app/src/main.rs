use yew_router::prelude::*;
use yew::prelude::*;

mod route;
use route::*;




#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={Switch::render(switch)} />
        </BrowserRouter>

    }
}
fn main() {
    yew::start_app::<App>();
}