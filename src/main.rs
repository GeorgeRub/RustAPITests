use axum::http::{HeaderMap, StatusCode};
use axum::response::{IntoResponse, Response};
use axum::routing::get;
use axum::Router;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};
use std::env;
use tokio::net::TcpListener;

#[derive(Serialize, FromRow)]
struct Item {
    id: i32,
    name: String,
    description: String,
}

#[derive(Deserialize)]
struct RequestItem {
    name: String,
    description: String,
}

#[derive(Clone)]
struct AppStage {
    db_pool: PgPool,
}

impl AppStage {}

async fn root(headers: HeaderMap) -> Response {
    (StatusCode::FORBIDDEN, "Access denied").into_response()
}

// #[derive(Deserialize, Debug)]
// struct Headers {
//     test: String,
//     host: String,
//     accept: String,
// }


async fn api_test(headers: HeaderMap) -> Response {
    // let headers_json = serde_json::from_str(&headers.try_into().unwrap()).unwrap();
    // println!("{:#?}", headers.get("test"));

    let resp_text: String = format!("from api test :) ==>> test header is {:#?}", headers.get("test").unwrap());

    (StatusCode::OK, resp_text).into_response()
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = PgPool::connect(&database_url)
        .await
        .expect("Database connection failed.");

    let app_state = AppStage { db_pool: pool };
    let app = Router::new()
        .route("/", get(root))
        .route("/api", get(api_test))
        .with_state(app_state);

    let listener = TcpListener::bind("0.0.0.0:8000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
