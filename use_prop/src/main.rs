/*
In this example, we will see how to pass property
*/
use yew::prelude::*;

mod components;
pub use components::title::main_title::{MainTitle, Color};

#[function_component(App)]
pub fn app()->Html{
    html!(
        <div>
        <MainTitle title="hi there!!!" color={Color::Info}/>
        </div>
    )
}
fn main() {
    yew::start_app::<App>();
}
