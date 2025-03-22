use axum::{
    extract::Json, response::IntoResponse, routing::{get, post}, Router
};
use serde::{Deserialize, Serialize};
use rand::{distr::Alphanumeric, Rng};

mod storage;


#[tokio::main]
async fn main() {
    
    let app = Router::new()
        .route("/shorten", post(shorten))
        .route("/", get(|| async {"Hello!"}));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[derive(Deserialize,Serialize)]
struct ShortenRequest{
    url: String,
}

async fn shorten(Json(payload): Json<ShortenRequest>) -> impl IntoResponse{
    Json(payload)
}

fn generate_short_id() -> String {
    rand::rng()
        .sample_iter(&Alphanumeric)
        .take(6)
        .map(char::from)
        .collect()
}
