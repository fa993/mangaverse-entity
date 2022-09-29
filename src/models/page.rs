use serde::Serialize;

#[derive(Default, Debug)]
pub struct PageTable {
    pub id: i32,
    pub url: String,
    pub page_number: i32,
    pub chapter_id: String,
}

#[derive(Serialize, Default, Debug)]
pub struct PageURL {
    pub url: String,
}

#[derive(Serialize, Default, Debug)]
pub struct ChapterPosition {
    pub index: i64,
    pub length: i64,
}
