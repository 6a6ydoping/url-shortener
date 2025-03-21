use axum::{
    extract::Json,
    routing::{get, patch, post},
    Router,
};
use serde::Deserialize;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/shorten", post(|| async {"Hello world!"}))
        .route("/", get(|| async {"Hello!"}));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[derive(Deserialize)]
struct ShortenRequest{
    url: String,
}

async fn shorten(Json(payload): Json<ShortenRequest>){
    print!("wip")
}
