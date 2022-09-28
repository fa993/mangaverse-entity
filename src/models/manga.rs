use chrono::NaiveDateTime;
use serde::Serialize;

use super::{
    author::Author,
    chapter::{ChapterTable, MangaChapter},
    genre::Genre,
    source::{MangaSource, SourceTable},
};

#[derive(Debug)]
pub struct MangaTable<'a> {
    pub id: String,
    pub linked_id: String,
    pub is_listed: bool,
    pub name: String,
    pub cover_url: String,
    pub url: String,
    pub last_updated: Option<NaiveDateTime>,
    pub status: String,
    pub is_main: String,
    pub description: String,
    pub last_watch_time: Option<NaiveDateTime>,
    pub public_id: String,
    pub is_old: bool,

    pub source: &'a SourceTable,
    pub chapter: &'a [ChapterTable<'a>],
    pub authors: &'a [Author],
    pub artists: &'a [Author],
    pub genres: &'a [Genre],
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
