use serde::Deserialize;

#[derive(Deserialize)]
pub struct RequestItem {
    pub(crate) name: String,
    pub(crate) description: String,
}
