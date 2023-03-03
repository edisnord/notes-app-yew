use crate::store;
use crate::StateContext;
use yew::prelude::*;
use yew::{function_component, html, Callback, Html, Properties};

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

    html!{
        <div class="editor_panel" >
            <div class={"divide-y divide-dashed divide-blue-400 h-full"}>
                <h1  contenteditable={"true"} class="text-3xl text-center editor w-full">{&*title}</h1>
                <div contenteditable={"true"} class="text-md text-center overflow-auto max-h-full editor w-full">{&*contents}</div>
            </div>
            <button class={"align-bottom w-auto border-2 border-solid border-blue-400 text-sm rounded-3xl"}>{"Save"}</button>
        </div>
    }  
}
