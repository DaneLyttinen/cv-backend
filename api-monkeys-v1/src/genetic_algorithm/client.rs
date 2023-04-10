use reqwest::{header::CONTENT_TYPE, Client};
use std::error::Error;

use crate::{requests::{AssessRequest, AssessResponse, TopRequest}};
pub struct RequestClient{
    pub client: Client
}

impl RequestClient{
    pub async fn send_assess_req(&self, assess_request: AssessRequest) -> Result<AssessResponse, Box<dyn Error>>{
        let resp: AssessResponse = self.client.post("https://httpbin.org/ip")
        .header(CONTENT_TYPE, "application/json")
        .json(&assess_request)
        .send()
        .await?
        .json::<AssessResponse>()
        .await?;
        println!("{}" ,resp.to_string());
        return Ok(resp)
    }

    pub async fn send_top_req(&self, top_request: TopRequest) {
        self.client.post("https://blazorapiservice.azurewebsites.net/api/notifications")
            .header(CONTENT_TYPE, "application/json")
            .json(&top_request)
            .send()
            .await;
    }

    pub fn new() -> RequestClient{
        return RequestClient{client:reqwest::Client::new()}
    }
}