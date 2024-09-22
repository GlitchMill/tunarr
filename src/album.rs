use serde::Serialize;

#[derive(Serialize)]
pub struct Album {
    pub name: String,
    pub cover_art: Option<String>, // Make sure this field is here
}
