use axum::{routing::post, Json, Router};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    let client = reqwest::Client::new();

    // build our application with a route
    let app = Router::new().route("/tokenize", post(parse));

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn parse(
    // this argument tells axum to parse the request body
    // as JSON into a `CreateUser` type
    Json(payload): Json<ParseRequest>,
) -> Json<ParseResponse> {
    
    client
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

#[derive(Serialize)]
struct HanLPParseRequest {
    language: String,
    text: String,
}

#[derive(Deserialize)]
struct HanLPParseResponse {
    #[serde(rename = "tok/fine")]
    fine: Vec<Vec<String>>,
}
