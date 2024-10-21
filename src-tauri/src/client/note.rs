use crate::domain::Note;

pub trait CreateNote {
    fn create_note(&self, note: &Note) -> Result<Note, String>;
}

struct NoteCreateService;

impl CreateNote for NoteCreateService {
    fn create_note(&self, note: &Note) -> Result<Note, String> {
        todo!()
    }
}