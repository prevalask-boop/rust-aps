use spin_sdk::http::{IntoResponse, Request, Response, Method};
use spin_sdk::http_component;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct ReverseRequest {
    input: String,
}

#[derive(Serialize)]
struct ReverseResponse {
    result: String,
}

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
}

fn json_response<T: Serialize>(status: u16, data: &T) -> anyhow::Result<Response> {
    let body = serde_json::to_string(data)?;
    Ok(Response::builder()
        .status(status)
        .header("content-type", "application/json")
        .body(body)
        .build())
}

fn error_response(status: u16, msg: &str) -> anyhow::Result<Response> {
    json_response(status, &ErrorResponse { error: msg.to_string() })
}

/// A simple Spin HTTP component that exposes algorithm services
#[http_component]
fn handle_algo_string(req: Request) -> anyhow::Result<impl IntoResponse> {
    let path = req.header("spin-path-info").map(|h| h.as_str().unwrap_or("")).unwrap_or("/");
    let method = req.method();

    match (method, path) {
        (&Method::Post, "/reverse") | (&Method::Post, "/api/v1/string/reverse") => {
            let body = req.body();
            match serde_json::from_slice::<ReverseRequest>(body) {
                Ok(rev_req) => {
                    let reversed: String = rev_req.input.chars().rev().collect();
                    json_response(200, &ReverseResponse { result: reversed })
                }
                Err(_) => error_response(400, "Invalid JSON body for ReverseRequest"),
            }
        }
        _ => error_response(404, "String Endpoint not found"),
    }
}
