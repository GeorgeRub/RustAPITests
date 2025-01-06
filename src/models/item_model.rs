use serde::Serialize;
use sqlx::FromRow;

#[derive(Serialize, FromRow)]
pub struct Item {
    pub(crate) id: i32,
    pub(crate) name: String,
    pub(crate) description: String,
}
