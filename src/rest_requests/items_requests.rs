use axum::extract::{Path, State};
use axum::http::{HeaderMap, StatusCode};
use axum::Json;
use axum::response::IntoResponse;
use crate::models::{AppStage, DeleteItemResponse, Item, RequestItem};
use axum::response::Json as AxumJson;

pub async fn create_item(
    State(state): State<AppStage>,
    Json(payload): Json<RequestItem>,
) -> (StatusCode, AxumJson<Item>) {
    let item = state
        .create_item(&payload.name, &payload.description)
        .await
        .unwrap();
    (StatusCode::CREATED, AxumJson(item))
}

pub async fn root() -> impl IntoResponse {
    (StatusCode::OK, "Access denied").into_response()
}

pub async fn api_test(headers: HeaderMap) -> impl IntoResponse {
    let resp_text: String = format!(
        "from api test :) ==>> test header is {:#?}",
        headers.get("test").unwrap()
    );
    (StatusCode::OK, resp_text).into_response()
}

pub async fn get_items(State(state): State<AppStage>) -> impl IntoResponse {
    AxumJson(state.get_items().await.unwrap())
}

pub async fn get_item(State(state): State<AppStage>, Path(id): Path<i32>) -> impl IntoResponse {
    match state.get_item(id).await.unwrap() {
        Some(item) => (StatusCode::OK, AxumJson(item)).into_response(),
        None => StatusCode::NOT_FOUND.into_response(),
    }
}

pub async fn update_item(
    State(state): State<AppStage>,
    Path(id): Path<i32>,
    Json(payload): Json<RequestItem>,
) -> impl IntoResponse {
    match state
        .update_item(id, &payload.name, &payload.description)
        .await
    {
        Ok(Some(item)) => (StatusCode::OK, AxumJson(item)).into_response(),
        Ok(None) => StatusCode::NOT_FOUND.into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

pub async fn delete_item(State(state): State<AppStage>, Path(id): Path<i32>) -> impl IntoResponse {
    match state.delete_item(id).await {
        Ok(true) => StatusCode::NO_CONTENT.into_response(),
        Ok(false) => StatusCode::NOT_FOUND.into_response(),
        _ => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

pub async fn delete_all_items(State(state): State<AppStage>) -> impl IntoResponse {
    match state.delete_all_items().await {
        Ok(deleted_items) => (
            StatusCode::OK,
            AxumJson(DeleteItemResponse { deleted_items }),
        )
            .into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}