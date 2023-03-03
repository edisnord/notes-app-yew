mod components;
mod routes;
mod store;

use components::{NotesPanel, NoteEditor};
use routes::Route;
use yew::prelude::*;
use yew::{function_component, html, Html};
use yew_router::prelude::*;

type StateContext = UseReducerHandle<store::State>;

#[function_component(App)]
fn app() -> Html {
    // Whole application state kept in a use reducer handle,
    // which will be passed to the context
    let state = use_reducer(store::State::default);

    // Handles what to render on different routes
    let route_switch = {
        let state = state.clone();
        move |routes: Route| match routes {
            Route::Home => html! {
                <NotesPanel />
            },
            Route::EditNote { id } => {
                let id: u64 = match id.parse(){
                    Ok(id) => id,
                    Err(_) => return html!{<p>{"Not a valid ID in route"}</p>}
                };

                let store::State{notes, ..} = (*state).clone();
                match notes.into_iter().find(|note| note.id == id) {
                    Some(note) => html!{<NoteEditor id={note.id} title={note.title} contents={note.contents}/>},
                    None => return html! {<p>{"Unknown ID passed to route"}</p>}
                }
            }
        }
    };

    html! {
        <div class="app">
            <ContextProvider<StateContext> context={state}>
                <BrowserRouter>
                    <Switch<Route> render={route_switch}/>
                </BrowserRouter>
            </ContextProvider<StateContext>>
        </div>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
