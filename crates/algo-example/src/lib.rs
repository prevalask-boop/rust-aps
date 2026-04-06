use spin_sdk::http::{IntoResponse, Request, Response, Method};
use spin_sdk::http_component;
use serde::{Deserialize, Serialize};
use sha2::{Sha256, Digest};
use std::str;

// Define requests and responses for the algorithms
#[derive(Deserialize)]
struct FibRequest {
    n: u32,
}

#[derive(Serialize)]
struct FibResponse {
    result: u32,
}

#[derive(Deserialize)]
struct HashRequest {
    input: String,
}

#[derive(Serialize)]
struct HashResponse {
    result: String,
}

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
}

/// Helper function to create a JSON response
fn json_response<T: Serialize>(status: u16, data: &T) -> anyhow::Result<Response> {
    let body = serde_json::to_string(data)?;
    Ok(Response::builder()
        .status(status)
        .header("content-type", "application/json")
        .body(body)
        .build())
}

/// Helper function to create an error response
fn error_response(status: u16, msg: &str) -> anyhow::Result<Response> {
    json_response(status, &ErrorResponse { error: msg.to_string() })
}

/// The Fibonacci algorithm
fn fibonacci(n: u32) -> u32 {
    if n <= 1 {
        return n;
    }
    let mut a = 0;
    let mut b = 1;
    for _ in 2..=n {
        let temp = a + b;
        a = b;
        b = temp;
    }
    b
}

/// The Hash algorithm
fn sha256_hash(input: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(input.as_bytes());
    let result = hasher.finalize();
    format!("{:x}", result)
}

/// A simple Spin HTTP component that exposes algorithm services
#[http_component]
fn handle_algo_example(req: Request) -> anyhow::Result<impl IntoResponse> {
    // Router
    let path = req.header("spin-path-info").map(|h| h.as_str().unwrap_or("")).unwrap_or("/");
    let method = req.method();

    match (method, path) {
        (&Method::Post, "/api/v1/math/fibonacci") => {
            let body = req.body();
            match serde_json::from_slice::<FibRequest>(body) {
                Ok(fib_req) => {
                    let res = fibonacci(fib_req.n);
                    json_response(200, &FibResponse { result: res })
                }
                Err(_) => error_response(400, "Invalid JSON body for FibRequest"),
            }
        }
        (&Method::Post, "/api/v1/math/hash") => {
            let body = req.body();
            match serde_json::from_slice::<HashRequest>(body) {
                Ok(hash_req) => {
                    let res = sha256_hash(&hash_req.input);
                    json_response(200, &HashResponse { result: res })
                }
                Err(_) => error_response(400, "Invalid JSON body for HashRequest"),
            }
        }
        (&Method::Get, "/") | (&Method::Get, "/api/v1") => {
            Ok(Response::builder()
                .status(200)
                .header("content-type", "application/json")
                .body(r#"{"message": "Welcome to the Rust Algorithm Monorepo! Available endpoints: POST /api/v1/math/fibonacci, POST /api/v1/math/hash"}"#)
                .build())
        }
        _ => error_response(404, "Endpoint not found"),
    }
}