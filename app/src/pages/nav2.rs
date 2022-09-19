use std::{ops::Deref};


use yew::{prelude::*};

use gloo::{console::log};



#[function_component(Nav2)]
pub fn nav2()->Html{
    let  pause_state = use_state(|| "pointer-events: none;".to_owned());
    let  start_state = use_state(|| "pointer-events: auto;".to_owned());

    let f1 = pause_state.clone();
    let f2 = start_state.clone();
    let pause = Callback::from(move |_|{
    
            log!("pause screenshot");
            f1.set("pointer-events: none; filter: invert(52%) sepia(3%) saturate(17%) hue-rotate(346deg) brightness(95%) contrast(85%)".to_owned());
            f2.set("pointer-events: auto; filter: grayscale(100%) brightness(80%) sepia(300%) hue-rotate(50deg) saturate(500%);".to_owned());
        
    });

    let f1 = pause_state.clone();
    let f2 = start_state.clone();
    let play = Callback::from(move |_|{
        
            log!("play screenshot");
            f2.set("pointer-events: none; filter: invert(52%) sepia(3%) saturate(17%) hue-rotate(346deg) brightness(95%) contrast(85%)".to_owned());
            f1.set("pointer-events: auto; filter:invert(10%) sepia(96%) saturate(5607%) hue-rotate(359deg) brightness(120%) contrast(114%);".to_owned());
        
    });
    html!(
        <div class="nav2">
            <input type="text" />

            <img src="img/play.svg" alt="play image" style={start_state.deref().clone()} onclick={play}/> 
         
         
            <img src="img/pause.svg" alt="pause image" style={pause_state.deref().clone()} onclick={pause}/> 
    
           
        </div>
    )
}