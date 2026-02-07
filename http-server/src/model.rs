use serde::{Deserialize, Serialize};

// Define the response structure for the root endpoint

#[derive(Serialize)]
pub struct Response {
    pub message: String,
}

// Define the response structure for the calculation endpoint

#[derive(Deserialize)]
pub struct CalculationRequest {
    pub a: f64,
    pub b: f64,
    pub operation: String,
}

#[derive(Serialize)]
pub struct CalculationResult {
    pub result: f64,
}
