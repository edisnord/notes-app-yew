use yew::{function_component, html, Html, Properties, Callback};
use yew::prelude::*;
use crate::StateContext;
use crate::store;
use crate::components::note_line::NoteLine;

#[derive(Properties)]
pub struct NotesPanelProps{
    pub edit_click_callback: Callback<u64>,
    pub delete_click_callback: Callback<u64>
}

impl PartialEq for NotesPanelProps {
    fn eq(&self, other: &Self) -> bool {
        self.edit_click_callback == other.edit_click_callback && self.delete_click_callback == other.delete_click_callback
    }
}

#[function_component]
pub fn NotesPanel(props: &NotesPanelProps) -> Html {
    let ctx = use_context::<StateContext>().expect("Use context error");
    let store::State {notes, ..} = (*ctx).clone();

    html!{
        <div class={"notes_panel"}>
            <p class={"header"}>{"Notes: "}</p>
            <div class={"notes"}>
                {notes.into_iter().map(|note| html!{
                    <NoteLine id={note.id} title={note.title} last_edited={note.last_updated} edit_click_callback={props.edit_click_callback.clone()} />
                }).collect::<Html>()}
            </div>
        </div>
    }
}