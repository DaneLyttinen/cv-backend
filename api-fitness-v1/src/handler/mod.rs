use lambda_http::{Body, Error, Request, Response};

use self::target_handler::target_handler;
use crate::handler::assess_handler::assess_handler;
mod assess_handler;
mod target_handler;
use std::sync::Mutex;
use once_cell::sync::Lazy;

static TARGET_STRING: Lazy<Mutex<String>> = Lazy::new(|| Mutex::new(String::new()));

fn set_goal(value: String) {
    *TARGET_STRING.lock().unwrap() = value;
}

pub async fn function_handler(_event: Request) -> Result<Response<Body>, Error> {
    // Extract some useful information from the request
    let resp: Result<Response<Body>, Error>;
    if _event.uri().path() == "/target"{
        resp = target_handler(_event);
    } else if _event.uri().path() == "/assess" {
        resp = assess_handler(_event);
    } else {
        resp = Ok(Response::builder()
        .status(404)
        .header("content-type", "application/json")
        .body("uri does not exist".into())
        .map_err(Box::new)?);
    }
    return resp;
}