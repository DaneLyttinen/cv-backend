use std::cmp;

use lambda_http::{aws_lambda_events::serde_json, Body, Error, Request, Response};

use crate::requests::{AssessRequest, AssessResponse};

use super::{TARGET_STRING};

pub fn assess_handler(_event: Request) -> Result<Response<Body>, Error> {
    let body = _event.body();
    let s = std::str::from_utf8(body).expect("invalid utf-8 sequence");
    let assess_request = match serde_json::from_str::<AssessRequest>(s) {
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
    let scores = assess(&assess_request);
    let response = AssessResponse {
        id: assess_request.id,
        scores: scores,
    };
    let json = serde_json::to_string(&response)?;
    let resp = Response::builder()
        .status(200)
        .header("content-type", "application/json")
        .body(json.into())
        .map_err(Box::new)?;
    Ok(resp)
}

pub fn assess(assess_request: &AssessRequest) -> Vec<u32> {
    let target_vec: Vec<char> = TARGET_STRING.lock().unwrap().chars().collect();
    let mut scores: Vec<u32> = Vec::new();
    for genome in &assess_request.genomes {
        let length = cmp::min(target_vec.len(), genome.len());
        let genome_vec: Vec<char> = genome.chars().collect();
        let mut score: u32 = (0..length)
            .map(|index| (genome_vec[index] != target_vec[index]) as u32)
            .sum();
        score = score + cmp::max(target_vec.len(), genome.len()) as u32 - length as u32;
        scores.push(score);
    }
    return scores;
}
