use chrono::{DateTime, Utc};
use yew::{function_component, Html, Properties, Callback, html, use_state, use_context};
use yew_router::prelude::use_navigator;

use crate::{StateContext, store, routes};

#[derive(Properties, PartialEq)]
pub struct NoteLineProps {
    pub id: u64,
    pub title: String,
    pub last_edited: DateTime<Utc>,
}

#[function_component]
pub fn NoteLine(props: &NoteLineProps) -> Html{
    let ctx = use_context::<StateContext>().expect("Use context error");
    let hovered = use_state(|| false);

    let on_mouse_enter = {
        let hovered = hovered.clone();
        Callback::from(move |_|{hovered.set(true)})
    };

    let on_mouse_leave = {
        let hovered = hovered.clone();
        Callback::from(move |_|{hovered.set(false)})
    };

    let on_delete_click = {
        let ctx = ctx.clone();
        let id = props.id.clone();
        Callback::from(move |_|{ctx.dispatch(store::Action::RmNote { number: id });})
    };

    let on_edit_click = {
        let navigator = use_navigator().unwrap();
        let id = props.id.clone();
        Callback::from(move |_| {navigator.push(&routes::Route::EditNote { id: format!("{}", id)});})
    };
    
    html!{
        <div class={"note_line"} onmouseenter={on_mouse_enter} onmouseleave={on_mouse_leave}>
        if !*hovered {
            <div class={"name"}>
                <h1>{"Title: "}{&props.title}</h1>
                <h1>{"Last Modified: "} {props.last_edited.format("%d/%m/%Y %H:%M")}</h1>
            </div>
        } else {
            <div class={"edit"}>
                <h1 class="my-4 mx-5 h-full">{&props.title}</h1>
                <button onclick={on_edit_click}>{"Edit"}</button>
                <button onclick={on_delete_click}>{"Delete"}</button>
            </div>
        }
        </div>
    }
}