use yew::{function_component, html, Html};
use yew::prelude::*;
use yew_router::prelude::use_navigator;
use crate::StateContext;
use crate::routes::Route;
use crate::store;
use super::note_line::NoteLine;

#[function_component]
pub fn NotesPanel() -> Html {
    let ctx = use_context::<StateContext>().expect("Use context error");
    let store::State {mut notes, ..} = (*ctx).clone();
    notes.sort_by(|note1, note2| note2.last_modified.cmp(&note1.last_modified));
    let editing = use_state(|| false);

    {
        let ctx = ctx.clone();
        let editing = editing.clone();
        let navigator = use_navigator().unwrap();

        use_effect(move || {
            if *editing {
                let new_note_id = (*ctx).notes.iter().last().unwrap().id;
                navigator.push(&Route::EditNote { id: new_note_id.to_string() });
                editing.set(false);
            }
        });
    }

    let on_create_click = {
        let ctx = ctx.clone();
        let editing = editing.clone();
        Callback::from(move |_| {
            ctx.dispatch(store::Action::AddNote { title: "New note".to_owned(), contents: String::new()});
            editing.set(true);
        })
    };

    html!{
        <div class={"notes_panel"}>
            <p class={"header"}>{"Notes: "}</p>
            <div class={"notes"}>
                {notes.into_iter().map(|note| html!{
                    <NoteLine id={note.id} title={note.title} last_edited={note.last_modified} />
                }).collect::<Html>()}
            </div>
            <button onclick={on_create_click} class={"button_with_hover"}>{"Create Note"}</button>
        </div>
    }
}