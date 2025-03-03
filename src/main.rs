mod models;
mod rest_requests;

use crate::models::AppStage;
use crate::rest_requests::{
    api_test, create_item, delete_all_items, delete_item, get_item, get_items, root, update_item,
};
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::Router;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use std::env;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = PgPool::connect(&database_url)
        .await
        .expect("Database connection failed.");

    // let app_state = AppStage { db_pool: pool };
    let app = Router::new()
        .nest(
            "/api/v1",
            Router::new()
                .route("/", get(root))
                .route("/test", get(api_test))
                .route(
                    "/items",
                    post(create_item).get(get_items).delete(delete_all_items),
                )
                .route(
                    "/items/{id}",
                    get(get_item).put(update_item).delete(delete_item),
                ),
        )
        .with_state(AppStage { db_pool: pool });

    let listener = TcpListener::bind("0.0.0.0:8000").await.unwrap();
    println!("Listening on port 8000");
    axum::serve(listener, app).await.unwrap();
}
