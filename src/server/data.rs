use serde::Serialize;

#[derive(Serialize)]
pub(crate) struct ServerItem {
    pub id: String,
    pub name: String,
    pub slug: String,
    pub language: String,
    pub version_code: u32,
    pub version_name: String,
}
