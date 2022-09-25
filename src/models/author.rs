use serde::Serialize;

#[derive(Serialize, Default, Debug)]
pub struct Author {
    pub id: String,
    pub name: String,
}
