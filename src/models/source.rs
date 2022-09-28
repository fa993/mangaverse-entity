use serde::Serialize;

#[derive(Debug)]
pub struct SourceTable {
    pub id: String,
    pub name: String,
    pub priority: i32,
}

#[derive(Serialize, Default, Debug)]
pub struct MangaSource {
    pub id: String,
    pub name: String,
}
