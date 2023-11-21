// api/src/invoice_handler.rs

use actix_web::{delete, get, post, put, web, HttpRequest, HttpResponse};

use shared::error_handler::CustomError;
use application::{verify_business, invoice::{create, update, read, delete}};
use domain::models::{Invoice, MyInvoice, MyNewInvoice};
use domain::modelsext::{InvoiceExt, ListInvoicesExt, InvoiceCreated, InvoiceResult};

//use crate::utils::check;
use crate::utils::response;

#[utoipa::path(
    get,
    path = "/getInvoice",
    responses(
        (status = 200, description = "Get all invoices", body = inline(response::InvoicesResponse)),
        (status = 400, description = "Error", body = inline(response::ErrorResponse))
    )
)]
#[get("/getInvoice")]
pub async fn list_invoices_handler(req: HttpRequest) -> Result<HttpResponse, CustomError> {
    match verify_business(req.headers()).await {
        Ok(_config) => {    
            let invoices= read::list_invoices(&_config).await?;
            Ok(HttpResponse::Ok().json(invoices))
        },
        Err(_) => Err(CustomError::new(401, "Not authorizated".to_string()))
    }    
}

#[utoipa::path(
    get,
    path = "/getInvoice/{payment_hash}",
    responses(
        (status = 200, description = "Get a invoice identifies with payment_hash", body = inline(Invoice)),
        (status = 400, description = "Error", body = inline(response::ErrorResponse)),
        (status = 404, description = "Invoice was not found", body = inline(response::ErrorResponse))
    )
)]
#[get("/getInvoice/{payment_hash}")]
pub async fn list_invoice_handler(payment_hash : web::Path<String>, req: HttpRequest) -> Result<HttpResponse, CustomError> {
    match verify_business(req.headers()).await {
        Ok(_config) => {      
            let invoices = read::list_invoice_by_hash(&_config, payment_hash.clone()).await?;
            Ok(HttpResponse::Ok().json(invoices))
        },
        Err(_) => Err(CustomError::new(401, "Not authorizated".to_string()))
    }    
}

#[utoipa::path(
    post,
    path = "/startPayment",
    responses(
        (status = 200, description = "Generate invoice in node", body = inline(response::MyInvoiceResponse)),
        (status = 400, description = "Error", body = inline(response::ErrorResponse))
    )
)]
#[post("/startPayment")]
pub async fn start_payment_handler(invoice: web::Json<InvoiceCreated>, req: HttpRequest) -> Result<HttpResponse, CustomError> {  
    match verify_business(req.headers()).await  {
        Ok(_config) => {    
            let invoices = create::start_payment(_config, invoice.into_inner()).await?;
            Ok(HttpResponse::Ok().json(invoices))
        },
        Err(_) => Err(CustomError::new(401, "Not authorizated".to_string()))
    }    
}

#[utoipa::path(
    post,
    path = "/sendPayment",
    responses(
        (status = 200, description = "Create a new invoice", body = inline(response::MyInvoiceResponse)),
        (status = 400, description = "Error", body = inline(response::ErrorResponse))
    )
)]
#[post("/sendPayment")]
pub async fn create_invoice_handler(invoice: web::Json<MyNewInvoice>, req: HttpRequest) -> Result<HttpResponse, CustomError> {  
    match verify_business(req.headers()).await  {
        Ok(_config) => {    
            let invoices = create::send_payment(_config, invoice.into_inner()).await?;
            Ok(HttpResponse::Ok().json(invoices))
        },
        Err(_) => Err(CustomError::new(401, "Not authorizated".to_string()))
    }    
}

#[utoipa::path(
    put,
    path = "/updInvoice",
    responses(
    (status = 200, description = "Modify a new invoice", body = inline(response::MyInvoiceResponse)),
    (status = 400, description = "Error", body = inline(response::ErrorResponse)),
    (status = 404, description = "Invoice was not found", body = inline(response::ErrorResponse))
    )
)]
#[put("/updInvoice")]
pub async fn update_invoice_handler(invoice: web::Json<MyInvoice>, req: HttpRequest) -> Result<HttpResponse, CustomError> {
    match verify_business(req.headers()).await {
        Ok(_config) => {      
            let invoices = update::update_invoice( invoice.into_inner()).await?;
            Ok(HttpResponse::Ok().json(invoices))
        },
        Err(_) => Err(CustomError::new(401, "Not authorizated".to_string()))
    }    
}

#[utoipa::path(
    delete,
    path = "/delInvoice/{model_id}",
    responses(
        (status = 200, description = "Delete a new invoice", body = inline(response::InvoicesResponse)),
        (status = 400, description = "Error", body = inline(response::ErrorResponse)),
        (status = 404, description = "Invoice was not found", body = inline(response::ErrorResponse))
    )
)]
#[delete("/delInvoice/{model_id}")]
pub async fn delete_invoice_handler(model_id: web::Path<i32>, req: HttpRequest) -> Result<HttpResponse, CustomError> {
    match verify_business(req.headers()).await {
        Ok(_config) => {       
            let invoices = delete::delete_invoice(*model_id)?;
            Ok(HttpResponse::Ok().json(invoices))
        },
        Err(_) => Err(CustomError::new(401, "Not authorizated".to_string()))
    }           
}
