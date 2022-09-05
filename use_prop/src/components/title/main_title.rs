use yew::prelude::*;
#[derive(Properties, PartialEq)]
pub struct Prop{
    pub title:String,
    pub color:Color,
}
#[derive(PartialEq)]
pub enum Color{
    Info,
    Warn
}
impl Color{
    pub fn to_string(&self)->String{
        match self{
            Color::Info=> "info".to_owned(),
            Color::Warn=> "warn".to_owned()
        }
    }
}

#[function_component(MainTitle)]
pub fn main_title(props: &Prop)->Html{
    html!(
        <h1 class={props.color.to_string()}>{&props.title}</h1>
    )
    
}