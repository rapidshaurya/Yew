use yew::prelude::*;

pub enum Button{
    AddOne,
    SubOne,
    Zero
}

pub struct Counter{
    pub count:i8,
}


impl Component for Counter{
    type Message = Button;
    type Properties = ();
    fn create(_ctx: &Context<Self>) -> Self {
        Self { count: 0 }
    }
    fn view(&self, ctx: &Context<Self>) -> Html {
        html!(
            <div>
                <button onclick={ctx.link().callback(|_|{ Button::AddOne })}> {"Add One"}</button>
                <button onclick={ctx.link().callback(|_|{ Button::SubOne })}> {"Sub One"}</button>
                <button onclick={ctx.link().callback(|_|{ Button::Zero })}> {"Reset One"}</button>
                <p>{"counter value: "} {self.count}</p>
            </div>
        )
    }
    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match  msg {
            Button::AddOne=> {
                self.count+=1;
                true
            },
            Button::SubOne=>{
                self.count-=1;
                true
            },
            Button::Zero=>{
                self.count=0;
                true
            }
        }
    }
}

#[function_component(App)]
pub fn app()-> Html{
    html!(
        <Counter />
    )
}

fn main() {
    yew::start_app::<App>();
}
