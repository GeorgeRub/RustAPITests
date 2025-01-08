use serde::Serialize;

#[derive(Serialize)]
pub struct DeleteItemResponse {
    pub(crate) deleted_items: u64,
}