use serde::Serialize;

#[derive(Serialize, Default, Debug)]
pub struct SourcePattern {
    pub url: String,
    pub source_id: String,
}
