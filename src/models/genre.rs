use serde::Serialize;

#[derive(Serialize, Default, Debug, Clone)]
pub struct Genre {
    pub id: String,
    pub name: String,
}
