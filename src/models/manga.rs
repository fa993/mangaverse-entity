use chrono::NaiveDateTime;
use serde::Serialize;

use super::{author::Author, chapter::MangaChapter, genre::Genre, source::MangaSource};

#[derive(Default, Debug)]
pub struct MangaTable {
    pub id: String,
    pub linked_id: String,
    pub is_listed: bool,
    pub cover_url: String,
    pub url: String,
    pub last_updated: Option<NaiveDateTime>,
    pub status: String,
    pub is_main: String,
    pub description: String,
    pub source_id: String,
    pub last_watch_time: Option<NaiveDateTime>,
    pub public_id: String,
    pub is_old: bool
}

#[derive(Serialize, Default, Debug)]
pub struct CompleteManga {
    pub main: MainManga,
    pub related: Vec<LinkedManga>,
}

#[derive(Serialize, Default, Debug)]
pub struct MainManga {
    #[serde(flatten)]
    pub manga_view: MangaView,
    pub chapters: Vec<MangaChapter>,
    pub authors: Vec<Author>,
    pub artists: Vec<Author>,
    pub genres: Vec<Genre>,
}

#[derive(Serialize, Default, Debug)]
pub struct LinkedManga {
    #[serde(flatten)]
    pub manga_view: MangaView,
    pub chapters: Vec<MangaChapter>,
}

#[derive(Serialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MangaView {
    pub id: String,
    pub linked_id: String,
    pub name: String,
    #[serde(rename = "coverURL")]
    pub cover_url: String,
    pub last_updated: Option<NaiveDateTime>,
    pub description: String,
    pub status: String,
    pub source: MangaSource,
}
