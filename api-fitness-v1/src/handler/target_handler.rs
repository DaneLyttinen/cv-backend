use lambda_http::{
    aws_lambda_events::serde_json, Body, Error, Request, Response,
};

use crate::requests::TargetRequest;

use super::{set_goal};

pub fn target_handler(_event: Request) -> Result<Response<Body>, Error> {
    let body = _event.body();
    let s = std::str::from_utf8(body).expect("invalid utf-8 sequence");
    let target_request = match serde_json::from_str::<TargetRequest>(s) {
        Ok(target_req) => target_req,
        Err(err) => {
            let resp = Response::builder()
                .status(400)
                .header("content-type", "application/json")
                .body(err.to_string().into())
                .map_err(Box::new)?;
            return Ok(resp);
        }
    };
    set_goal(target_request.target);
    //TARGET_STRING.replace_range(0..TARGET_STRING.len(), &target_request.target);
    let resp = Response::builder()
        .status(200)
        .header("content-type", "application/json")
        .body("Target request received".into())
        .map_err(Box::new)?;
    Ok(resp)
}
