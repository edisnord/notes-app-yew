use std::{rc::Rc};

use chrono::{DateTime, Utc};
use yew::Reducible;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Note {
    pub id: u64,
    pub title: String,
    pub last_modified: DateTime<Utc>,
    pub contents: String,
}

impl Note {
    pub fn new(id: u64, title: &str, contents: &str) -> Note {
        Note {
            id,
            title: title.to_owned(),
            last_modified: Utc::now(),
            contents: contents.to_owned(),
        }
    }
}

#[derive(Clone,Debug,PartialEq,Eq)]
pub struct State {
    note_nr: u64,
    pub notes: Vec<Note>,
}

impl Reducible for State {
    type Action = Action;

    fn reduce(self: std::rc::Rc<Self>, action: Self::Action) -> std::rc::Rc<Self> {
        Rc::new(match action {
            Action::AddNote { title, contents } => {
                let last_id = self.notes.iter().last().unwrap().id;
                let mut new_notes = self.notes.clone();
                State {
                    notes: {new_notes.push(Note::new(last_id + 1, &title, &contents)); new_notes},
                    ..*self
                }
            },
            Action::RmNote { number } => {
                State {
                    notes: self.notes.clone().into_iter().filter(|note| note.id != number).collect(),
                    ..*self
                }
            },
            Action::EditNote {
                id,
                title,
                contents,
            } => State {
                    notes: self.notes.clone().into_iter().map(
                        |note|  if note.id == id { Note::new(id, &title, &contents) } else {note} 
                    ).collect(),
                    ..*self
                }
            ,
        })
    }
}

impl Default for State{
    fn default() -> Self {
        State {
            note_nr: 0,
            notes: (1..30).into_iter().map(|id| Note::new(id, &format!("note {}", id), "Some Note")).collect()
        }
    }
}

pub enum Action {
    AddNote {
        title: String,
        contents: String,
    },
    RmNote {
        number: u64,
    },
    EditNote {
        id: u64,
        title: String,
        contents: String,
    },
}

