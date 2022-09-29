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
    pub is_main: bool,
    pub description: String,
    pub last_watch_time: Option<NaiveDateTime>,
    pub public_id: String,
    pub is_old: bool,

    pub source: &'a SourceTable,
    pub chapter: Vec<ChapterTable>,
    pub authors: Vec<String>,
    pub artists: Vec<String>,
    pub genres: &'a [Genre],
    pub titles: Vec<String>
}

impl MangaTable<'_> {
    
    pub fn new<'a>(st: &'a SourceTable, ge: &'a [Genre]) -> MangaTable<'a> {
        MangaTable { 
            id: String::default(),
            linked_id: String::default(), 
            is_listed: bool::default(), 
            name: String::default(), 
            cover_url: String::default(), 
            url: String::default(), 
            last_updated: None, 
            status: String::default(), 
            is_main: bool::default(), 
            description: String::default(), 
            last_watch_time: None,
            public_id: String::default(), 
            is_old: bool::default(), 
            source: st, 
            chapter: Vec::default(), 
            authors: Vec::default(), 
            artists: Vec::default(), 
            genres: ge, 
            titles: Vec::default()
        }
    }

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
