use serde::Serialize;

#[derive(Serialize, Default, Debug, Clone, PartialEq, Eq)]
pub struct Genre {
    pub id: String,
    pub name: String,
}
