use serde::Serialize;

#[derive(Serialize, Default, Debug)]
pub struct Genre {
    pub id: String,
    pub name: String,
}
