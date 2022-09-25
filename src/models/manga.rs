use chrono::NaiveDateTime;
use serde::Serialize;

use super::{author::Author, chapter::MangaChapter, genre::Genre, source::MangaSource};

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
