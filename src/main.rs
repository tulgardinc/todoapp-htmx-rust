use axum::{
    response::{Html, IntoResponse},
    routing::{get, post},
    Router,
};
use maud::html;
use sqlx::sqlite::{SqliteConnectOptions, SqliteConnection};
use tower_http::services::ServeDir;

async fn root() -> Html<&'static str> {
    Html(include_str!("../index.html"))
}

#[tokio::main]
async fn main() {
    let pool = sqlx::SqlitePool::connect("sqlite://../todos.db")
        .await
        .unwrap();

    let app = Router::new()
        .route("/", get(root))
        .nest_service("/public", ServeDir::new("./public"))
        .with_state(pool);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
