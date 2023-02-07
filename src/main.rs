mod repo;

use axum::{extract::State, routing::post, Json, Router};
use http::Method;
use jieba_rs::Jieba;
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgPoolOptions;
use std::{net::SocketAddr, sync::Arc};
use tower_http::cors::{Any, CorsLayer};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://postgres:postgres@postgres/wodemao")
        .await
        .unwrap();

    let jieba = Arc::new(Jieba::new());

    let cors = CorsLayer::new()
        .allow_methods(vec![Method::GET, Method::POST])
        .allow_origin(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/tokenize", post(tokenize))
        .with_state(jieba)
        .layer(cors);

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::info!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn tokenize(
    State(jieba): State<Arc<Jieba>>,
    Json(payload): Json<ParseRequest>,
) -> Json<ParseResponse> {
    let fine = vec![jieba
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
