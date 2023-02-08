mod repo;
mod service;

use axum::{extract::State, routing::post, Json, Router};
use http::Method;
use repo::word_repo::WordRepo;
use serde::{Deserialize, Serialize};
use service::sentence_service::SentenceService;
use sqlx::postgres::PgPoolOptions;
use std::{net::SocketAddr, sync::Arc};
use tower_http::cors::{Any, CorsLayer};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let pool = Arc::new(
        PgPoolOptions::new()
            .max_connections(5)
            .connect("postgres://postgres:postgres@postgres/wodemao")
            .await
            .unwrap(),
    );

    let word_repo = Arc::new(WordRepo::new(pool));
    let sentence_service = Arc::new(SentenceService::new(word_repo));

    let cors = CorsLayer::new()
        .allow_methods(vec![Method::GET, Method::POST])
        .allow_origin(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/tokenize", post(tokenize))
        .with_state(sentence_service)
        .layer(cors);

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::info!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn tokenize(
    State(sentence_service): State<Arc<SentenceService>>,
    Json(payload): Json<ParseRequest>,
) -> Json<ParseResponse> {
    let fine = vec![sentence_service
        .cut(&payload.text, true)
        .iter()
        .map(|s| s.to_string())
        .collect()];
    Json(ParseResponse { fine })
}

#[derive(Deserialize)]
struct ParseRequest {
    text: String,
}

#[derive(Serialize)]
struct ParseResponse {
    fine: Vec<Vec<String>>,
}
