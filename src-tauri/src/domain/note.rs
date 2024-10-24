use serde::Deserialize;

#[derive(Deserialize)]
pub struct Note {
    key: String,
    value: String,
}
