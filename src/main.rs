mod repo;
mod service;

use axum::{
    extract::State,
    response::{IntoResponse, Response},
    routing::post,
    Json, Router,
};
use http::{Method, StatusCode};
use repo::word_repo::WordRepo;
use serde::{Deserialize, Serialize};
use serde_json::json;
use service::sentence_service::{SentenceService, SentenceServiceError};
use sqlx::postgres::PgPoolOptions;
use std::{net::SocketAddr, sync::Arc};
use tower_http::cors::{Any, CorsLayer};

use crate::service::sentence_service::WordInfo;

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
        .route("/process", post(process))
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
    let fine = sentence_service.cut(&payload.text, false);

    Json(ParseResponse { fine })
}

async fn process(
    State(sentence_service): State<Arc<SentenceService>>,
    Json(payload): Json<ParseRequest>,
) -> Result<Json<ProcessResponse>, AppError> {
    let word_info_list = sentence_service.process_sentence(&payload.text).await?;

    Ok(Json(ProcessResponse { word_info_list }))
}

#[derive(Deserialize)]
struct ParseRequest {
    text: String,
}

#[derive(Serialize)]
struct ParseResponse {
    fine: Vec<Vec<String>>,
}

#[derive(Serialize)]
struct ProcessResponse {
    word_info_list: Vec<Vec<WordInfo>>,
}

enum AppError {
    SentenceService(SentenceServiceError),
}

impl From<SentenceServiceError> for AppError {
    fn from(err: SentenceServiceError) -> Self {
        AppError::SentenceService(err)
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, msg) = match self {
            AppError::SentenceService(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()),
        };

        let body = Json(json!({
            "error": msg,
        }));

        (status, body).into_response()
    }
}
