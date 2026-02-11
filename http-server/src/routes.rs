use crate::model::{CalculationRequest, CalculationResult, MyApiResponse, Response};
use axum::{
    Json, debug_handler,
    extract::Query,
    http::{StatusCode, header},
    response::IntoResponse,
};
use reqwest::Client;
use std::time::{SystemTime, UNIX_EPOCH};

#[debug_handler]
pub async fn root() -> Json<Response> {
    Json(Response {
        message: "Hello, World!".to_string(),
    })
}

fn calculate(payload: CalculationRequest) -> Result<Json<CalculationResult>, StatusCode> {
    let result = match payload.operation.as_str() {
        "add" => payload.a + payload.b,
        "subtract" => payload.a - payload.b,
        "multiply" => payload.a * payload.b,
        "divide" => {
            if payload.b == 0.0 {
                return Err(StatusCode::BAD_REQUEST);
            } else {
                payload.a / payload.b
            }
        }
        _ => return Err(StatusCode::BAD_REQUEST),
    };

    Ok(Json(CalculationResult { result }))
}

#[debug_handler]
pub async fn calculate_get(
    Query(payload): Query<CalculationRequest>,
) -> Result<Json<CalculationResult>, StatusCode> {
    calculate(payload)
}

#[debug_handler]
pub async fn calculate_post(
    Json(payload): Json<CalculationRequest>,
) -> Result<Json<CalculationResult>, StatusCode> {
    calculate(payload)
}

fn generate_random_bytes() -> (u8, u8, u8) {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .subsec_nanos();

    (nanos as u8, (nanos >> 8) as u8, (nanos >> 16) as u8)
}

#[debug_handler]
pub async fn random_png() -> impl IntoResponse {
    let (r, g, b) = generate_random_bytes();
    let bytes: Vec<u8> = vec![
        71, 73, 70, 56, 55, 97, 1, 0, 1, 0, 128, 1, 0, 0, 0, 0, r, g, b, 44, 0, 0, 0, 0, 1, 0, 1,
        0, 0, 2, 2, 76, 1, 0, 59,
    ];

    ([(header::CONTENT_TYPE, "image/png")], bytes)
}

#[debug_handler]
pub async fn redirect() -> impl IntoResponse {
    (
        StatusCode::MOVED_PERMANENTLY,
        [(header::LOCATION, "https://rust-lang.org/learn/")],
    )
}

#[debug_handler]
pub async fn get_my_ip() -> Json<MyApiResponse> {
    let reqwest = Client::new();
    let response = reqwest
        .get("https://httpbin.org/ip")
        .send()
        .await
        .unwrap()
        .json::<MyApiResponse>()
        .await
        .unwrap();

    Json(response)
}
