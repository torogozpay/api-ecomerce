// shared/src/response_models.rs

use domain::models::*;
use serde::Serialize;

#[derive(Serialize)]
pub enum ResponseBody {
    Message(String),
    Business(Business),
    Businesses(Vec<Business>),
    Invoice(Invoice),
    Invoices(Vec<Invoice>),
    MyInvoice(MyInvoice),
    MyInvoices(Vec<MyInvoice>)
}

#[derive(Serialize)]
pub struct Response {
    pub body: ResponseBody,
}