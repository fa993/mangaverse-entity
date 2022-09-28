use chrono::NaiveDateTime;
use serde::Serialize;

use super::page::PageTable;

#[derive(Debug)]
pub struct ChapterTable<'a> {
    pub chapter_id: String,
    pub chapter_name: String,
    pub chapter_number: String,
    pub updated_at: Option<NaiveDateTime>,
    pub manga_id: String,
    pub last_watch_time: i64,
    pub sequence_number: i32,

    pub pages: &'a [PageTable],
}

#[derive(Serialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MangaChapter {
    pub id: String,
    pub chapter_name: String,
    pub chapter_number: String,
    pub sequence_number: i32,
    pub updated_at: Option<NaiveDateTime>,
}
