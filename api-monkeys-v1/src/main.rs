mod requests;
use crate::requests::TryRequest;
mod genetic_algorithm;
use crate::genetic_algorithm::genetic_algorithm;
use lambda_http::{run, service_fn, Body, Error, Request, RequestExt, Response};
use serde::{Deserializer, Serialize};
use serde_json::{json, Value};
use tracing::info;

#[derive(Debug, Serialize)]
struct SuccessResponse {
    pub body: String,
}

#[derive(Debug, Serialize)]
struct FailureResponse {
    pub body: String,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    println!("Received request!");
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_target(false)
        .with_ansi(false)
        .without_time()
        .init();
    run(service_fn(handle)).await;
    Ok(())
}

async fn handle(_event: Request) -> Result<Response<Body>, Error> {
    let body = _event.body();
    let s = std::str::from_utf8(body).expect("invalid utf-8 sequence");
    let mut try_request = match serde_json::from_str::<TryRequest>(s) {
        Ok(try_req) => try_req,
        Err(err) => {
            let resp = Response::builder()
                .status(400)
                .header("content-type", "application/json")
                .body(err.to_string().into())
                .map_err(Box::new)?;
            return Ok(resp);
        }
    };
    genetic_algorithm(&mut try_request).await;
    let j = serde_json::to_string("Try Request received")?;
    let resp = Response::builder()
        .status(200)
        .header("content-type", "application/json")
        .body(j.into())
        .map_err(Box::new)?;
    Ok(resp)
}
