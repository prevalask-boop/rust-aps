mod solver;

use spin_sdk::http::{IntoResponse, Request, Response, Method};
use spin_sdk::http_component;
use serde::Serialize;
use solver::{solve_cp, ScheduleRequest};

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

#[http_component]
fn handle_algo_aps_cp(req: Request) -> anyhow::Result<impl IntoResponse> {
    let path = req.header("spin-path-info").map(|h| h.as_str().unwrap_or("")).unwrap_or("/");
    let method = req.method();

    match (method, path) {
        (&Method::Post, "/schedule") | (&Method::Post, "/api/v1/aps/cp/schedule") => {
            let body = req.body();
            match serde_json::from_slice::<ScheduleRequest>(body) {
                Ok(schedule_req) => {
                    let result = solve_cp(&schedule_req);
                    json_response(200, &result)
                }
                Err(e) => error_response(400, &format!("Invalid JSON body for ScheduleRequest: {}", e)),
            }
        }
        _ => error_response(404, "APS CP Endpoint not found. Use POST /api/v1/aps/cp/schedule"),
    }
}
