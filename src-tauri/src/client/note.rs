use reqwest::Client;
use crate::domain::Note;
use std::error::Error;

pub trait CreateNote {
    fn create_note(&self, note: &Note) -> Result<Note, NoteCreateFailure>;
}

pub struct NoteCreateFailure {}

struct NoteCreateService;
