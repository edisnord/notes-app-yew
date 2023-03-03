use crate::routes;
use crate::store;
use crate::StateContext;
use wasm_bindgen::JsCast;
use log::info;
use web_sys::HtmlInputElement;
use web_sys::HtmlTextAreaElement;
use web_sys::{EventTarget};
use yew::prelude::*;
use yew::{function_component, html, Callback, Html, Properties};
use yew_router::prelude::use_navigator;

#[derive(Properties, PartialEq)]
pub struct NoteEditorProps {
    pub id: u64,
    pub title: String,
    pub contents: String,
}

#[function_component]
pub fn NoteEditor(props: &NoteEditorProps) -> Html {
    let ctx = use_context::<StateContext>().expect("Use context error");

    let contents = use_state(|| props.contents.clone());
    let title = use_state(|| props.title.clone());

    let on_contents_changed = {
        let contents = contents.clone();
        Callback::from(move |e: KeyboardEvent| {
            let target: Option<EventTarget> = e.target();
            let input = target.and_then(|t| t.dyn_into::<HtmlTextAreaElement>().ok());
            if let Some(input) = input {
                contents.set(input.value())
            }
        })
    };

    let on_title_changed = {
        let title = title.clone();
        Callback::from(move |e: KeyboardEvent| {
            info!("{}", *title.clone());
            let target: Option<EventTarget> = e.target();
            let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());
            if let Some(input) = input {
                info!("{}", input.value());
                title.set(input.value());
            }
        })
    };

    let on_save_clicked = {
        let ctx = ctx.clone();
        let title = title.clone();
        let contents = contents.clone();
        let id = props.id.clone();
        let navigator = use_navigator().unwrap();

        Callback::from(move |_| {
            ctx.dispatch(store::Action::EditNote { id: id, title: (*title).clone(), contents: (*contents).clone() });
            navigator.push(&routes::Route::Home);
        })
    };

    html! {
        <div class="editor_panel" >
            <div class={"flex flex-col w-full divide-y divide-dashed divide-blue-400 h-full"}>
                <input type={"text"} value={{(*title).clone()}} onkeyup={on_title_changed} class="text-3xl outline-none resize-none text-center editor w-full"/>
                <textarea value={(*contents).clone()} onkeyup={on_contents_changed} class="text-md h-full bg-transparent resize-none text-center outline-none overflow-auto max-h-full editor w-full"></textarea>
            </div>
            <button onclick={on_save_clicked} class={"align-bottom py-2 px-4 h-auto w-auto button_with_hover text-sm rounded-3xl"}>{"Save"}</button>
        </div>
    }
}
