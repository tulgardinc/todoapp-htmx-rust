use std::fs;

use axum::{
    http::HeaderMap,
    response::{Html, IntoResponse},
    routing::{get, post},
    Router,
};
use maud::html;
use tower_http::services::ServeDir;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

async fn root() -> Html<&'static str> {
    Html(include_str!("../index.html"))
}

async fn clicked() -> impl IntoResponse {
    html! {
        div {
            "I'm mogging"
        }
    }
    .into_response()
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(root))
        .route("/clicked", post(clicked))
        .nest_service("/public", ServeDir::new("./public"));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
