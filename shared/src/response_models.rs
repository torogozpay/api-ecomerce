// shared/src/response_models.rs

use domain::models::*;
use serde::Serialize;

#[derive(Serialize)]
pub enum ResponseBody {
    Message(String),
    Business(Business),
    Businesses(Vec<Business>),
}

#[derive(Serialize)]
pub struct Response {
    pub body: ResponseBody,
}